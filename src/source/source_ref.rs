use crate::{source::Resume, std::fmt, Error, Receiver, Result, Source, SourceValue};

#[cfg(feature = "alloc")]
use crate::std::borrow::ToOwned;

pub trait SourceRef<'a, T: SourceValue + ?Sized, R: SourceValue + ?Sized = T>
where
    Self: Source<'a>,
{
    type Error: Into<Error> + fmt::Debug + fmt::Display;

    fn take(&mut self) -> Result<&T, TakeError<Self::Error>>;

    #[inline]
    #[cfg(feature = "alloc")]
    fn take_owned(&mut self) -> Result<T::Owned, TakeError<Self::Error>>
    where
        T: ToOwned,
    {
        self.take().map(ToOwned::to_owned)
    }

    #[inline]
    fn try_take(&mut self) -> Result<&'a R, TryTakeError<&T, Self::Error>> {
        Err(TryTakeError::Fallback(self.take()?))
    }

    #[inline]
    #[cfg(feature = "alloc")]
    fn try_take_owned(&mut self) -> Result<&'a R, TryTakeError<T::Owned, Self::Error>>
    where
        T: ToOwned,
    {
        self.try_take()
            .map_err(|e| e.map_fallback(ToOwned::to_owned))
    }
}

impl<'a, 'b, T: SourceValue + ?Sized, R: SourceValue + ?Sized, S: SourceRef<'a, T, R> + ?Sized>
    SourceRef<'a, T, R> for &'b mut S
{
    type Error = S::Error;

    #[inline]
    fn take(&mut self) -> Result<&T, TakeError<Self::Error>> {
        (**self).take()
    }

    #[inline]
    #[cfg(feature = "alloc")]
    fn take_owned(&mut self) -> Result<T::Owned, TakeError<Self::Error>>
    where
        T: ToOwned,
    {
        (**self).take_owned()
    }

    #[inline]
    fn try_take(&mut self) -> Result<&'a R, TryTakeError<&T, Self::Error>> {
        (**self).try_take()
    }

    #[inline]
    #[cfg(feature = "alloc")]
    fn try_take_owned(&mut self) -> Result<&'a R, TryTakeError<T::Owned, Self::Error>>
    where
        T: ToOwned,
    {
        (**self).try_take_owned()
    }
}

impl<'a, T: SourceValue + ?Sized> SourceRef<'a, T> for &'a T {
    type Error = Impossible;

    #[inline]
    fn take(&mut self) -> Result<&T, TakeError<Self::Error>> {
        Ok(self)
    }

    #[inline]
    #[cfg(feature = "alloc")]
    fn take_owned(&mut self) -> Result<T::Owned, TakeError<Self::Error>>
    where
        T: ToOwned,
    {
        Ok(self.to_owned())
    }

    #[inline]
    fn try_take(&mut self) -> Result<&'a T, TryTakeError<&T, Self::Error>> {
        Ok(self)
    }

    #[inline]
    #[cfg(feature = "alloc")]
    fn try_take_owned(&mut self) -> Result<&'a T, TryTakeError<T::Owned, Self::Error>>
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

impl SourceValue for Impossible {
    fn stream<'a, R: Receiver<'a>>(&'a self, _: R) -> Result {
        unreachable!()
    }
}

impl<'a> Source<'a> for Impossible {
    fn stream_resume<'b, R: Receiver<'b>>(&mut self, _: R) -> Result<Resume>
    where
        'a: 'b,
    {
        unreachable!()
    }
}

impl<'a, V: SourceValue + ?Sized, U: SourceValue + ?Sized> SourceRef<'a, V, U> for Impossible {
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

    pub fn map_fallback<U>(self, f: impl FnOnce(T) -> U) -> TryTakeError<U, E> {
        match self {
            TryTakeError::Fallback(fallback) => TryTakeError::Fallback(f(fallback)),
            TryTakeError::Err(e) => TryTakeError::Err(e),
        }
    }

    pub fn map_err<U>(self, f: impl FnOnce(E) -> U) -> TryTakeError<T, U> {
        match self {
            TryTakeError::Fallback(fallback) => TryTakeError::Fallback(fallback),
            TryTakeError::Err(e) => TryTakeError::Err(e.map_err(f)),
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
        std::{borrow::ToOwned, boxed::Box},
        Result, SourceRef, SourceValue,
    };

    impl<
            'a,
            'b,
            T: SourceValue + ?Sized,
            R: SourceValue + ?Sized,
            S: SourceRef<'a, T, R> + ?Sized,
        > SourceRef<'a, T, R> for Box<S>
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
        {
            (**self).take_owned()
        }

        #[inline]
        fn try_take(&mut self) -> Result<&'a R, TryTakeError<&T, Self::Error>> {
            (**self).try_take()
        }

        #[inline]
        fn try_take_owned(&mut self) -> Result<&'a R, TryTakeError<T::Owned, Self::Error>>
        where
            T: ToOwned,
        {
            (**self).try_take_owned()
        }
    }
}
