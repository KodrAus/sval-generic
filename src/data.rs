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
    fn take(&mut self) -> crate::Result<&(), source::TakeError<Self::Error>> {
        Ok(self)
    }
}

impl<T: Value> Value for Option<T> {
    fn stream<'a, R: Receiver<'a>>(&'a self, receiver: R) -> crate::Result {
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

impl<T: Value, E: Value> Value for Result<T, E> {
    fn stream<'a, R: Receiver<'a>>(&'a self, receiver: R) -> crate::Result {
        match self {
            Ok(v) => tagged(v)
                .with_label("Ok")
                .with_id(0)
                .with_kind(tag::Kind::Fallible)
                .stream_to_end(receiver),
            Err(e) => tagged(e)
                .with_label("Err")
                .with_id(1)
                .with_kind(tag::Kind::Fallible)
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

    fn take(&mut self) -> crate::Result<&bool, source::TakeError<Self::Error>> {
        Ok(self)
    }
}

#[cfg(feature = "alloc")]
mod alloc_support {
    use crate::{
        for_all,
        source::{self, ToOwned, TryTakeError},
        std::borrow::{Borrow, Cow},
        Receiver, Result, Source, Value, ValueSource,
    };

    impl<'a, V: ToOwned + Value + ?Sized> Source<'a> for Cow<'a, V>
    where
        V::Owned: Value,
    {
        fn stream_resume<'b, R: Receiver<'b>>(
            &mut self,
            receiver: R,
        ) -> crate::Result<source::Stream>
        where
            'a: 'b,
        {
            self.stream_to_end(receiver).map(|_| source::Stream::Done)
        }

        fn stream_to_end<'b, R: Receiver<'b>>(&mut self, receiver: R) -> crate::Result
        where
            'a: 'b,
        {
            match self {
                Cow::Borrowed(v) => v.stream(receiver),
                Cow::Owned(v) => v.stream(for_all(receiver)),
            }
        }
    }

    impl<'a, 'r, V: ToOwned + Value + ?Sized> Source<'a> for &'r Cow<'a, V>
    where
        V::Owned: Value,
    {
        fn stream_resume<'b, R: Receiver<'b>>(
            &mut self,
            receiver: R,
        ) -> crate::Result<source::Stream>
        where
            'a: 'b,
        {
            self.stream_to_end(receiver).map(|_| source::Stream::Done)
        }

        fn stream_to_end<'b, R: Receiver<'b>>(&mut self, receiver: R) -> crate::Result
        where
            'a: 'b,
        {
            match self {
                Cow::Borrowed(v) => (*v).stream(receiver),
                Cow::Owned(v) => v.stream(for_all(receiver)),
            }
        }
    }

    impl<'a, V: ToOwned + Value + ?Sized> ValueSource<'a, V> for Cow<'a, V>
    where
        V::Owned: Value,
    {
        type Error = source::Impossible;

        #[inline]
        fn take(&mut self) -> Result<&V, source::TakeError<Self::Error>> {
            Ok(&**self)
        }

        #[inline]
        fn try_take_ref(&mut self) -> Result<&'a V, TryTakeError<&V, Self::Error>> {
            match self {
                Cow::Borrowed(v) => Ok(*v),
                Cow::Owned(ref v) => Err(TryTakeError::Fallback(v.borrow())),
            }
        }

        // NOTE: With specialization we could specialize for `V::Owned: Default` for `try_take_owned`
    }

    impl<'a, 'r, V: ToOwned + Value + ?Sized> ValueSource<'a, V> for &'r Cow<'a, V>
    where
        V::Owned: Value,
    {
        type Error = source::Impossible;

        #[inline]
        fn take(&mut self) -> Result<&V, source::TakeError<Self::Error>> {
            Ok(&**self)
        }

        #[inline]
        fn try_take_ref(&mut self) -> Result<&'a V, TryTakeError<&V, Self::Error>> {
            match self {
                Cow::Borrowed(v) => Ok(*v),
                Cow::Owned(ref v) => Err(TryTakeError::Fallback(v.borrow())),
            }
        }
    }
}
