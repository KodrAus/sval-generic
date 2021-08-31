use std::ops::Deref;

use crate::{for_all::ForAll, stream::Stream, value::Value, Result};

pub trait UnknownValueRef<'a>: Copy + Deref {
    fn stream<'b, S>(self, stream: S) -> Result
    where
        'a: 'b,
        S: Stream<'b>;

    fn for_all(self) -> ForAll<Self>
    where
        Self: Sized,
    {
        ForAll(self)
    }
}

pub trait TypedValueRef<'a, T: ?Sized>: UnknownValueRef<'a> + Deref<Target = T> {
    fn to_ref(self) -> Option<&'a T>;
}

impl<'a, T: ?Sized> UnknownValueRef<'a> for &'a T
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
}

// TODO: Should we implement this for `String`, `Arc` etc?
impl<'a, T: ?Sized> TypedValueRef<'a, T> for &'a T
where
    T: Value,
{
    fn to_ref(self) -> Option<&'a T> {
        Some(self)
    }
}
