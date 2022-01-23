use crate::{co::Resume, Receiver};

use sval::Value;

pub trait CoroutineValue: Value {
    type State<'a>
    where
        Self: 'a;
    type Coroutine<'a, R: Receiver<'a>>: Resume<'a, R, State = Self::State<'a>>
    where
        Self: 'a;

    fn state<'a>(&'a self) -> Self::State<'a>;
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
}
