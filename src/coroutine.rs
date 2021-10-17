mod internal;
pub use self::internal::*;

use crate::Receiver;

pub trait CoroutineValue {
    #[doc(hidden)]
    type State<'a>;

    #[doc(hidden)]
    type Coroutine<'a, R: Receiver<'a>>: Coroutine<'a, R, State = Self::State<'a>>
    where
        Self: 'a;

    #[doc(hidden)]
    fn state<'a>(&'a self) -> Self::State<'a>;
}
