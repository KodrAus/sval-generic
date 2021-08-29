use std::ops::Deref;

use crate::{stream::Stream, value::Value, Result};

pub trait AnyRef<'a>: Copy {
    fn stream<'b, S>(self, stream: S) -> Result
    where
        'a: 'b,
        S: Stream<'b>;

    fn stream_for_all<'b, S>(self, stream: S) -> Result
    where
        S: Stream<'b>;
}

// TODO: Consider requirement for same `stream` impl?
pub trait TypedRef<'a, T: ?Sized>: AnyRef<'a> + Deref<Target = T> {
    fn to_ref(self) -> Option<&'a T>;
}

impl<'a, T: ?Sized> AnyRef<'a> for &'a T
where
    T: Value,
{
    fn stream<'b, S>(self, stream: S) -> Result
    where
        'a: 'b,
        S: Stream<'b>,
    {
        (*self).stream(stream)
    }

    fn stream_for_all<'b, S>(self, stream: S) -> Result
    where
        S: Stream<'b>,
    {
        (*self).stream_for_all(stream)
    }
}

impl<'a, T: ?Sized> TypedRef<'a, T> for &'a T
where
    T: Value,
{
    fn to_ref(self) -> Option<&'a T> {
        Some(self)
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

impl<'a, 'b, T> AnyRef<'a> for ForAll<T>
where
    T: AnyRef<'b>,
{
    fn stream<'c, S>(self, stream: S) -> Result
    where
        'a: 'c,
        S: Stream<'c>,
    {
        self.0.stream_for_all(stream)
    }

    fn stream_for_all<'c, S>(self, stream: S) -> Result
    where
        S: Stream<'c>,
    {
        // TODO: Can we remove the `stream_for_all` method from `Value`?
        self.0.stream_for_all(stream)
    }
}

impl<'a, 'b, T, U: ?Sized> TypedRef<'a, U> for ForAll<T>
where
    T: TypedRef<'b, U>,
{
    fn to_ref(self) -> Option<&'a U> {
        None
    }
}
