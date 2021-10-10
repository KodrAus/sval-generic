use std::{borrow::ToOwned, fmt};

use crate::{Error, Receiver, Result, Value};

pub trait Source<'a> {
    fn stream<'b, R: Receiver<'b>>(&mut self, receiver: R) -> Result
    where
        'a: 'b;
}

impl<'a, 'b, T: Source<'a> + ?Sized> Source<'a> for &'b mut T {
    fn stream<'c, S: Receiver<'c>>(&mut self, stream: S) -> Result
    where
        'a: 'c,
    {
        (**self).stream(stream)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Impossible {}

impl From<Impossible> for Error {
    fn from(_: Impossible) -> Error {
        unreachable!()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ToRefError<T, E>(Result<T, E>);

impl<T, E> ToRefError<T, E> {
    pub fn from_value(value: T) -> Self {
        ToRefError(Ok(value))
    }

    pub fn from_error(err: E) -> Self {
        ToRefError(Err(err))
    }

    pub fn from_result(r: Result<T, E>) -> Self {
        ToRefError(r)
    }

    pub fn into_result(self) -> Result<T, ToValueError<E>> {
        self.0.map_err(ToValueError::from_error)
    }
}

impl<T, E: Into<Error>> From<ToRefError<T, E>> for Error {
    fn from(err: ToRefError<T, E>) -> Error {
        match err.into_result() {
            Ok(_) => Error,
            Err(err) => err.into(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ToValueError<E>(E);

impl<E> ToValueError<E> {
    pub fn from_error(err: E) -> Self {
        ToValueError(err)
    }

    pub fn into_inner(self) -> E {
        self.0
    }
}

impl<E: Into<Error>> From<ToValueError<E>> for Error {
    fn from(err: ToValueError<E>) -> Error {
        err.0.into()
    }
}

pub trait ValueSource<'a, T: Value + ?Sized>: Source<'a> {
    type Error: Into<Error> + fmt::Debug;

    fn value(&mut self) -> Result<&T, ToValueError<Self::Error>>;

    #[inline]
    fn value_ref(&mut self) -> Result<&'a T, ToRefError<&T, Self::Error>> {
        Err(ToRefError::from_result(
            self.value().map_err(|e| e.into_inner()),
        ))
    }

    #[inline]
    fn value_owned(&mut self) -> Result<T::Owned, ToValueError<Self::Error>>
    where
        T: ToOwned,
        T::Owned: Value,
    {
        self.value().map(ToOwned::to_owned)
    }
}

impl<'a, 'b, T: Value + ?Sized, S: ValueSource<'a, T> + ?Sized> ValueSource<'a, T> for &'b mut S {
    type Error = S::Error;

    #[inline]
    fn value(&mut self) -> Result<&T, ToValueError<Self::Error>> {
        (**self).value()
    }

    #[inline]
    fn value_ref(&mut self) -> Result<&'a T, ToRefError<&T, Self::Error>> {
        (**self).value_ref()
    }

    #[inline]
    fn value_owned(&mut self) -> Result<T::Owned, ToValueError<Self::Error>>
    where
        T: ToOwned,
        T::Owned: Value,
    {
        (**self).value_owned()
    }
}

impl<'a, T: Value + ?Sized> Source<'a> for &'a T {
    fn stream<'b, R: Receiver<'b>>(&mut self, mut receiver: R) -> Result
    where
        'a: 'b,
    {
        receiver.value(*self)
    }
}

impl<'a, T: Value + ?Sized> ValueSource<'a, T> for &'a T {
    type Error = Impossible;

    #[inline]
    fn value(&mut self) -> Result<&T, ToValueError<Self::Error>> {
        Ok(self)
    }

    #[inline]
    fn value_ref(&mut self) -> Result<&'a T, ToRefError<&T, Self::Error>> {
        Ok(self)
    }

    #[inline]
    fn value_owned(&mut self) -> Result<T::Owned, ToValueError<Self::Error>>
    where
        T: ToOwned,
        T::Owned: Value,
    {
        Ok(self.to_owned())
    }
}
