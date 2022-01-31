use crate::{std::fmt, Error, Receiver, Result, Value};

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
    fn take_owned(&mut self) -> Result<T::Owned, TakeError<Self::Error>>
    where
        T: ToOwned,
        T::Owned: Value,
    {
        #[cfg(not(feature = "std"))]
        {
            // The polyfilled `ToOwned` trait has no implementations
            // So this path is intentionally unreachable
            unreachable!()
        }
        #[cfg(feature = "std")]
        {
            self.take().map(ToOwned::to_owned)
        }
    }

    #[inline]
    fn try_take_ref(&mut self) -> Result<&'a R, TryTakeError<&T, Self::Error>> {
        Err(TryTakeError::Fallback(self.take()?))
    }

    #[inline]
    fn try_take_owned(&mut self) -> Result<T::Owned, TryTakeError<&T, Self::Error>>
    where
        T: ToOwned,
        T::Owned: Value,
    {
        Err(TryTakeError::Fallback(self.take()?))
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
    fn take_owned(&mut self) -> Result<T::Owned, TakeError<Self::Error>>
    where
        T: ToOwned,
        T::Owned: Value,
    {
        (**self).take_owned()
    }

    #[inline]
    fn try_take_ref(&mut self) -> Result<&'a R, TryTakeError<&T, Self::Error>> {
        (**self).try_take_ref()
    }

    #[inline]
    fn try_take_owned(&mut self) -> Result<T::Owned, TryTakeError<&T, Self::Error>>
    where
        T: ToOwned,
        T::Owned: Value,
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
    fn take_owned(&mut self) -> Result<T::Owned, TakeError<Self::Error>>
    where
        T: ToOwned,
        T::Owned: Value,
    {
        #[cfg(not(feature = "std"))]
        {
            unreachable!()
        }
        #[cfg(feature = "std")]
        {
            Ok(self.to_owned())
        }
    }

    #[inline]
    fn try_take_ref(&mut self) -> Result<&'a T, TryTakeError<&T, Self::Error>> {
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

mod private {
    mod no_implementations {
        pub trait NoImplementations {}
    }

    pub trait Polyfill: no_implementations::NoImplementations {}
}

#[cfg(not(feature = "alloc"))]
pub trait ToOwned: private::Polyfill {
    type Owned;
}

#[cfg(feature = "alloc")]
pub use crate::std::borrow::ToOwned;

#[cfg(feature = "alloc")]
mod alloc_support {
    use super::*;

    use crate::std::boxed::Box;

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
        fn try_take_ref(&mut self) -> Result<&'a R, TryTakeError<&T, Self::Error>> {
            (**self).try_take_ref()
        }

        #[inline]
        fn take_owned(&mut self) -> Result<T::Owned, TakeError<Self::Error>>
        where
            T: ToOwned,
            T::Owned: Value,
        {
            (**self).take_owned()
        }
    }
}
