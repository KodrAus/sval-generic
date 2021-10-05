use std::fmt;

use crate::{for_all::ForAll, stream::Stream, value::Value, Error, Result};

pub trait Source<'a> {
    fn stream<'b, S: Stream<'b>>(&mut self, stream: S) -> Result
    where
        'a: 'b;

    fn for_all(&mut self) -> ForAll<&mut Self>
    where
        Self: Sized,
    {
        ForAll(self)
    }
}

impl<'a, 'b, T: Source<'a> + ?Sized> Source<'a> for &'b mut T {
    fn stream<'c, S: Stream<'c>>(&mut self, stream: S) -> Result
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

pub trait TypedSource<'a, T: Value + ?Sized + 'static>: Source<'a> {
    type Error: Into<Error> + fmt::Debug;

    fn stream_to_value(&mut self) -> Result<&T, ToValueError<Self::Error>>;

    fn stream_to_ref(&mut self) -> Result<&'a T, ToRefError<&T, Self::Error>> {
        Err(ToRefError::from_result(
            self.stream_to_value().map_err(|e| e.into_inner()),
        ))
    }

    // TODO: fn stream_to_owned when we figure out how to erase it (T::Owned + 'static -> Box<dyn Any> -> Box<T> -> T?)
}

impl<'a, 'b, T: Value + ?Sized + 'static, S: TypedSource<'a, T> + ?Sized> TypedSource<'a, T>
    for &'b mut S
{
    type Error = S::Error;

    fn stream_to_value(&mut self) -> Result<&T, ToValueError<Self::Error>> {
        (**self).stream_to_value()
    }

    fn stream_to_ref(&mut self) -> Result<&'a T, ToRefError<&T, Self::Error>> {
        (**self).stream_to_ref()
    }
}

impl<'a, T: Value + ?Sized> Source<'a> for &'a T {
    fn stream<'b, S: Stream<'b>>(&mut self, stream: S) -> Result
    where
        'a: 'b,
    {
        (**self).stream(stream)
    }
}

impl<'a, T: Value + ?Sized + 'static> TypedSource<'a, T> for &'a T {
    type Error = Impossible;

    fn stream_to_value(&mut self) -> Result<&T, ToValueError<Self::Error>> {
        Ok(self)
    }

    fn stream_to_ref(&mut self) -> Result<&'a T, ToRefError<&T, Self::Error>> {
        Ok(self)
    }
}
