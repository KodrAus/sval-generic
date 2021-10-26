use std::pin::Pin;

use crate::{
    co::{Coroutine, RefMutSource, Resume},
    Receiver, Result,
};

use sval_generic_api::{Source, Value};

pub trait CoroutineValue: Value {
    type State<'a>
    where
        Self: 'a;
    type Resume<'a, R: Receiver<'a>>: Resume<'a, R, State = Self::State<'a>>
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
    type Resume<'a, R: Receiver<'a>>
    where
        T: 'a,
        'b: 'a,
    = T::Resume<'a, R>;

    #[inline]
    fn state<'a>(&'a self) -> Self::State<'a> {
        (**self).state()
    }
}
