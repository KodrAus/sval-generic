use std::{borrow::ToOwned, fmt};

use crate::Value;

pub use crate::{
    for_all::{for_all, ForAll},
    tag::{type_tag, variant_tag},
    Error, Receiver, Result,
};

pub fn stream_to_end<'a>(s: impl Receiver<'a>, mut v: impl Source<'a>) -> Result {
    v.stream_to_end(s)
}

pub trait Source<'a> {
    fn stream<'b, R: Receiver<'b>>(&mut self, receiver: R) -> Result<Stream>
    where
        'a: 'b;

    fn stream_to_end<'b, R: Receiver<'b>>(&mut self, mut receiver: R) -> Result
    where
        'a: 'b,
    {
        while let Stream::Yield = self.stream(&mut receiver)? {}

        Ok(())
    }
}

impl<'a, 'b, T: Source<'a> + ?Sized> Source<'a> for &'b mut T {
    fn stream<'c, S: Receiver<'c>>(&mut self, receiver: S) -> Result<Stream>
    where
        'a: 'c,
    {
        (**self).stream(receiver)
    }

    fn stream_to_end<'c, R: Receiver<'c>>(&mut self, receiver: R) -> Result
    where
        'a: 'c,
    {
        (**self).stream_to_end(receiver)
    }
}

#[must_use]
pub enum Stream {
    Yield,
    Done,
}

#[derive(Debug, Clone, Copy)]
pub enum Impossible {}

impl From<Impossible> for Error {
    fn from(_: Impossible) -> Error {
        unreachable!()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TakeError<E>(E);

impl<E> TakeError<E> {
    pub fn from_error(err: E) -> Self {
        TakeError(err)
    }

    pub fn into_error(self) -> E {
        self.0
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TakeRefError<T, E>(Result<T, E>);

impl<T, E> TakeRefError<T, E> {
    pub fn from_value(value: T) -> Self {
        TakeRefError(Ok(value))
    }

    pub fn from_error(err: E) -> Self {
        TakeRefError(Err(err))
    }

    pub fn from_result(r: Result<T, E>) -> Self {
        TakeRefError(r)
    }

    pub fn into_result(self) -> Result<T, TakeError<E>> {
        self.0.map_err(TakeError)
    }
}

impl<E: Into<Error>> From<TakeError<E>> for Error {
    fn from(err: TakeError<E>) -> Error {
        err.0.into()
    }
}

impl<T, E: Into<Error>> From<TakeRefError<T, E>> for Error {
    fn from(err: TakeRefError<T, E>) -> Error {
        match err.into_result() {
            Ok(_) => Error,
            Err(err) => err.into(),
        }
    }
}

impl<T, E> From<TakeError<E>> for TakeRefError<T, E> {
    fn from(err: TakeError<E>) -> TakeRefError<T, E> {
        TakeRefError::from_error(err.into_error())
    }
}

pub trait ValueSource<'a, T: Value + ?Sized>: Source<'a> {
    type Error: Into<Error> + fmt::Debug;

    fn take(&mut self) -> Result<&T, TakeError<Self::Error>>;

    #[inline]
    fn take_ref(&mut self) -> Result<&'a T, TakeRefError<&T, Self::Error>> {
        Err(TakeRefError::from_value(self.take()?))
    }

    #[inline]
    fn take_owned(&mut self) -> Result<T::Owned, TakeError<Self::Error>>
    where
        T: ToOwned,
        T::Owned: Value,
    {
        self.take().map(ToOwned::to_owned)
    }
}

impl<'a, 'b, T: Value + ?Sized, S: ValueSource<'a, T> + ?Sized> ValueSource<'a, T> for &'b mut S {
    type Error = S::Error;

    #[inline]
    fn take(&mut self) -> Result<&T, TakeError<Self::Error>> {
        (**self).take()
    }

    #[inline]
    fn take_ref(&mut self) -> Result<&'a T, TakeRefError<&T, Self::Error>> {
        (**self).take_ref()
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

impl<'a, T: Value + ?Sized> Source<'a> for &'a T {
    fn stream<'b, R: Receiver<'b>>(&mut self, receiver: R) -> Result<Stream>
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
    fn take_ref(&mut self) -> Result<&'a T, TakeRefError<&T, Self::Error>> {
        Ok(self)
    }

    #[inline]
    fn take_owned(&mut self) -> Result<T::Owned, TakeError<Self::Error>>
    where
        T: ToOwned,
        T::Owned: Value,
    {
        Ok(self.to_owned())
    }
}
