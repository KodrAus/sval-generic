mod bytes;
mod error;
mod number;
mod seq;
mod tag;
mod text;

#[doc(inline)]
pub use self::{bytes::*, error::*, tag::*, text::*};

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

    fn stream_to_end<'b, R: Receiver<'b>>(&mut self, receiver: R) -> crate::Result
    where
        'a: 'b,
    {
        match self {
            None => tagged_nullable(())
                .with_id(0)
                .with_label("None")
                .stream_to_end(receiver),
            Some(v) => tagged_nullable(v)
                .with_id(1)
                .with_label("Some")
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

#[cfg(feature = "alloc")]
mod alloc_support {
    use super::*;

    use crate::{
        for_all, source,
        std::{
            borrow::{Borrow, Cow, ToOwned},
            mem,
            string::String,
        },
        Result, SourceRef, SourceValue,
    };

    impl<'a, V: ToOwned + SourceValue + ?Sized> SourceValue for Cow<'a, V> {
        fn stream<'b, S: Receiver<'b>>(&'b self, receiver: S) -> crate::Result {
            match self {
                Cow::Borrowed(v) => v.stream(receiver),
                Cow::Owned(ref v) => (*v).borrow().stream(for_all(receiver)),
            }
        }
    }

    trait SpecializedSource<'a> {
        fn specialized_stream_to_end<'b, R: Receiver<'b>>(&mut self, receiver: R) -> Result
        where
            'a: 'b;
    }

    trait SpecializedSourceRef<'a, T: ToOwned + SourceValue + ?Sized> {
        fn specialized_take(&mut self) -> Result<&T, source::TakeError<source::Impossible>>;

        fn specialized_take_owned(
            &mut self,
        ) -> Result<T::Owned, source::TakeError<source::Impossible>>;

        fn specialized_try_take(
            &mut self,
        ) -> Result<&'a T, source::TryTakeError<&T, source::Impossible>>;

        fn specialized_try_take_owned(
            &mut self,
        ) -> Result<&'a T, source::TryTakeError<T::Owned, source::Impossible>>;
    }

    impl<'a, V: ToOwned + SourceValue + ?Sized> SpecializedSourceRef<'a, V> for Cow<'a, V> {
        #[inline]
        default fn specialized_take(
            &mut self,
        ) -> Result<&V, source::TakeError<source::Impossible>> {
            Ok(&**self)
        }

        #[inline]
        default fn specialized_take_owned(
            &mut self,
        ) -> Result<V::Owned, source::TakeError<source::Impossible>> {
            Ok((&**self).to_owned())
        }

        #[inline]
        default fn specialized_try_take(
            &mut self,
        ) -> Result<&'a V, source::TryTakeError<&V, source::Impossible>> {
            match self {
                Cow::Borrowed(v) => Ok(*v),
                Cow::Owned(v) => Err(source::TryTakeError::Fallback((*v).borrow())),
            }
        }

        #[inline]
        default fn specialized_try_take_owned(
            &mut self,
        ) -> Result<&'a V, source::TryTakeError<V::Owned, source::Impossible>> {
            match self {
                Cow::Borrowed(v) => Ok(*v),
                Cow::Owned(v) => Err(source::TryTakeError::Fallback((*v).borrow().to_owned())),
            }
        }
    }

    impl<'a> SpecializedSourceRef<'a, str> for Cow<'a, str> {
        #[inline]
        default fn specialized_take(
            &mut self,
        ) -> Result<&str, source::TakeError<source::Impossible>> {
            Ok(&**self)
        }

        #[inline]
        default fn specialized_take_owned(
            &mut self,
        ) -> Result<String, source::TakeError<source::Impossible>> {
            Ok(mem::take(self).into_owned())
        }

        #[inline]
        default fn specialized_try_take(
            &mut self,
        ) -> Result<&'a str, source::TryTakeError<&str, source::Impossible>> {
            match self {
                Cow::Borrowed(v) => Ok(*v),
                Cow::Owned(v) => Err(source::TryTakeError::Fallback(&**v)),
            }
        }

        #[inline]
        default fn specialized_try_take_owned(
            &mut self,
        ) -> Result<&'a str, source::TryTakeError<String, source::Impossible>> {
            match mem::take(self) {
                Cow::Borrowed(v) => Ok(v),
                Cow::Owned(v) => Err(source::TryTakeError::Fallback(v)),
            }
        }
    }

    impl<'a, V: ToOwned + SourceValue + ?Sized> SpecializedSource<'a> for Cow<'a, V> {
        default fn specialized_stream_to_end<'b, R: Receiver<'b>>(
            &mut self,
            receiver: R,
        ) -> crate::Result
        where
            'a: 'b,
        {
            match self {
                Cow::Borrowed(v) => v.stream(receiver),
                Cow::Owned(v) => (*v).borrow().stream_to_end(for_all(receiver)),
            }
        }
    }

    impl<'a> SpecializedSource<'a> for Cow<'a, str> {
        fn specialized_stream_to_end<'b, R: Receiver<'b>>(
            &mut self,
            mut receiver: R,
        ) -> crate::Result
        where
            'a: 'b,
        {
            match mem::take(self) {
                Cow::Borrowed(v) => receiver.str(v),
                Cow::Owned(v) => receiver.str(Cow::Owned(v)),
            }
        }
    }

    impl<'a, V: ToOwned + SourceValue + ?Sized> SourceRef<'a, V> for Cow<'a, V> {
        type Error = source::Impossible;

        #[inline]
        fn take(&mut self) -> Result<&V, source::TakeError<Self::Error>> {
            self.specialized_take()
        }

        #[inline]
        fn take_owned(&mut self) -> Result<V::Owned, source::TakeError<Self::Error>> {
            self.specialized_take_owned()
        }

        #[inline]
        fn try_take(&mut self) -> Result<&'a V, source::TryTakeError<&V, Self::Error>> {
            self.specialized_try_take()
        }

        #[inline]
        fn try_take_owned(&mut self) -> Result<&'a V, source::TryTakeError<V::Owned, Self::Error>> {
            self.specialized_try_take_owned()
        }
    }

    impl<'a, V: ToOwned + SourceValue + ?Sized> Source<'a> for Cow<'a, V> {
        fn stream_resume<'b, R: Receiver<'b>>(
            &mut self,
            receiver: R,
        ) -> crate::Result<source::Resume>
        where
            'a: 'b,
        {
            self.stream_to_end(receiver).map(|_| source::Resume::Done)
        }

        fn stream_to_end<'b, R: Receiver<'b>>(&mut self, receiver: R) -> crate::Result
        where
            'a: 'b,
        {
            self.specialized_stream_to_end(receiver)
        }
    }
}
