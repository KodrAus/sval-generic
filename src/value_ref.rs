use std::{fmt, ops::Deref};

use crate::{stream::Stream, value::Value, Error, Result};

// TODO: Remove the `ForAll` directly and unify these again
pub trait UntypedValue<'a> {
    type ForAll: for<'b> UntypedValue<'b>;

    fn for_all(&self) -> Self::ForAll;

    fn stream<'b, S>(&self, stream: S) -> Result
    where
        'a: 'b,
        S: Stream<'b>;
}

pub trait TypedValue<'a, T: ?Sized>: Deref<Target = T> {
    type Base: UntypedValue<'a, ForAll = Self::ForAll>;
    type ForAll: for<'b> TypedValue<'b, T> + for<'b> UntypedValue<'b>;

    fn base(&self) -> Self::Base;

    fn for_all(&self) -> Self::ForAll {
        self.base().for_all()
    }

    fn to_ref(&self) -> Option<&'a T>;

    fn stream<'b, S>(&self, stream: S) -> Result
    where
        'a: 'b,
        S: Stream<'b>,
    {
        self.base().stream(stream)
    }
}

impl<'a, T: ?Sized> UntypedValue<'a> for &'a T
where
    T: Value,
{
    type ForAll = ForAll<'a, T>;

    fn for_all(&self) -> ForAll<'a, T> {
        ForAll(*self)
    }

    fn stream<'b, S>(&self, stream: S) -> Result
    where
        'a: 'b,
        S: Stream<'b>,
    {
        (**self).stream(stream)
    }
}

impl<'a, T: ?Sized> TypedValue<'a, T> for &'a T
where
    T: Value,
{
    type Base = Self;
    type ForAll = ForAll<'a, T>;

    fn base(&self) -> Self {
        self
    }

    fn to_ref(&self) -> Option<&'a T> {
        Some(*self)
    }
}

pub struct ForAll<'a, T: ?Sized>(&'a T);

pub fn for_all<'a, T: ?Sized>(
    v: &'a T,
) -> impl for<'b> TypedValue<'b, T> + for<'b> UntypedValue<'b> + 'a
where
    T: Value,
{
    TypedValue::for_all(&v)
}

impl<'a, T: ?Sized> ForAll<'a, T>
where
    T: Value,
{
    pub fn new(v: &'a T) -> Self {
        ForAll(v)
    }
}

impl<'a, T: ?Sized> Deref for ForAll<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<'a, 'b, T: ?Sized> UntypedValue<'a> for ForAll<'b, T>
where
    T: Value,
{
    type ForAll = Self;

    fn for_all(&self) -> Self {
        ForAll(self.0)
    }

    fn stream<'c, S>(&self, stream: S) -> Result
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

            fn str<'v, V: TypedValue<'v, str>>(&mut self, v: V) -> Result
            where
                'v: 'a,
            {
                self.0.str(v.for_all())
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

            fn map_key<'k, K: UntypedValue<'k>>(&mut self, k: K) -> Result
            where
                'k: 'a,
            {
                self.0.map_key(k.for_all())
            }

            fn map_value<'v, V: UntypedValue<'v>>(&mut self, v: V) -> Result
            where
                'v: 'a,
            {
                self.0.map_value(v.for_all())
            }

            fn map_entry<'k, 'v, K: UntypedValue<'k>, V: UntypedValue<'v>>(
                &mut self,
                k: K,
                v: V,
            ) -> Result
            where
                'k: 'a,
                'v: 'a,
            {
                self.0.map_entry(k.for_all(), v.for_all())
            }

            fn map_field<'v, F: TypedValue<'static, str>, V: UntypedValue<'v>>(
                &mut self,
                f: F,
                v: V,
            ) -> Result
            where
                'v: 'a,
            {
                self.0.map_field(f.for_all(), v.for_all())
            }
        }

        self.0.stream(AnyStream(stream))
    }
}

impl<'a, 'b, T: ?Sized> TypedValue<'a, T> for ForAll<'b, T>
where
    T: Value,
{
    type Base = Self;
    type ForAll = Self;

    fn base(&self) -> Self {
        ForAll(self.0)
    }

    fn to_ref(&self) -> Option<&'a T> {
        None
    }
}
