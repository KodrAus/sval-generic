use std::{
    marker::{PhantomData, PhantomPinned},
    pin::Pin,
};

#[doc(hidden)]
pub trait Coroutine {
    type State;

    const MAY_YIELD: bool = true;

    // TODO: This needs to accept a `R: Receiver<'a>`
    // That might affect the raw `fn` representation
    // So we might need to make this `Coroutine<'a, R: Receiver<'a>>`
    fn resume(cx: Context<Self>) -> Resume;

    #[doc(hidden)]
    fn into_raw() -> RawCoroutine {
        RawCoroutine::new::<Self>()
    }

    #[doc(hidden)]
    unsafe fn resume_raw(cx: RawContext) -> Resume {
        Self::resume(Context::from_raw_unchecked(cx))
    }
}

pub struct Resume(RawResume);

#[derive(Clone, Copy)]
enum RawResume {
    Yield(RawCoroutine, RawSlot),
    Return(Option<(RawCoroutine, RawSlot)>),
}

enum Void {}

#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct RawContext {
    slot: RawSlot,
}

#[doc(hidden)]
#[derive(Clone, Copy)]
struct RawSlot(*mut Void);

#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct RawCoroutine(unsafe fn(RawContext) -> Resume);

impl RawCoroutine {
    fn new<C: Coroutine + ?Sized>() -> Self {
        RawCoroutine(C::resume_raw)
    }

    unsafe fn resume_raw(self, cx: RawContext) -> Resume {
        (self.0)(cx)
    }
}

impl RawSlot {
    fn new(slot: *mut Void) -> Self {
        RawSlot(slot)
    }
}

impl RawContext {
    fn new<C: Coroutine + ?Sized>(slot: RawSlot) -> Self {
        RawContext { slot }
    }

    unsafe fn slot_unchecked_mut<C: Coroutine + ?Sized>(&mut self) -> Pin<&mut Slot<C::State>> {
        Pin::new_unchecked(&mut *(self.slot.0 as *mut Slot<C::State>))
    }
}

pub struct Context<C: Coroutine + ?Sized> {
    raw: RawContext,
    _marker: PhantomData<fn(&mut Slot<C::State>)>,
}

impl<C: Coroutine + ?Sized> Context<C> {
    unsafe fn from_raw_unchecked(raw: RawContext) -> Self {
        Context {
            raw,
            _marker: PhantomData,
        }
    }

    pub fn state(&mut self) -> Pin<&mut C::State> {
        unsafe { self.raw.slot_unchecked_mut::<C>() }.state()
    }

    pub fn yield_to<Y: Coroutine<State = C::State> + ?Sized>(self) -> Resume {
        Resume(RawResume::Yield(Y::into_raw(), self.raw.slot))
    }

    pub fn yield_into<Y: Coroutine + ?Sized, R: Coroutine<State = C::State> + ?Sized>(
        mut self,
        enter: fn(Pin<&mut C::State>) -> Pin<&mut Slot<Y::State>>,
    ) -> Resume {
        let continuation = self.raw.slot;

        let enter = {
            let mut enter = enter(unsafe { self.raw.slot_unchecked_mut::<C>() }.state());

            enter
                .as_mut()
                .continue_with_raw(R::into_raw(), continuation);

            (unsafe { enter.get_unchecked_mut() }) as *mut Slot<Y::State> as *mut Void
        };

        Resume(RawResume::Yield(Y::into_raw(), RawSlot::new(enter)))
    }

    pub fn yield_return(mut self) -> Resume {
        Resume(RawResume::Return(
            unsafe { self.raw.slot_unchecked_mut::<C>() }.continuation_raw(),
        ))
    }
}

pub struct Slot<S> {
    state: S,
    continuation: Option<(RawCoroutine, RawSlot)>,
    _pin: PhantomPinned,
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

pub struct Driver<'a, C: Coroutine + ?Sized> {
    resume: Option<(RawCoroutine, RawSlot)>,
    _marker: PhantomData<&'a mut Slot<C::State>>,
}

impl<'a, C: Coroutine + ?Sized> Driver<'a, C> {
    pub fn new(mut slot: Pin<&'a mut Slot<C::State>>) -> Self {
        let begin = RawSlot::new(
            (unsafe { slot.as_mut().get_unchecked_mut() }) as *mut Slot<C::State> as *mut Void,
        );

        Driver {
            resume: Some((C::into_raw(), begin)),
            _marker: PhantomData,
        }
    }

    pub fn resume(&mut self) -> bool {
        match self.resume.take() {
            Some((co, state)) => match unsafe { co.resume_raw(RawContext::new::<C>(state)) } {
                Resume(RawResume::Yield(resume, state)) => {
                    self.resume = Some((resume, state));
                    true
                }
                Resume(RawResume::Return(resume)) => {
                    self.resume = resume;
                    self.resume.is_some()
                }
            },
            None => false,
        }
    }

    pub fn into_iter(self) -> IntoIter<'a, C> {
        IntoIter(self)
    }
}

pub struct IntoIter<'a, C: Coroutine + ?Sized>(Driver<'a, C>);

impl<'a, C: Coroutine + ?Sized> Iterator for IntoIter<'a, C> {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        self.0.resume().then(|| ())
    }
}
