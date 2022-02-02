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

use crate::{source, Receiver, Result, Source, SourceValue};

impl SourceValue for () {
    fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> crate::Result {
        receiver.null()
    }
}

impl<'a> Source<'a> for () {
    fn stream_next<'b, R: Receiver<'b>>(&mut self, receiver: R) -> crate::Result<source::Next>
    where
        'a: 'b,
    {
        self.stream_all(receiver).map(|_| source::Next::Done)
    }

    fn stream_all<'b, R: Receiver<'b>>(&mut self, mut receiver: R) -> crate::Result
    where
        'a: 'b,
    {
        receiver.null()
    }
}

impl<T: SourceValue> SourceValue for Option<T> {
    fn stream<'a, R: Receiver<'a>>(&'a self, receiver: R) -> crate::Result {
        self.as_ref().stream_all(receiver)
    }
}

impl<'a, T: Source<'a>> Source<'a> for Option<T> {
    fn stream_begin<'b, R: Receiver<'b>>(&mut self, mut receiver: R) -> crate::Result
    where
        'a: 'b,
    {
        match self {
            None => receiver.tagged_begin(
                tag()
                    .with_label("None")
                    .with_id(0)
                    .with_kind(tag::Kind::Nullable),
            ),
            Some(v) => {
                receiver.tagged_begin(
                    tag()
                        .with_label("Some")
                        .with_id(1)
                        .with_kind(tag::Kind::Nullable),
                )?;
                v.stream_begin(receiver)
            }
        }
    }

    fn stream_next<'b, R: Receiver<'b>>(&mut self, mut receiver: R) -> crate::Result<source::Next>
    where
        'a: 'b,
    {
        match self {
            None => {
                receiver.null()?;
                Ok(source::Next::Done)
            }
            Some(v) => v.stream_next(receiver),
        }
    }

    fn stream_end<'b, R: Receiver<'b>>(&mut self, mut receiver: R) -> crate::Result
    where
        'a: 'b,
    {
        match self {
            None => receiver.tagged_end(
                tag()
                    .with_label("None")
                    .with_id(0)
                    .with_kind(tag::Kind::Fallible),
            ),
            Some(v) => {
                v.stream_end(&mut receiver)?;
                receiver.tagged_end(
                    tag()
                        .with_label("Some")
                        .with_id(1)
                        .with_kind(tag::Kind::Fallible),
                )
            }
        }
    }

    fn stream_all<'b, R: Receiver<'b>>(&mut self, mut receiver: R) -> crate::Result
    where
        'a: 'b,
    {
        match self {
            None => tagged(())
                .with_label("None")
                .with_id(0)
                .with_kind(tag::Kind::Nullable)
                .stream_all(receiver),
            Some(v) => tagged(v)
                .with_label("Some")
                .with_id(1)
                .with_kind(tag::Kind::Nullable)
                .stream_all(receiver),
        }
    }
}

impl SourceValue for bool {
    fn stream<'a, R: Receiver<'a>>(&'a self, receiver: R) -> crate::Result {
        (*self).stream_all(receiver)
    }
}

impl<'a> Source<'a> for bool {
    fn stream_next<'b, R: Receiver<'b>>(&mut self, receiver: R) -> crate::Result<source::Next>
    where
        'a: 'b,
    {
        self.stream_all(receiver).map(|_| source::Next::Done)
    }

    fn stream_all<'b, R: Receiver<'b>>(&mut self, mut receiver: R) -> crate::Result
    where
        'a: 'b,
    {
        receiver.bool(*self)
    }
}
