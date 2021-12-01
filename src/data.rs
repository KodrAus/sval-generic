mod bytes;
mod error;
mod seq;
mod text;

pub mod digits;
pub mod tag;

#[doc(inline)]
pub use self::{
    bytes::{bytes, Bytes},
    digits::{digits, digits_unchecked, Digits},
    error::Error,
    tag::{type_tag, variant_tag, TypeTag, VariantTag},
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
        receiver.none()
    }
}

impl<'a> Source<'a> for () {
    fn stream<'b, R: Receiver<'b>>(&mut self, receiver: R) -> crate::Result<source::Stream>
    where
        'a: 'b,
    {
        self.stream_to_end(receiver).map(|_| source::Stream::Done)
    }

    fn stream_to_end<'b, R: Receiver<'b>>(&mut self, mut receiver: R) -> crate::Result
    where
        'a: 'b,
    {
        receiver.none()
    }
}

impl<'a> ValueSource<'a, ()> for () {
    type Error = source::Impossible;

    #[inline]
    fn take(&mut self) -> Result<&(), source::TakeError<Self::Error>> {
        Ok(self)
    }
}

impl Value for bool {
    fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> crate::Result {
        receiver.bool(*self)
    }
}

impl<'a> Source<'a> for bool {
    fn stream<'b, R: Receiver<'b>>(&mut self, receiver: R) -> crate::Result<source::Stream>
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

impl<T> Value for Option<T>
where
    T: Value,
{
    fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> crate::Result {
        match self {
            Some(v) => v.stream(receiver),
            None => receiver.none(),
        }
    }
}

#[cfg(feature = "alloc")]
mod alloc_support {
    use super::*;

    use crate::std::boxed::Box;

    impl<T: Value + ?Sized> Value for Box<T> {
        fn stream<'a, S: Receiver<'a>>(&'a self, stream: S) -> crate::Result {
            (**self).stream(stream)
        }
    }
}
