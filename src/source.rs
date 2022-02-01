use crate::{std::fmt, Error, Receiver, Result, Value};

#[cfg(feature = "alloc")]
use crate::std::borrow::{Cow, ToOwned};

pub fn stream_to_end<'a>(s: impl Receiver<'a>, mut v: impl Source<'a>) -> Result {
    v.stream_to_end(s)
}

pub trait Source<'a> {
    fn stream_resume<'b, R: Receiver<'b>>(&mut self, receiver: R) -> Result<Stream>
    where
        'a: 'b;

    fn stream_to_end<'b, R: Receiver<'b>>(&mut self, mut receiver: R) -> Result
    where
        'a: 'b,
    {
        while let Stream::Yield = self.stream_resume(&mut receiver)? {}

        Ok(())
    }
}

#[must_use]
pub enum Stream {
    Yield,
    Done,
}

impl<'a, 'b, T: Source<'a> + ?Sized> Source<'a> for &'b mut T {
    fn stream_resume<'c, S: Receiver<'c>>(&mut self, receiver: S) -> Result<Stream>
    where
        'a: 'c,
    {
        (**self).stream_resume(receiver)
    }

    fn stream_to_end<'c, R: Receiver<'c>>(&mut self, receiver: R) -> Result
    where
        'a: 'c,
    {
        (**self).stream_to_end(receiver)
    }
}

impl<'a, S: Source<'a>> Source<'a> for Option<S> {
    fn stream_resume<'b, R: Receiver<'b>>(&mut self, mut receiver: R) -> Result<Stream>
    where
        'a: 'b,
    {
        match self {
            Some(source) => source.stream_resume(receiver),
            None => {
                receiver.null()?;

                Ok(Stream::Done)
            }
        }
    }

    fn stream_to_end<'b, R: Receiver<'b>>(&mut self, mut receiver: R) -> Result
    where
        'a: 'b,
    {
        match self {
            Some(source) => source.stream_to_end(receiver),
            None => receiver.null(),
        }
    }
}

pub trait ValueSource<'a, T: Value + ?Sized, R: Value + ?Sized = T>: Source<'a> {
    type Error: Into<Error> + fmt::Debug + fmt::Display;

    fn take(&mut self) -> Result<&T, TakeError<Self::Error>>;

    #[inline]
    #[cfg(feature = "alloc")]
    fn try_take_owned(&mut self) -> Result<&T, TryTakeError<T::Owned, Self::Error>>
    where
        T: ToOwned,
    {
        Ok(self.take()?)
    }

    #[inline]
    fn try_take_ref(&mut self) -> Result<&'a R, TryTakeError<&T, Self::Error>> {
        Err(TryTakeError::Fallback(self.take()?))
    }

    #[inline]
    #[cfg(feature = "alloc")]
    fn try_take_owned_ref(&mut self) -> Result<&'a R, TryTakeError<T::Owned, Self::Error>>
    where
        T: ToOwned,
    {
        match self.try_take_ref() {
            Ok(v) => Ok(v),
            Err(TryTakeError::Fallback(v)) => Err(TryTakeError::Fallback(v.to_owned())),
            Err(TryTakeError::Err(e)) => Err(TryTakeError::Err(e)),
        }
    }
}

impl<'a, 'b, T: Value + ?Sized, R: Value + ?Sized, S: ValueSource<'a, T, R> + ?Sized>
    ValueSource<'a, T, R> for &'b mut S
{
    type Error = S::Error;

    #[inline]
    fn take(&mut self) -> Result<&T, TakeError<Self::Error>> {
        (**self).take()
    }

    #[inline]
    #[cfg(feature = "alloc")]
    fn take_owned(&mut self) -> Result<Cow<T>, TakeError<Self::Error>>
    where
        T: ToOwned,
    {
        (**self).take_owned()
    }

    #[inline]
    fn try_take_ref(&mut self) -> Result<&'a R, TryTakeError<&T, Self::Error>> {
        (**self).try_take_ref()
    }

    #[inline]
    #[cfg(feature = "alloc")]
    fn try_take_owned(&mut self) -> Result<&'a R, TryTakeError<Cow<T>, Self::Error>>
    where
        T: ToOwned,
    {
        (**self).try_take_owned()
    }
}

impl<'a, T: Value + ?Sized> Source<'a> for &'a T {
    fn stream_resume<'b, R: Receiver<'b>>(&mut self, receiver: R) -> Result<Stream>
    where
        'a: 'b,
    {
        self.stream_to_end(receiver).map(|_| Stream::Done)
    }

    fn stream_to_end<'b, R: Receiver<'b>>(&mut self, mut receiver: R) -> Result
    where
        'a: 'b,
    {
        receiver.value(*self)
    }
}

impl<'a, T: Value + ?Sized> ValueSource<'a, T> for &'a T {
    type Error = Impossible;

    #[inline]
    fn take(&mut self) -> Result<&T, TakeError<Self::Error>> {
        Ok(self)
    }

    #[inline]
    #[cfg(feature = "alloc")]
    fn take_owned(&mut self) -> Result<Cow<T>, TakeError<Self::Error>>
    where
        T: ToOwned,
    {
        Ok(Cow::Borrowed(self))
    }

    #[inline]
    fn try_take_ref(&mut self) -> Result<&'a T, TryTakeError<&T, Self::Error>> {
        Ok(self)
    }

    #[inline]
    #[cfg(feature = "alloc")]
    fn try_take_owned(&mut self) -> Result<&'a T, TryTakeError<Cow<T>, Self::Error>>
    where
        T: ToOwned,
    {
        Ok(self)
    }
}

#[derive(Clone, Copy)]
pub enum Impossible {}

impl fmt::Debug for Impossible {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        unreachable!()
    }
}

impl fmt::Display for Impossible {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        unreachable!()
    }
}

impl From<Impossible> for Error {
    fn from(_: Impossible) -> Error {
        unreachable!()
    }
}

impl Value for Impossible {
    fn stream<'a, R: Receiver<'a>>(&'a self, _: R) -> Result {
        unreachable!()
    }
}

impl<'a> Source<'a> for Impossible {
    fn stream_resume<'b, R: Receiver<'b>>(&mut self, _: R) -> Result<Stream>
    where
        'a: 'b,
    {
        unreachable!()
    }
}

impl<'a, V: Value + ?Sized, U: Value + ?Sized> ValueSource<'a, V, U> for Impossible {
    type Error = Impossible;

    fn take(&mut self) -> Result<&V, TakeError<Self::Error>> {
        unreachable!()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TakeError<E>(E);

impl<E> TakeError<E> {
    pub fn from_error(err: E) -> Self {
        TakeError(err)
    }

    pub fn map_err<U>(self, f: impl FnOnce(E) -> U) -> TakeError<U> {
        TakeError(f(self.0))
    }

    pub fn into_error(self) -> E {
        self.0
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TryTakeError<T, E> {
    Fallback(T),
    Err(TakeError<E>),
}

impl<T, E> TryTakeError<T, E> {
    pub fn from_result(r: Result<T, E>) -> Self {
        match r {
            Ok(fallback) => TryTakeError::Fallback(fallback),
            Err(e) => TryTakeError::Err(TakeError::from_error(e)),
        }
    }

    pub fn into_result(self) -> Result<T, E> {
        match self {
            TryTakeError::Fallback(fallback) => Ok(fallback),
            TryTakeError::Err(err) => Err(err.into_error()),
        }
    }
}

impl<E: Into<Error>> From<TakeError<E>> for Error {
    fn from(err: TakeError<E>) -> Error {
        err.0.into()
    }
}

impl<T, E: Into<Error>> From<TryTakeError<T, E>> for Error {
    fn from(err: TryTakeError<T, E>) -> Error {
        match err {
            TryTakeError::Fallback(_) => Error,
            TryTakeError::Err(e) => e.into(),
        }
    }
}

impl<T, E> From<TakeError<E>> for TryTakeError<T, E> {
    fn from(err: TakeError<E>) -> TryTakeError<T, E> {
        TryTakeError::Err(err)
    }
}

#[cfg(feature = "alloc")]
mod alloc_support {
    use super::*;

    use crate::{
        for_all,
        source::{self, TryTakeError},
        std::{
            borrow::{Borrow, Cow, ToOwned},
            boxed::Box,
        },
        Receiver, Result, Source, Value, ValueSource,
    };

    impl<'a, T: Source<'a> + ?Sized> Source<'a> for Box<T> {
        fn stream_resume<'c, S: Receiver<'c>>(&mut self, receiver: S) -> Result<Stream>
        where
            'a: 'c,
        {
            (**self).stream_resume(receiver)
        }

        fn stream_to_end<'c, R: Receiver<'c>>(&mut self, receiver: R) -> Result
        where
            'a: 'c,
        {
            (**self).stream_to_end(receiver)
        }
    }

    impl<'a, 'b, T: Value + ?Sized, R: Value + ?Sized, S: ValueSource<'a, T, R> + ?Sized>
        ValueSource<'a, T, R> for Box<S>
    {
        type Error = S::Error;

        #[inline]
        fn take(&mut self) -> Result<&T, TakeError<Self::Error>> {
            (**self).take()
        }

        #[inline]
        fn take_owned(&mut self) -> Result<Cow<T>, TakeError<Self::Error>>
        where
            T: ToOwned,
        {
            (**self).take_owned()
        }

        #[inline]
        fn try_take_ref(&mut self) -> Result<&'a R, TryTakeError<&T, Self::Error>> {
            (**self).try_take_ref()
        }

        #[inline]
        fn try_take_owned(&mut self) -> Result<&'a R, TryTakeError<Cow<T>, Self::Error>>
        where
            T: ToOwned,
        {
            (**self).try_take_owned()
        }
    }

    impl<'a, V: ToOwned + Value + ?Sized> Source<'a> for Cow<'a, V> {
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
                Cow::Owned(v) => {
                    let v: &V = (*v).borrow();
                    v.stream(for_all(receiver))
                }
            }
        }
    }

    impl<'a, 'b, V: ToOwned + Value + ?Sized> Source<'a> for &'b Cow<'a, V> {
        fn stream_resume<'c, R: Receiver<'c>>(
            &mut self,
            receiver: R,
        ) -> crate::Result<source::Stream>
        where
            'a: 'c,
        {
            self.stream_to_end(receiver).map(|_| source::Stream::Done)
        }

        fn stream_to_end<'c, R: Receiver<'c>>(&mut self, receiver: R) -> crate::Result
        where
            'a: 'c,
        {
            match self {
                Cow::Borrowed(v) => (*v).stream(receiver),
                Cow::Owned(v) => v.borrow().stream(for_all(receiver)),
            }
        }
    }

    impl<'a, V: ToOwned + Value + ?Sized> ValueSource<'a, V> for Cow<'a, V> {
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

        // NOTE: With specialization we could specialize for `V::Owned: Default` for `take_owned` and `try_take_owned`
    }

    impl<'a, 'b, V: ToOwned + Value + ?Sized> ValueSource<'a, V> for &'b Cow<'a, V> {
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

        // NOTE: With specialization we could specialize for `V::Owned: Default` for `take_owned` and `try_take_owned`
    }
}
