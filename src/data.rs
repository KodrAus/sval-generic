mod bytes;
mod digits;
mod error;
mod seq;
mod text;

pub mod tag;

#[doc(inline)]
pub use self::{
    bytes::{bytes, Bytes},
    error::Error,
    tag::{tag, tagged, Tag, Tagged, VariantTagged},
};

#[doc(inline)]
pub use crate::std::fmt::Display;

#[cfg(feature = "std")]
#[doc(inline)]
pub use self::error::error;

use crate::{
    source::{self, ValueSource},
    Receiver, Source, Value,
};

impl Value for () {
    fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> crate::Result {
        receiver.null()
    }
}

impl<'a> Source<'a> for () {
    fn stream_resume<'b, R: Receiver<'b>>(&mut self, receiver: R) -> crate::Result<source::Stream>
    where
        'a: 'b,
    {
        self.stream_to_end(receiver).map(|_| source::Stream::Done)
    }

    fn stream_to_end<'b, R: Receiver<'b>>(&mut self, mut receiver: R) -> crate::Result
    where
        'a: 'b,
    {
        receiver.null()
    }
}

impl<'a> ValueSource<'a, ()> for () {
    type Error = source::Impossible;

    #[inline]
    fn take(&mut self) -> Result<&(), source::TakeError<Self::Error>> {
        Ok(self)
    }
}

impl<T: Value> Value for Option<T> {
    fn stream<'a, R: Receiver<'a>>(&'a self, receiver: R) -> crate::Result {
        match self {
            Some(v) => tagged(v)
                .with_label("Some")
                .with_id(1)
                .with_kind(tag::Kind::Nullable)
                .stream_to_end(receiver),
            None => tagged(())
                .with_label("None")
                .with_id(0)
                .with_kind(tag::Kind::Nullable)
                .stream_to_end(receiver),
        }
    }
}

impl Value for bool {
    fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> crate::Result {
        receiver.bool(*self)
    }
}

impl<'a> Source<'a> for bool {
    fn stream_resume<'b, R: Receiver<'b>>(&mut self, receiver: R) -> crate::Result<source::Stream>
    where
        'a: 'b,
    {
        self.stream_to_end(receiver).map(|_| source::Stream::Done)
    }

    fn stream_to_end<'b, R: Receiver<'b>>(&mut self, mut receiver: R) -> crate::Result
    where
        'a: 'b,
    {
        receiver.bool(*self)
    }
}

impl<'a> ValueSource<'a, bool> for bool {
    type Error = source::Impossible;

    fn take(&mut self) -> Result<&bool, source::TakeError<Self::Error>> {
        Ok(self)
    }
}
