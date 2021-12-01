use crate::{std::fmt, Error, Receiver, Result, Value};

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

#[must_use]
pub enum Stream {
    Yield,
    Done,
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

// Implementing `ValueSource` directly:
// - When a type is useful to pass by value, such as numbers, or strings/vecs that can
//   be converted into their owned variants without any copies
// - When a type has a direct conversion to another, particularly the `data` types
//   like `Bytes` and `Digits`
pub trait ValueSource<'a, T: Value + ?Sized, R: Value + ?Sized = T>: Source<'a> {
    type Error: Into<Error> + fmt::Debug + fmt::Display;

    fn take(&mut self) -> Result<&T, TakeError<Self::Error>>;

    #[inline]
    fn take_ref(&mut self) -> Result<&'a R, TakeRefError<&T, Self::Error>> {
        Err(TakeRefError::from_value(self.take()?))
    }

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
    fn take_ref(&mut self) -> Result<&'a R, TakeRefError<&T, Self::Error>> {
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
        #[cfg(not(feature = "std"))]
        {
            unreachable!()
        }
        #[cfg(feature = "std")]
        {
            Ok(self.to_owned())
        }
    }
}

#[derive(Clone, Copy)]
pub enum Impossible {}

impl fmt::Debug for Impossible {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unreachable!()
    }
}

impl fmt::Display for Impossible {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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
    fn stream<'b, R: Receiver<'b>>(&mut self, _: R) -> Result<Stream>
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
