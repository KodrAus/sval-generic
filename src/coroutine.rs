mod internal;
pub use self::internal::*;

pub trait CoroutineValue {
    #[doc(hidden)]
    type State<'a>;

    #[doc(hidden)]
    type Coroutine<'a>: Coroutine<State = Self::State<'a>>
    where
        Self: 'a;

    #[doc(hidden)]
    fn state<'a>(&'a self) -> Self::State<'a>;
}
