use std::ops::Deref;

use crate::{stream::Stream, value::Value, Result};

pub trait UntypedValue<'a>: Copy {
    fn stream<'b, S>(&self, stream: S) -> Result
    where
        'a: 'b,
        S: Stream<'b>;

    fn stream_for_all<'b, S>(&self, stream: S) -> Result
    where
        S: Stream<'b>;
}

pub trait TypedValue<'a, T: ?Sized>: UntypedValue<'a> + Deref<Target = T> {
    fn to_ref(&self) -> Option<&'a T>;
}

impl<'a, T: ?Sized> UntypedValue<'a> for &'a T
where
    T: Value,
{
    fn stream<'b, S>(&self, stream: S) -> Result
    where
        'a: 'b,
        S: Stream<'b>,
    {
        (**self).stream(stream)
    }

    fn stream_for_all<'b, S>(&self, stream: S) -> Result
    where
        S: Stream<'b>,
    {
        (**self).stream_for_all(stream)
    }
}

impl<'a, T: ?Sized> TypedValue<'a, T> for &'a T
where
    T: Value,
{
    fn to_ref(&self) -> Option<&'a T> {
        Some(*self)
    }
}

#[derive(Clone, Copy)]
pub struct ForAll<T>(pub T);

impl<T: Deref> Deref for ForAll<T> {
    type Target = T::Target;

    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

impl<'a, 'b, T> UntypedValue<'a> for ForAll<T>
where
    T: UntypedValue<'b>,
{
    fn stream<'c, S>(&self, stream: S) -> Result
    where
        'a: 'c,
        S: Stream<'c>,
    {
        self.0.stream_for_all(stream)
    }

    fn stream_for_all<'c, S>(&self, stream: S) -> Result
    where
        S: Stream<'c>,
    {
        self.0.stream_for_all(stream)
    }
}

impl<'a, 'b, T, U: ?Sized> TypedValue<'a, U> for ForAll<T>
where
    T: TypedValue<'b, U>,
{
    fn to_ref(&self) -> Option<&'a U> {
        None
    }
}
