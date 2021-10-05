use std::fmt;

use crate::{erased, Error, ForAll, Receiver, Result, Value};

pub trait Source<'a> {
    fn stream<'b, R: Receiver<'b>>(&mut self, receiver: R) -> Result
    where
        'a: 'b;

    fn for_all(&mut self) -> ForAll<&mut Self> {
        ForAll(self)
    }

    fn erase<'b>(&'b mut self) -> erased::Source<'a, 'b>
    where
        Self: Sized,
    {
        erased::Source::new(self)
    }
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

pub trait ValueSource<'a, T: Value + ?Sized + 'static>: Source<'a> {
    type Error: Into<Error> + fmt::Debug;

    fn value(&mut self) -> Result<&T, ToValueError<Self::Error>>;

    fn value_ref(&mut self) -> Result<&'a T, ToRefError<&T, Self::Error>> {
        Err(ToRefError::from_result(
            self.value().map_err(|e| e.into_inner()),
        ))
    }

    fn for_all_typed(&mut self) -> ForAll<&mut Self> {
        ForAll(self)
    }

    fn erase_typed<'b>(&'b mut self) -> erased::ValueSource<'a, 'b, T>
    where
        Self: Sized,
    {
        erased::ValueSource::new(self)
    }

    // TODO: fn stream_to_owned when we figure out how to erase it (T::Owned + 'static -> Box<dyn Any> -> Box<T> -> T?)
}

impl<'a, 'b, T: Value + ?Sized + 'static, S: ValueSource<'a, T> + ?Sized> ValueSource<'a, T>
    for &'b mut S
{
    type Error = S::Error;

    fn value(&mut self) -> Result<&T, ToValueError<Self::Error>> {
        (**self).value()
    }

    fn value_ref(&mut self) -> Result<&'a T, ToRefError<&T, Self::Error>> {
        (**self).value_ref()
    }
}

impl<'a, T: Value + ?Sized> Source<'a> for &'a T {
    fn stream<'b, R: Receiver<'b>>(&mut self, receiver: R) -> Result
    where
        'a: 'b,
    {
        (**self).stream(receiver)
    }
}

impl<'a, T: Value + ?Sized + 'static> ValueSource<'a, T> for &'a T {
    type Error = Impossible;

    fn value(&mut self) -> Result<&T, ToValueError<Self::Error>> {
        Ok(self)
    }

    fn value_ref(&mut self) -> Result<&'a T, ToRefError<&T, Self::Error>> {
        Ok(self)
    }
}
