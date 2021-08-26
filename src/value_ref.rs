use std::{fmt, ops::Deref};

use crate::{stream::Stream, value::Value, Error, Result};

// TODO: This is only worthwhile if it makes it easier to erase

pub trait UntypedValue<'a> {
    type ForAll: for<'b> UntypedValue<'b>;

    fn for_all(&self) -> Self::ForAll;

    fn stream<'b, S>(&self, stream: S) -> Result
    where
        'a: 'b,
        S: Stream<'b>;
}

pub trait TypedValue<'a, T: ?Sized>: Deref<Target = T> {
    type UntypedValue: UntypedValue<'a, ForAll = Self::ForAll>;
    type ForAll: for<'b> TypedValue<'b, T> + for<'b> UntypedValue<'b>;

    fn untype(&self) -> Self::UntypedValue;

    fn to_ref(&self) -> Option<&'a T>;

    fn for_all(&self) -> Self::ForAll {
        self.untype().for_all()
    }

    fn stream<'b, S>(&self, stream: S) -> Result
    where
        'a: 'b,
        S: Stream<'b>,
    {
        self.untype().stream(stream)
    }
}

impl<'a, T: ?Sized> UntypedValue<'a> for &'a T
where
    T: Value,
{
    type ForAll = AnyRef<'a, T>;

    fn for_all(&self) -> AnyRef<'a, T> {
        AnyRef(*self)
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
    type UntypedValue = Self;
    type ForAll = AnyRef<'a, T>;

    fn untype(&self) -> Self::UntypedValue {
        self
    }

    fn to_ref(&self) -> Option<&'a T> {
        Some(*self)
    }
}

pub struct AnyRef<'a, T: ?Sized>(pub &'a T);

impl<'a, T: ?Sized> Deref for AnyRef<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<'a, 'b, T: ?Sized> UntypedValue<'a> for AnyRef<'b, T>
where
    T: Value,
{
    type ForAll = Self;

    fn for_all(&self) -> Self {
        AnyRef(self.0)
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

            fn str<V: TypedValue<'a, str>>(&mut self, v: V) -> Result {
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

            fn map_key<K: UntypedValue<'a>>(&mut self, k: K) -> Result {
                self.0.map_key(k.for_all())
            }

            fn map_value<V: UntypedValue<'a>>(&mut self, v: V) -> Result {
                self.0.map_value(v.for_all())
            }

            fn map_entry<K: UntypedValue<'a>, V: UntypedValue<'a>>(
                &mut self,
                k: K,
                v: V,
            ) -> Result {
                self.0.map_entry(k.for_all(), v.for_all())
            }

            fn map_field<F: TypedValue<'static, str>, V: UntypedValue<'a>>(
                &mut self,
                f: F,
                v: V,
            ) -> Result {
                self.0.map_field(f.for_all(), v.for_all())
            }
        }

        self.0.stream(AnyStream(stream))
    }
}

impl<'a, 'b, T: ?Sized> TypedValue<'a, T> for AnyRef<'b, T>
where
    T: Value,
{
    type UntypedValue = Self;
    type ForAll = Self;

    fn untype(&self) -> Self::UntypedValue {
        AnyRef(self.0)
    }

    fn to_ref(&self) -> Option<&'a T> {
        None
    }
}

pub fn for_all<'a, T: ?Sized>(
    v: &'a T,
) -> impl for<'b> TypedValue<'b, T> + for<'b> UntypedValue<'b> + 'a
where
    T: Value,
{
    TypedValue::for_all(&v)
}
