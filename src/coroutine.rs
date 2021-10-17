use std::pin::Pin;

mod internal;
pub use self::internal::*;

use crate::{Receiver, Result};

pub trait CoroutineValue {
    #[doc(hidden)]
    type State<'a>
    where
        Self: 'a;

    #[doc(hidden)]
    type Coroutine<'a, R: Receiver<'a>>: Coroutine<'a, R, State = Self::State<'a>>
    where
        Self: 'a;

    #[doc(hidden)]
    fn state<'a>(&'a self) -> Self::State<'a>;

    fn stream<'a, R: Receiver<'a>>(&'a self, receiver: R) -> Result {
        let mut state = Slot::new(self.state());

        Driver::<R, Self::Coroutine<'a, R>>::new(receiver, unsafe {
            Pin::new_unchecked(&mut state)
        })
        .into_iter()
        .collect()
    }

    fn stream_iter<'a, R: Receiver<'a>>(&'a self, receiver: R) -> Result {
        let mut state = Slot::new(self.state());

        Driver::<R, Self::Coroutine<'a, R>>::new(receiver, unsafe {
            Pin::new_unchecked(&mut state)
        })
        .into_iter()
        .collect()
    }
}

impl<'b, T: CoroutineValue + ?Sized> CoroutineValue for &'b T {
    type State<'a>
    where
        T: 'a,
        'b: 'a,
    = T::State<'a>;

    type Coroutine<'a, R: Receiver<'a>>
    where
        T: 'a,
        'b: 'a,
    = T::Coroutine<'a, R>;

    #[inline]
    fn state<'a>(&'a self) -> Self::State<'a> {
        (**self).state()
    }

    fn stream_iter<'a, R: Receiver<'a>>(&'a self, receiver: R) -> Result {
        (**self).stream_iter(receiver)
    }

    fn stream<'a, R: Receiver<'a>>(&'a self, receiver: R) -> Result {
        (**self).stream(receiver)
    }
}

#[allow(dead_code, unused_mut)]
const _: () = {
    impl CoroutineValue for () {
        type State<'a> = ();

        type Coroutine<'a, R: Receiver<'a>> = UnitCoroutine;

        #[inline]
        fn state<'a>(&'a self) -> Self::State<'a> {
            ()
        }

        fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> Result {
            receiver.none()
        }
    }

    pub struct UnitCoroutine;

    impl<'a, R: Receiver<'a>> Coroutine<'a, R> for UnitCoroutine {
        type State = ();

        const MAY_YIELD: bool = false;

        fn resume<'resume>(
            cx: Context<'a, 'resume, R, Self>,
            receiver: &mut R,
        ) -> Result<Resume<'a, 'resume, R, Self>> {
            receiver.none()?;

            cx.yield_return()
        }
    }
};
