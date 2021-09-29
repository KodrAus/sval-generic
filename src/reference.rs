use crate::{for_all::ForAll, stream::Stream, value::Value, Result};

pub trait ValueRef<'a>: Value + Copy {
    fn stream<'b, S: Stream<'b>>(self, stream: S) -> Result
    where
        'a: 'b;

    fn to_str(self) -> Option<&'a str>;

    fn for_all(self) -> ForAll<Self>
    where
        Self: Sized,
    {
        ForAll(self)
    }
}

pub trait TypedRef<'a, T: ?Sized + Value + 'static>: ValueRef<'a> {
    fn get(&self) -> &T;
    fn try_unwrap(self) -> Option<&'a T>;
}

impl<'a, T: Value + ?Sized> ValueRef<'a> for &'a T {
    fn stream<'b, S: Stream<'b>>(self, stream: S) -> Result
    where
        'a: 'b,
    {
        (*self).stream(stream)
    }

    fn to_str(self) -> Option<&'a str> {
        (*self).to_str()
    }
}

impl<'a, T: Value + ?Sized + 'static> TypedRef<'a, T> for &'a T {
    fn get(&self) -> &T {
        self
    }

    fn try_unwrap(self) -> Option<&'a T> {
        Some(self)
    }
}
