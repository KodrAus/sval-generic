use std::{
    marker::{PhantomData, PhantomPinned},
    pin::Pin,
};

use crate::{Receiver, Result};

pub trait Coroutine<'a, R: Receiver<'a>> {
    type State;

    const MAY_YIELD: bool = true;

    fn resume<'resume, 'receiver>(
        cx: Context<'a, 'resume, R, Self>,
        receiver: &'receiver mut R,
    ) -> Result<Resume<'a, 'resume, R, Self>>;

    #[doc(hidden)]
    fn into_raw() -> RawCoroutine {
        RawCoroutine::new::<R, Self>()
    }

    #[doc(hidden)]
    unsafe fn resume_raw(cx: RawContext, mut receiver: RawReceiver) -> Result<RawResume> {
        Self::resume(
            Context::from_raw_unchecked(cx),
            receiver.get_unchecked_mut::<R>(),
        )
        .map(|r| r.resume)
    }
}

pub struct Driver<'a, 'driver, R: Receiver<'a>, C: Coroutine<'a, R> + ?Sized> {
    resume: Option<(RawCoroutine, RawSlot)>,
    receiver: R,
    _marker: PhantomData<&'driver mut Slot<C::State>>,
}

impl<'a, 'driver, R: Receiver<'a>, C: Coroutine<'a, R> + ?Sized> Driver<'a, 'driver, R, C> {
    pub fn new(receiver: R, mut slot: Pin<&'driver mut Slot<C::State>>) -> Self {
        let begin = RawSlot::new::<R, C>(slot.as_mut());

        Driver {
            resume: Some((C::into_raw(), begin)),
            receiver,
            _marker: PhantomData,
        }
    }

    pub fn resume(&mut self) -> Result<bool> {
        match self.resume.take() {
            Some((co, state)) => match unsafe {
                co.resume_raw(
                    RawContext::new::<R, C>(state),
                    RawReceiver::new(&mut self.receiver),
                )?
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

    pub fn into_iter(self) -> IntoIter<'a, 'driver, R, C> {
        IntoIter(self)
    }
}

pub struct IntoIter<'a, 'driver, R: Receiver<'a>, C: Coroutine<'a, R> + ?Sized>(
    Driver<'a, 'driver, R, C>,
);

impl<'a, 'driver, R: Receiver<'a>, C: Coroutine<'a, R> + ?Sized> Iterator
    for IntoIter<'a, 'driver, R, C>
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

pub struct Resume<'a, 'resume, R: Receiver<'a>, C: Coroutine<'a, R> + ?Sized> {
    resume: RawResume,
    _marker: PhantomData<Context<'a, 'resume, R, C>>,
}

pub struct Context<'a, 'resume, R: Receiver<'a>, C: Coroutine<'a, R> + ?Sized> {
    raw: RawContext,
    _marker: PhantomData<&'resume mut Slot<C::State>>,
}

pub struct Slot<S> {
    state: S,
    continuation: Option<(RawCoroutine, RawSlot)>,
    _pin: PhantomPinned,
}

impl<'a, 'resume, R: Receiver<'a>, C: Coroutine<'a, R> + ?Sized> Context<'a, 'resume, R, C> {
    unsafe fn from_raw_unchecked(raw: RawContext) -> Self {
        Context {
            raw,
            _marker: PhantomData,
        }
    }

    fn slot(&mut self) -> Pin<&mut Slot<C::State>> {
        unsafe { self.raw.slot.get_unchecked_mut::<R, C>() }
    }

    pub fn state(&mut self) -> Pin<&mut C::State> {
        self.slot().state()
    }

    pub fn yield_to<Y: Coroutine<'a, R, State = C::State> + ?Sized>(
        self,
    ) -> Result<Resume<'a, 'resume, R, C>> {
        Ok(Resume {
            resume: RawResume(RawResumeInner::Yield(Y::into_raw(), self.raw.slot)),
            _marker: PhantomData,
        })
    }

    pub fn yield_into<
        Y: Coroutine<'a, R> + ?Sized,
        T: Coroutine<'a, R, State = C::State> + ?Sized,
    >(
        mut self,
        enter: fn(Pin<&mut C::State>) -> Pin<&mut Slot<Y::State>>,
    ) -> Result<Resume<'a, 'resume, R, C>> {
        let continuation = self.raw.slot;

        let enter = {
            let mut enter = enter(self.state());

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

    pub fn yield_return(mut self) -> Result<Resume<'a, 'resume, R, C>> {
        Ok(Resume {
            resume: RawResume(RawResumeInner::Return(self.slot().continuation_raw())),
            _marker: PhantomData,
        })
    }
}

impl<S> Slot<S> {
    pub fn new(state: S) -> Self {
        Slot {
            state,
            continuation: None,
            _pin: PhantomPinned,
        }
    }

    fn state(self: Pin<&mut Self>) -> Pin<&mut S> {
        unsafe { self.map_unchecked_mut(|s| &mut s.state) }
    }

    fn continue_with_raw(self: Pin<&mut Self>, resume: RawCoroutine, state: RawSlot) {
        let self_mut = unsafe { self.get_unchecked_mut() };

        debug_assert!(
            self_mut.continuation.is_none(),
            "attempt to override continuation"
        );

        self_mut.continuation = Some((resume, state));
    }

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
    slot: RawSlot,
}

#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct RawCoroutine(unsafe fn(RawContext, RawReceiver) -> Result<RawResume>);

#[doc(hidden)]
#[derive(Clone, Copy)]
struct RawSlot(*mut Void);

#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct RawReceiver(*mut Void);

impl RawCoroutine {
    fn new<'a, R: Receiver<'a>, C: Coroutine<'a, R> + ?Sized>() -> Self {
        RawCoroutine(C::resume_raw)
    }

    unsafe fn resume_raw(self, cx: RawContext, receiver: RawReceiver) -> Result<RawResume> {
        (self.0)(cx, receiver)
    }
}

impl RawSlot {
    fn new<'a, R: Receiver<'a>, C: Coroutine<'a, R> + ?Sized>(
        slot: Pin<&mut Slot<C::State>>,
    ) -> Self {
        RawSlot((unsafe { slot.get_unchecked_mut() }) as *mut Slot<C::State> as *mut Void)
    }

    unsafe fn get_unchecked_mut<'a, R: Receiver<'a>, C: Coroutine<'a, R> + ?Sized>(
        &mut self,
    ) -> Pin<&mut Slot<C::State>> {
        Pin::new_unchecked(&mut *(self.0 as *mut Slot<C::State>))
    }
}

impl RawContext {
    fn new<'a, R: Receiver<'a>, C: Coroutine<'a, R> + ?Sized>(slot: RawSlot) -> Self {
        RawContext { slot }
    }
}

impl RawReceiver {
    fn new<'a, R: Receiver<'a>>(receiver: &mut R) -> Self {
        RawReceiver(receiver as *mut R as *mut Void)
    }

    unsafe fn get_unchecked_mut<'a, R: Receiver<'a>>(&mut self) -> &mut R {
        &mut *(self.0 as *mut R)
    }
}
