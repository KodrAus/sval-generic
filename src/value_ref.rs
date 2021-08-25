use std::{
    fmt,
    ops::Deref,
};

use crate::{stream::Stream, value::Value, Error, Result};

pub trait ValueRef<'a>: Deref {
    type AnyRef: for<'b> ValueRef<'b, Target = Self::Target>;

    fn stream_ref<'b, S>(self, stream: S) -> Result
    where
        'a: 'b,
        S: Stream<'b>;

    fn try_into_ref(self) -> Result<&'a Self::Target, IntoRefError<Self>>
    where
        Self: Sized,
    {
        Err(IntoRefError(self))
    }

    fn any_ref(self) -> Self::AnyRef;
}

pub struct IntoRefError<T>(pub T);

impl<T> fmt::Debug for IntoRefError<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("IntoRefError").finish()
    }
}

impl<T> IntoRefError<T> {
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> From<IntoRefError<T>> for Error {
    fn from(_: IntoRefError<T>) -> Error {
        Error
    }
}

impl<'a, T: ?Sized> ValueRef<'a> for &'a T
where
    T: Value,
{
    type AnyRef = AnyRef<'a, T>;

    fn stream_ref<'b, S>(self, stream: S) -> Result
    where
        'a: 'b,
        S: Stream<'b>,
    {
        (*self).stream(stream)
    }

    fn try_into_ref(self) -> Result<&'a Self::Target, IntoRefError<Self>>
    where
        Self: Sized,
    {
        Ok(self)
    }

    fn any_ref(self) -> AnyRef<'a, Self::Target> {
        AnyRef(self)
    }
}

pub struct AnyRef<'a, T: ?Sized>(&'a T);

impl<'a, T: ?Sized> Deref for AnyRef<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<'a, 'b, T: ?Sized> ValueRef<'a> for AnyRef<'b, T>
where
    T: Value,
{
    type AnyRef = Self;

    fn stream_ref<'c, S>(self, stream: S) -> Result
    where
        'a: 'c,
        S: Stream<'c>,
    {
        struct AnyStream<S>(S);

        impl<'a, 'b, S> Stream<'a> for AnyStream<S>
        where
            S: Stream<'b>,
        {
            fn u128(&mut self, v: u128) -> Result {
                self.0.u128(v)
            }

            fn i128(&mut self, v: i128) -> Result {
                self.0.i128(v)
            }

            fn map_begin(&mut self, len: Option<usize>) -> Result {
                self.0.map_begin(len)
            }

            fn map_key_begin(&mut self) -> Result {
                self.0.map_key_begin()
            }

            fn map_value_begin(&mut self) -> Result {
                self.0.map_value_begin()
            }

            fn map_end(&mut self) -> Result {
                self.0.map_end()
            }

            fn str<V: ValueRef<'a, Target = str>>(&mut self, v: V) -> Result {
                self.0.str(v.any_ref())
            }
        }

        self.0.stream(AnyStream(stream))
    }

    fn try_into_ref(self) -> Result<&'a Self::Target, IntoRefError<Self>>
    where
        Self: Sized,
    {
        Err(IntoRefError(self))
    }

    fn any_ref(self) -> AnyRef<'b, Self::Target> {
        self
    }
}

pub fn any_ref<'a, T: ?Sized>(v: &'a T) -> impl for<'b> ValueRef<'b, Target = T> + 'a
where
    T: Value,
{
    v.any_ref()
}
