use crate::{for_all::ForAll, stream::Stream, value::Value, Result};

pub trait UnknownValueRef<'a>: Copy {
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

pub trait TypedValueRef<'a, T: ?Sized + Value>: UnknownValueRef<'a> {
    fn get(&self) -> &T;
    fn get_ref(&self) -> Option<&'a T>;
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

impl<'a, T: ?Sized> TypedValueRef<'a, T> for &'a T
where
    T: Value,
{
    fn get(&self) -> &T {
        self
    }

    fn get_ref(&self) -> Option<&'a T> {
        Some(self)
    }
}
