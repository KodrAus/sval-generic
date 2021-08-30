use std::ops::Deref;

use crate::{stream::Stream, value::Value, Result};

// TODO: Consider `AnyValueRef` and `TypedValueRef`?
pub trait AnyRef<'a>: Copy + Deref {
    fn stream<'b, S>(self, stream: S) -> Result
    where
        'a: 'b,
        S: Stream<'b>;

    // TODO: Can we remove this now `erased` is fixed?
    fn stream_for_all<'b, S>(self, stream: S) -> Result
    where
        S: Stream<'b>;

    fn for_all(self) -> ForAll<Self>
    where
        Self: Sized,
    {
        ForAll(self)
    }
}

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

impl<T> Value for ForAll<T>
where
    T: Value,
{
    fn stream<'a, S>(&'a self, stream: S) -> Result
    where
        S: Stream<'a>,
    {
        self.0.stream_for_all(stream)
    }

    fn stream_for_all<'a, S>(&self, stream: S) -> Result
    where
        S: Stream<'a>,
    {
        self.0.stream_for_all(stream)
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
