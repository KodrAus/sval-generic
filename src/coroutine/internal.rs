use std::{
    marker::{PhantomData, PhantomPinned},
    pin::Pin,
};

use crate::{Receiver, Result};

pub trait Coroutine<'sval, R: Receiver<'sval>> {
    type State;

    const MAY_YIELD: bool = true;

    fn resume<'resume>(cx: Context<'resume, R, Self>) -> Result<Resume<'resume, Self>>;

    #[doc(hidden)]
    fn into_raw() -> RawCoroutine {
        RawCoroutine::new::<R, Self>()
    }

    #[doc(hidden)]
    unsafe fn resume_raw(cx: RawContext) -> Result<RawResume> {
        Self::resume(Context::from_raw_unchecked(cx)).map(|r| r.resume)
    }
}

pub struct Driver<'sval, 'driver, R: Receiver<'sval>, C: Coroutine<'sval, R> + ?Sized> {
    resume: Option<(RawCoroutine, RawSlot)>,
    receiver: R,
    _marker: PhantomData<&'driver mut Slot<C::State>>,
}

impl<'sval, 'driver, R: Receiver<'sval>, C: Coroutine<'sval, R> + ?Sized>
    Driver<'sval, 'driver, R, C>
{
    #[inline]
    pub fn new(receiver: R, mut slot: Pin<&'driver mut Slot<C::State>>) -> Self {
        let begin = RawSlot::new::<R, C>(slot.as_mut());

        Driver {
            resume: Some((C::into_raw(), begin)),
            receiver,
            _marker: PhantomData,
        }
    }

    #[inline]
    pub fn resume(&mut self) -> Result<bool> {
        match self.resume.take() {
            Some((co, state)) => match unsafe {
                co.resume_raw(RawContext::new(RawReceiver::new(&mut self.receiver), state))?
            } {
                RawResume(RawResumeInner::Yield(resume, state)) => {
                    self.resume = Some((resume, state));
                    Ok(true)
                }
                RawResume(RawResumeInner::Return(resume)) => {
                    self.resume = resume;
                    Ok(self.resume.is_some())
                }
            },
            None => Ok(false),
        }
    }

    pub fn into_iter(self) -> IntoIter<'sval, 'driver, R, C> {
        IntoIter(self)
    }
}

pub struct IntoIter<'sval, 'driver, R: Receiver<'sval>, C: Coroutine<'sval, R> + ?Sized>(
    Driver<'sval, 'driver, R, C>,
);

impl<'sval, 'driver, R: Receiver<'sval>, C: Coroutine<'sval, R> + ?Sized> Iterator
    for IntoIter<'sval, 'driver, R, C>
{
    type Item = Result;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0.resume() {
            Ok(true) => Some(Ok(())),
            Ok(false) => None,
            Err(e) => Some(Err(e)),
        }
    }
}

pub struct Resume<'resume, C: ?Sized> {
    resume: RawResume,
    _marker: PhantomData<&'resume mut C>,
}

pub struct Context<'resume, R: ?Sized, C: ?Sized> {
    raw: RawContext,
    _marker: PhantomData<fn(&'resume mut C, &'resume mut R)>,
}

pub struct Slot<S> {
    state: S,
    continuation: Option<(RawCoroutine, RawSlot)>,
    _pin: PhantomPinned,
}

impl<'sval, 'resume, R: Receiver<'sval>, C: Coroutine<'sval, R> + ?Sized> Context<'resume, R, C> {
    #[inline]
    unsafe fn from_raw_unchecked(raw: RawContext) -> Self {
        Context {
            raw,
            _marker: PhantomData,
        }
    }

    #[inline]
    fn slot(&mut self) -> Pin<&mut Slot<C::State>> {
        unsafe { self.raw.slot.get_unchecked_mut::<R, C>() }
    }

    #[inline]
    pub fn state(&mut self) -> (&mut R, Pin<&mut C::State>) {
        unsafe {
            (
                self.raw.receiver.get_unchecked_mut::<R>(),
                self.raw.slot.get_unchecked_mut::<R, C>().state(),
            )
        }
    }

    #[inline]
    pub fn receiver(&mut self) -> &mut R {
        unsafe { self.raw.receiver.get_unchecked_mut::<R>() }
    }

    #[inline]
    pub fn yield_self(self) -> Result<Resume<'resume, C>> {
        Ok(Resume {
            resume: RawResume(RawResumeInner::Yield(C::into_raw(), self.raw.slot)),
            _marker: PhantomData,
        })
    }

    #[inline]
    pub fn yield_to<Y: Coroutine<'sval, R, State = C::State> + ?Sized>(
        self,
    ) -> Result<Resume<'resume, C>> {
        Ok(Resume {
            resume: RawResume(RawResumeInner::Yield(Y::into_raw(), self.raw.slot)),
            _marker: PhantomData,
        })
    }

    #[inline]
    pub fn yield_into<
        Y: Coroutine<'sval, R> + ?Sized,
        T: Coroutine<'sval, R, State = C::State> + ?Sized,
    >(
        mut self,
        enter: fn(Pin<&mut C::State>) -> Pin<&mut Slot<Y::State>>,
    ) -> Result<Resume<'resume, C>> {
        let continuation = self.raw.slot;

        let enter = {
            let mut enter = enter(self.slot().state());

            enter
                .as_mut()
                .continue_with_raw(T::into_raw(), continuation);

            RawSlot::new::<R, Y>(enter)
        };

        Ok(Resume {
            resume: RawResume(RawResumeInner::Yield(Y::into_raw(), enter)),
            _marker: PhantomData,
        })
    }

    #[inline]
    pub fn yield_return(mut self) -> Result<Resume<'resume, C>> {
        Ok(Resume {
            resume: RawResume(RawResumeInner::Return(self.slot().continuation_raw())),
            _marker: PhantomData,
        })
    }
}

impl<S> Slot<S> {
    #[inline]
    pub fn new(state: S) -> Self {
        Slot {
            state,
            continuation: None,
            _pin: PhantomPinned,
        }
    }

    #[inline]
    fn state(self: Pin<&mut Self>) -> Pin<&mut S> {
        unsafe { self.map_unchecked_mut(|s| &mut s.state) }
    }

    #[inline]
    fn continue_with_raw(self: Pin<&mut Self>, resume: RawCoroutine, state: RawSlot) {
        let self_mut = unsafe { self.get_unchecked_mut() };

        debug_assert!(
            self_mut.continuation.is_none(),
            "attempt to override continuation"
        );

        self_mut.continuation = Some((resume, state));
    }

    #[inline]
    fn continuation_raw(self: Pin<&mut Self>) -> Option<(RawCoroutine, RawSlot)> {
        let self_mut = unsafe { self.get_unchecked_mut() };

        self_mut.continuation.take()
    }
}

enum Void {}

#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct RawResume(RawResumeInner);

#[derive(Clone, Copy)]
enum RawResumeInner {
    Yield(RawCoroutine, RawSlot),
    Return(Option<(RawCoroutine, RawSlot)>),
}

#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct RawContext {
    receiver: RawReceiver,
    slot: RawSlot,
}

#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct RawCoroutine(unsafe fn(RawContext) -> Result<RawResume>);

#[doc(hidden)]
#[derive(Clone, Copy)]
struct RawSlot(*mut Void);

#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct RawReceiver(*mut Void);

impl RawCoroutine {
    #[inline]
    fn new<'sval, R: Receiver<'sval>, C: Coroutine<'sval, R> + ?Sized>() -> Self {
        RawCoroutine(C::resume_raw)
    }

    #[inline]
    unsafe fn resume_raw(self, cx: RawContext) -> Result<RawResume> {
        (self.0)(cx)
    }
}

impl RawSlot {
    #[inline]
    fn new<'sval, R: Receiver<'sval>, C: Coroutine<'sval, R> + ?Sized>(
        slot: Pin<&mut Slot<C::State>>,
    ) -> Self {
        RawSlot((unsafe { slot.get_unchecked_mut() }) as *mut Slot<C::State> as *mut Void)
    }

    #[inline]
    unsafe fn get_unchecked_mut<'sval, R: Receiver<'sval>, C: Coroutine<'sval, R> + ?Sized>(
        &mut self,
    ) -> Pin<&mut Slot<C::State>> {
        Pin::new_unchecked(&mut *(self.0 as *mut Slot<C::State>))
    }
}

impl RawContext {
    #[inline]
    fn new(receiver: RawReceiver, slot: RawSlot) -> Self {
        RawContext { receiver, slot }
    }
}

impl RawReceiver {
    #[inline]
    fn new<'sval, R: Receiver<'sval>>(receiver: &mut R) -> Self {
        RawReceiver(receiver as *mut R as *mut Void)
    }

    #[inline]
    unsafe fn get_unchecked_mut<'sval, R: Receiver<'sval>>(&mut self) -> &mut R {
        &mut *(self.0 as *mut R)
    }
}
