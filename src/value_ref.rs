use crate::{for_all::ForAll, stream::Stream, value::Value, Result};

pub trait UnknownStreamValue<'a>: Value + Copy {
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

pub trait StreamValue<'a, T: ?Sized + Value>: UnknownStreamValue<'a> {
    fn get(&self) -> &T;
    fn get_ref(&self) -> Option<&'a T>;
}

impl<'a, T: ?Sized> UnknownStreamValue<'a> for &'a T
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

impl<'a, T: ?Sized> StreamValue<'a, T> for &'a T
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
