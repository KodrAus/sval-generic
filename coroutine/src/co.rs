use std::{
    marker::{PhantomData, PhantomPinned, Unpin},
    pin::Pin,
};

use crate::Result;

pub trait Resume<'a, R> {
    type State;

    const MAY_YIELD: bool = true;

    fn resume(cx: Context<Self, R>) -> Result<Yield<Self>>;

    #[doc(hidden)]
    fn into_raw() -> RawResume {
        RawResume::new::<Self, R>()
    }

    #[doc(hidden)]
    unsafe fn resume_raw(cx: RawContext) -> Result<RawYield> {
        Self::resume(Context::from_raw_unchecked(cx)).map(|r| r.resume)
    }
}

pub struct RefMutSource<'a, 'b, R, C: Resume<'a, R> + ?Sized> {
    resume: Option<(RawResume, RawCoroutine)>,
    receiver: R,
    _marker: PhantomData<&'b mut Coroutine<C::State>>,
}

impl<'a, 'b, R, C: Resume<'a, R> + ?Sized> Unpin for RefMutSource<'a, 'b, R, C> {}

impl<'a, 'b, R, C: Resume<'a, R> + ?Sized> RefMutSource<'a, 'b, R, C> {
    #[inline]
    pub fn new(receiver: R, slot: Pin<&'b mut Coroutine<C::State>>) -> Self {
        let begin = RawCoroutine::new::<C::State>(slot);

        RefMutSource {
            resume: Some((C::into_raw(), begin)),
            receiver,
            _marker: PhantomData,
        }
    }

    #[inline]
    fn resume(&mut self) -> Result<bool> {
        match self.resume.take() {
            Some((co, state)) => match unsafe {
                co.resume_raw(RawContext::new(RawReceiver::new(&mut self.receiver), state))?
            } {
                RawYield(RawYieldInner::Yield(resume, state)) => {
                    self.resume = Some((resume, state));
                    Ok(true)
                }
                RawYield(RawYieldInner::Return(resume)) => {
                    self.resume = resume;
                    Ok(self.resume.is_some())
                }
            },
            None => Ok(false),
        }
    }

    pub fn into_iter(self) -> IntoIter<'a, 'b, R, C> {
        IntoIter(self)
    }
}

pub struct IntoIter<'a, 'b, R, C: Resume<'a, R> + ?Sized>(RefMutSource<'a, 'b, R, C>);

impl<'a, 'b, R, C: Resume<'a, R> + ?Sized> Iterator for IntoIter<'a, 'b, R, C> {
    type Item = Result;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0.resume() {
            Ok(true) => Some(Ok(())),
            Ok(false) => None,
            Err(e) => Some(Err(e)),
        }
    }
}

pub struct Yield<'r, C: ?Sized> {
    resume: RawYield,
    _marker: PhantomData<&'r mut C>,
}

pub struct Context<'r, C: ?Sized, R: ?Sized> {
    raw: RawContext,
    _marker: PhantomData<fn(&'r mut C, &'r mut R)>,
}

#[repr(C)]
pub struct Coroutine<S> {
    continuation: Option<(RawResume, RawCoroutine)>,
    state: S,
    _pin: PhantomPinned,
}

impl<'a, 'r, R, C: Resume<'a, R> + ?Sized> Context<'r, C, R> {
    #[inline]
    unsafe fn from_raw_unchecked(raw: RawContext) -> Self {
        Context {
            raw,
            _marker: PhantomData,
        }
    }

    #[inline]
    fn slot(&mut self) -> Pin<&mut Coroutine<C::State>> {
        unsafe { self.raw.slot.get_unchecked_mut::<C::State>() }
    }

    #[inline]
    pub fn state(&mut self) -> (&mut R, Pin<&mut C::State>) {
        unsafe {
            (
                self.raw.receiver.get_unchecked_mut::<R>(),
                self.raw.slot.get_unchecked_mut::<C::State>().state(),
            )
        }
    }

    #[inline]
    pub fn receiver(&mut self) -> &mut R {
        unsafe { self.raw.receiver.get_unchecked_mut::<R>() }
    }

    #[inline]
    pub fn yield_resume_self(self) -> Result<Yield<'r, C>> {
        Ok(Yield {
            resume: RawYield(RawYieldInner::Yield(C::into_raw(), self.raw.slot)),
            _marker: PhantomData,
        })
    }

    #[inline]
    pub fn yield_resume<Y: Resume<'a, R, State = C::State> + ?Sized>(self) -> Result<Yield<'r, C>> {
        Ok(Yield {
            resume: RawYield(RawYieldInner::Yield(Y::into_raw(), self.raw.slot)),
            _marker: PhantomData,
        })
    }

    #[inline]
    pub fn yield_into_resume<
        Y: Resume<'a, R> + ?Sized,
        T: Resume<'a, R, State = C::State> + ?Sized,
    >(
        mut self,
        enter: fn(Pin<&mut C::State>) -> Pin<&mut Coroutine<Y::State>>,
    ) -> Result<Yield<'r, C>> {
        let continuation = self.raw.slot;

        let enter = {
            let mut enter = enter(self.slot().state());

            enter
                .as_mut()
                .continue_with_raw(T::into_raw(), continuation);

            RawCoroutine::new::<Y::State>(enter)
        };

        Ok(Yield {
            resume: RawYield(RawYieldInner::Yield(Y::into_raw(), enter)),
            _marker: PhantomData,
        })
    }

    #[inline]
    pub fn yield_into_resume_self<Y: Resume<'a, R> + ?Sized>(
        mut self,
        enter: fn(Pin<&mut C::State>) -> Pin<&mut Coroutine<Y::State>>,
    ) -> Result<Yield<'r, C>> {
        let continuation = self.raw.slot;

        let enter = {
            let mut enter = enter(self.slot().state());

            enter
                .as_mut()
                .continue_with_raw(C::into_raw(), continuation);

            RawCoroutine::new::<Y::State>(enter)
        };

        Ok(Yield {
            resume: RawYield(RawYieldInner::Yield(Y::into_raw(), enter)),
            _marker: PhantomData,
        })
    }

    #[inline]
    pub fn yield_into_return<Y: Resume<'a, R> + ?Sized>(
        mut self,
        enter: fn(Pin<&mut C::State>) -> Pin<&mut Coroutine<Y::State>>,
    ) -> Result<Yield<'r, C>> {
        let continuation = self.raw.slot;

        let enter = {
            let mut enter = enter(self.slot().state());

            enter
                .as_mut()
                .continue_with_raw(RawResume::return_raw(), continuation);

            RawCoroutine::new::<Y::State>(enter)
        };

        Ok(Yield {
            resume: RawYield(RawYieldInner::Yield(Y::into_raw(), enter)),
            _marker: PhantomData,
        })
    }

    #[inline]
    pub fn yield_return(mut self) -> Result<Yield<'r, C>> {
        Ok(Yield {
            resume: RawYield(RawYieldInner::Return(self.slot().continuation_raw())),
            _marker: PhantomData,
        })
    }
}

impl<S> Coroutine<S> {
    #[inline]
    pub fn new(state: S) -> Self {
        Coroutine {
            continuation: None,
            state,
            _pin: PhantomPinned,
        }
    }

    #[inline]
    fn state(self: Pin<&mut Self>) -> Pin<&mut S> {
        unsafe { self.map_unchecked_mut(|s| &mut s.state) }
    }

    #[inline]
    fn continue_with_raw(self: Pin<&mut Self>, resume: RawResume, state: RawCoroutine) {
        let self_mut = unsafe { self.get_unchecked_mut() };

        debug_assert!(
            self_mut.continuation.is_none(),
            "attempt to override continuation"
        );

        self_mut.continuation = Some((resume, state));
    }

    #[inline]
    fn continuation_raw(self: Pin<&mut Self>) -> Option<(RawResume, RawCoroutine)> {
        let self_mut = unsafe { self.get_unchecked_mut() };

        self_mut.continuation.take()
    }
}

enum Void {}

#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct RawYield(RawYieldInner);

#[derive(Clone, Copy)]
enum RawYieldInner {
    Yield(RawResume, RawCoroutine),
    Return(Option<(RawResume, RawCoroutine)>),
}

#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct RawContext {
    receiver: RawReceiver,
    slot: RawCoroutine,
}

#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct RawResume(unsafe fn(RawContext) -> Result<RawYield>);

#[doc(hidden)]
#[derive(Clone, Copy)]
struct RawCoroutine(*mut Void);

#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct RawReceiver(*mut Void);

impl RawResume {
    #[inline]
    fn new<'a, C: Resume<'a, R> + ?Sized, R>() -> Self {
        RawResume(C::resume_raw)
    }

    #[inline]
    unsafe fn resume_raw(self, cx: RawContext) -> Result<RawYield> {
        (self.0)(cx)
    }

    #[inline]
    fn return_raw() -> RawResume {
        RawResume(|cx| {
            let continuation = unsafe { &mut *(cx.slot.0 as *mut Coroutine<Void>) }
                .continuation
                .take();

            Ok(RawYield(RawYieldInner::Return(continuation)))
        })
    }
}

impl RawCoroutine {
    #[inline]
    fn new<'a, S>(slot: Pin<&mut Coroutine<S>>) -> Self {
        RawCoroutine((unsafe { slot.get_unchecked_mut() }) as *mut Coroutine<S> as *mut Void)
    }

    #[inline]
    unsafe fn get_unchecked_mut<'a, S>(&mut self) -> Pin<&mut Coroutine<S>> {
        Pin::new_unchecked(&mut *(self.0 as *mut Coroutine<S>))
    }
}

impl RawContext {
    #[inline]
    fn new(receiver: RawReceiver, slot: RawCoroutine) -> Self {
        RawContext { receiver, slot }
    }
}

impl RawReceiver {
    #[inline]
    fn new<R>(receiver: &mut R) -> Self {
        RawReceiver(receiver as *mut R as *mut Void)
    }

    #[inline]
    unsafe fn get_unchecked_mut<R>(&mut self) -> &mut R {
        &mut *(self.0 as *mut R)
    }
}
