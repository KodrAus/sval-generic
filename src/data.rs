pub(crate) mod bytes;
pub(crate) mod digits;
pub(crate) mod error;
pub(crate) mod seq;
pub(crate) mod text;

pub mod tag;

#[doc(inline)]
pub use self::{
    bytes::{bytes, Bytes},
    error::Error,
    tag::{tag, tagged, Tag, Tagged},
    text::{text, Text},
};

#[cfg(feature = "std")]
#[doc(inline)]
pub use self::error::error;

use crate::{source, Receiver, Source, SourceValue};

impl SourceValue for () {
    fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> crate::Result {
        receiver.null()
    }
}

impl<'a> Source<'a> for () {
    fn stream_resume<'b, R: Receiver<'b>>(&mut self, receiver: R) -> crate::Result<source::Resume>
    where
        'a: 'b,
    {
        self.stream_to_end(receiver).map(|_| source::Resume::Done)
    }

    fn stream_to_end<'b, R: Receiver<'b>>(&mut self, mut receiver: R) -> crate::Result
    where
        'a: 'b,
    {
        receiver.null()
    }
}

impl<T: SourceValue> SourceValue for Option<T> {
    fn stream<'a, R: Receiver<'a>>(&'a self, receiver: R) -> crate::Result {
        self.as_ref().stream_to_end(receiver)
    }
}

impl<'a, T: Source<'a>> Source<'a> for Option<T> {
    fn stream_resume<'b, R: Receiver<'b>>(&mut self, receiver: R) -> crate::Result<source::Resume>
    where
        'a: 'b,
    {
        self.stream_to_end(receiver).map(|_| source::Resume::Done)
    }

    fn stream_to_end<'b, R: Receiver<'b>>(&mut self, mut receiver: R) -> crate::Result
    where
        'a: 'b,
    {
        match self {
            None => tagged(())
                .with_label("None")
                .with_id(0)
                .with_kind(tag::Kind::Nullable)
                .stream_to_end(receiver),
            Some(v) => tagged(v)
                .with_label("Some")
                .with_id(1)
                .with_kind(tag::Kind::Nullable)
                .stream_to_end(receiver),
        }
    }
}

impl SourceValue for bool {
    fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> crate::Result {
        receiver.bool(*self)
    }
}

impl<'a> Source<'a> for bool {
    fn stream_resume<'b, R: Receiver<'b>>(&mut self, receiver: R) -> crate::Result<source::Resume>
    where
        'a: 'b,
    {
        self.stream_to_end(receiver).map(|_| source::Resume::Done)
    }

    fn stream_to_end<'b, R: Receiver<'b>>(&mut self, mut receiver: R) -> crate::Result
    where
        'a: 'b,
    {
        receiver.bool(*self)
    }
}
