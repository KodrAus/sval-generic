use crate::{for_all::ForAll, source::Source, stream::Stream, Result};

pub trait SourceRef<'a>: Source + Copy {
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

pub trait TypedRef<'a, T: ?Sized + Source + 'static>: SourceRef<'a> {
    fn get(&self) -> &T;
    fn try_unwrap(self) -> Option<&'a T>;
}

impl<'a, T: Source + ?Sized> SourceRef<'a> for &'a T {
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

impl<'a, T: Source + ?Sized + 'static> TypedRef<'a, T> for &'a T {
    fn get(&self) -> &T {
        self
    }

    fn try_unwrap(self) -> Option<&'a T> {
        Some(self)
    }
}
