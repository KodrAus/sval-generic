use std::ops::Deref;

use crate::{stream::Stream, value::Value, Result};

pub trait UntypedValue<'a>: Copy {
    fn stream<'b, S>(&self, stream: S) -> Result
    where
        'a: 'b,
        S: Stream<'b>;
}

pub trait TypedValue<'a, T: ?Sized>: UntypedValue<'a> + Deref<Target = T> {
    fn to_ref(&self) -> Option<&'a T>;
}

impl<'a, T: ?Sized> UntypedValue<'a> for &'a T
where
    T: Value,
{
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
    fn to_ref(&self) -> Option<&'a T> {
        Some(*self)
    }
}

#[derive(Clone, Copy)]
pub struct ForAll<T>(pub T);

impl<T: Deref> Deref for ForAll<T> {
    type Target = T::Target;

    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

impl<T> Value for ForAll<T>
where
    T: Value,
{
    fn stream<'a, S>(&'a self, stream: S) -> Result
    where
        S: Stream<'a>,
    {
        self.0.stream(stream)
    }
}

impl<'a, 'b, T> UntypedValue<'a> for ForAll<T>
where
    T: UntypedValue<'b>,
{
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
                self.0.str(ForAll(v))
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
                self.0.map_key(ForAll(k))
            }

            fn map_value<'v, V: UntypedValue<'v>>(&mut self, v: V) -> Result
            where
                'v: 'a,
            {
                self.0.map_value(ForAll(v))
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
                self.0.map_entry(ForAll(k), ForAll(v))
            }

            fn map_field<'v, F: TypedValue<'static, str>, V: UntypedValue<'v>>(
                &mut self,
                f: F,
                v: V,
            ) -> Result
            where
                'v: 'a,
            {
                self.0.map_field(f, ForAll(v))
            }
        }

        self.0.stream(AnyStream(stream))
    }
}

impl<'a, 'b, T, U: ?Sized> TypedValue<'a, U> for ForAll<T>
where
    T: TypedValue<'b, U>,
{
    fn to_ref(&self) -> Option<&'a U> {
        None
    }
}
