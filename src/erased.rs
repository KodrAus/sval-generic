use std::ops::Deref;

use crate::{stream, value, value_ref, Result};

pub struct Value<'a>(&'a dyn ErasedValue);

impl<'a> Value<'a> {
    pub fn new(v: &'a impl value::Value) -> Self {
        Value(v)
    }
}

trait ErasedValue {
    fn erased_stream<'a>(&'a self, stream: Stream<'a, '_>) -> Result;
}

impl<T: ?Sized> ErasedValue for T
where
    T: value::Value,
{
    fn erased_stream<'a>(&'a self, stream: Stream<'a, '_>) -> Result {
        self.stream(stream)
    }
}

impl<'a> value::Value for Value<'a> {
    fn stream<'b, S>(&'b self, mut stream: S) -> Result
    where
        S: stream::Stream<'b>,
    {
        self.0.erased_stream(Stream(&mut stream))
    }
}

pub struct Stream<'a, 'b>(&'b mut dyn ErasedStream<'a>);

impl<'a, 'b> Stream<'a, 'b> {
    pub fn new(s: &'b mut impl stream::Stream<'a>) -> Self {
        Stream(s)
    }
}

trait ErasedStream<'a> {
    fn erased_u64(&mut self, v: u64) -> Result;
    fn erased_i64(&mut self, v: i64) -> Result;
    fn erased_i128(&mut self, v: i128) -> Result;
    fn erased_u128(&mut self, v: u128) -> Result;
    fn erased_f64(&mut self, v: f64) -> Result;
    fn erased_bool(&mut self, v: bool) -> Result;
    fn erased_none(&mut self) -> Result;
    fn erased_str<'b, 'v>(&mut self, v: TypedValueRef<'v, 'b, str>) -> Result
    where
        'v: 'a;
    fn erased_map_begin(&mut self, len: Option<usize>) -> Result;
    fn erased_map_key_begin(&mut self) -> Result;
    fn erased_map_value_begin(&mut self) -> Result;
    fn erased_map_end(&mut self) -> Result;
    fn erased_map_key<'b, 'k>(&mut self, k: UnknownValueRef<'k, 'b>) -> Result
    where
        'k: 'a;
    fn erased_map_value<'b, 'v>(&mut self, v: UnknownValueRef<'v, 'b>) -> Result
    where
        'v: 'a;
    fn erased_map_entry<'b, 'k, 'v>(
        &mut self,
        k: UnknownValueRef<'k, 'b>,
        v: UnknownValueRef<'v, 'b>,
    ) -> Result
    where
        'k: 'a,
        'v: 'a;
    fn erased_map_field<'b, 'v>(
        &mut self,
        f: TypedValueRef<'static, 'b, str>,
        v: UnknownValueRef<'v, 'b>,
    ) -> Result
    where
        'v: 'a;
    fn erased_seq_begin(&mut self, len: Option<usize>) -> Result;
    fn erased_seq_elem_begin(&mut self) -> Result;
    fn erased_seq_end(&mut self) -> Result;
    fn erased_seq_elem<'b, 'e>(&mut self, e: UnknownValueRef<'e, 'b>) -> Result
    where
        'e: 'a;
}

impl<'a, T: ?Sized> ErasedStream<'a> for T
where
    T: stream::Stream<'a>,
{
    fn erased_u64(&mut self, v: u64) -> Result {
        self.u64(v)
    }

    fn erased_i64(&mut self, v: i64) -> Result {
        self.i64(v)
    }

    fn erased_i128(&mut self, v: i128) -> Result {
        self.i128(v)
    }

    fn erased_u128(&mut self, v: u128) -> Result {
        self.u128(v)
    }

    fn erased_f64(&mut self, v: f64) -> Result {
        self.f64(v)
    }

    fn erased_bool(&mut self, v: bool) -> Result {
        self.bool(v)
    }

    fn erased_none(&mut self) -> Result {
        self.none()
    }

    fn erased_str<'b, 'v>(&mut self, v: TypedValueRef<'v, 'b, str>) -> Result
    where
        'v: 'a,
    {
        self.str(v)
    }

    fn erased_map_begin(&mut self, len: Option<usize>) -> Result {
        self.map_begin(len)
    }

    fn erased_map_key_begin(&mut self) -> Result {
        self.map_key_begin()
    }

    fn erased_map_value_begin(&mut self) -> Result {
        self.map_value_begin()
    }

    fn erased_map_end(&mut self) -> Result {
        self.map_end()
    }

    fn erased_map_key<'b, 'k>(&mut self, k: UnknownValueRef<'k, 'b>) -> Result
    where
        'k: 'a,
    {
        self.map_key(k)
    }

    fn erased_map_value<'b, 'v>(&mut self, v: UnknownValueRef<'v, 'b>) -> Result
    where
        'v: 'a,
    {
        self.map_value(v)
    }

    fn erased_map_entry<'b, 'k, 'v>(
        &mut self,
        k: UnknownValueRef<'k, 'b>,
        v: UnknownValueRef<'v, 'b>,
    ) -> Result
    where
        'k: 'a,
        'v: 'a,
    {
        self.map_entry(k, v)
    }

    fn erased_map_field<'b, 'v>(
        &mut self,
        f: TypedValueRef<'static, 'b, str>,
        v: UnknownValueRef<'v, 'b>,
    ) -> Result
    where
        'v: 'a,
    {
        self.map_field(f, v)
    }

    fn erased_seq_begin(&mut self, len: Option<usize>) -> Result {
        self.seq_begin(len)
    }

    fn erased_seq_elem_begin(&mut self) -> Result {
        self.seq_elem_begin()
    }

    fn erased_seq_end(&mut self) -> Result {
        self.seq_end()
    }

    fn erased_seq_elem<'b, 'e>(&mut self, e: UnknownValueRef<'e, 'b>) -> Result
    where
        'e: 'a,
    {
        self.seq_elem(e)
    }
}

impl<'a, 'b> stream::Stream<'a> for Stream<'a, 'b> {
    fn u64(&mut self, v: u64) -> Result {
        self.0.erased_u64(v)
    }

    fn i64(&mut self, v: i64) -> Result {
        self.0.erased_i64(v)
    }

    fn u128(&mut self, v: u128) -> Result {
        self.0.erased_u128(v)
    }

    fn i128(&mut self, v: i128) -> Result {
        self.0.erased_i128(v)
    }

    fn f64(&mut self, v: f64) -> Result {
        self.0.erased_f64(v)
    }

    fn bool(&mut self, v: bool) -> Result {
        self.0.erased_bool(v)
    }

    fn none(&mut self) -> Result {
        self.0.erased_none()
    }

    fn str<'v, V: stream::TypedValueRef<'v, str>>(&mut self, v: V) -> Result
    where
        'v: 'a,
    {
        self.0.erased_str(TypedValueRef(&v))
    }

    fn map_begin(&mut self, len: Option<usize>) -> Result {
        self.0.erased_map_begin(len)
    }

    fn map_key_begin(&mut self) -> Result {
        self.0.erased_map_key_begin()
    }

    fn map_value_begin(&mut self) -> Result {
        self.0.erased_map_value_begin()
    }

    fn map_end(&mut self) -> Result {
        self.0.erased_map_end()
    }

    fn map_key<'k, K: stream::UnknownValueRef<'k>>(&mut self, k: K) -> Result
    where
        'k: 'a,
    {
        self.0.erased_map_key(UnknownValueRef(&k, ()))
    }

    fn map_value<'v, V: stream::UnknownValueRef<'v>>(&mut self, v: V) -> Result
    where
        'v: 'a,
    {
        self.0.erased_map_value(UnknownValueRef(&v, ()))
    }

    fn map_entry<'k, 'v, K: stream::UnknownValueRef<'k>, V: stream::UnknownValueRef<'v>>(
        &mut self,
        k: K,
        v: V,
    ) -> Result
    where
        'k: 'a,
        'v: 'a,
    {
        self.0
            .erased_map_entry(UnknownValueRef(&k, ()), UnknownValueRef(&v, ()))
    }

    fn map_field<'v, F: stream::TypedValueRef<'static, str>, V: stream::UnknownValueRef<'v>>(
        &mut self,
        f: F,
        v: V,
    ) -> Result
    where
        'v: 'a,
    {
        self.0
            .erased_map_field(TypedValueRef(&f), UnknownValueRef(&v, ()))
    }

    fn seq_begin(&mut self, len: Option<usize>) -> Result {
        self.0.erased_seq_begin(len)
    }

    fn seq_elem_begin(&mut self) -> Result {
        self.0.erased_seq_elem_begin()
    }

    fn seq_end(&mut self) -> Result {
        self.0.erased_seq_end()
    }

    fn seq_elem<'e, E: stream::UnknownValueRef<'e>>(&mut self, e: E) -> Result
    where
        'e: 'a,
    {
        self.0.erased_seq_elem(UnknownValueRef(&e, ()))
    }
}

struct UnknownValueRef<'a, 'b>(&'b dyn ErasedUnknownValueRef<'a>, ());

impl<'a, 'b> Deref for UnknownValueRef<'a, 'b> {
    type Target = ();

    fn deref(&self) -> &Self::Target {
        &self.1
    }
}

impl<'a, 'b> Clone for UnknownValueRef<'a, 'b> {
    fn clone(&self) -> Self {
        UnknownValueRef(self.0, ())
    }
}

impl<'a, 'b> Copy for UnknownValueRef<'a, 'b> {}

trait ErasedUnknownValueRef<'a> {
    fn erased_stream<'b>(&self, stream: Stream<'b, '_>) -> Result
    where
        'a: 'b;
}

impl<'a, T> ErasedUnknownValueRef<'a> for T
where
    T: value_ref::UnknownValueRef<'a>,
{
    fn erased_stream<'b>(&self, stream: Stream<'b, '_>) -> Result
    where
        'a: 'b,
    {
        self.stream(stream)
    }
}

impl<'a, 'b> value_ref::UnknownValueRef<'a> for UnknownValueRef<'a, 'b> {
    fn stream<'c, S>(self, mut stream: S) -> Result
    where
        'a: 'c,
        S: stream::Stream<'c>,
    {
        self.0.erased_stream(Stream(&mut stream))
    }
}

struct TypedValueRef<'a, 'b, T: ?Sized>(&'b dyn ErasedTypedValueRef<'a, T>);

impl<'a, 'b, T: ?Sized> Clone for TypedValueRef<'a, 'b, T> {
    fn clone(&self) -> Self {
        TypedValueRef(self.0)
    }
}

impl<'a, 'b, T: ?Sized> Copy for TypedValueRef<'a, 'b, T> {}

impl<'a, 'b, T: ?Sized> Deref for TypedValueRef<'a, 'b, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &**self.0
    }
}

trait ErasedTypedValueRef<'a, T: ?Sized>: Deref<Target = T> {
    fn erased_stream<'b>(&self, stream: Stream<'b, '_>) -> Result
    where
        'a: 'b;

    fn erased_to_ref(&self) -> Option<&'a T>;
}

impl<'a, T, U: ?Sized> ErasedTypedValueRef<'a, U> for T
where
    T: value_ref::TypedValueRef<'a, U>,
{
    fn erased_stream<'b>(&self, stream: Stream<'b, '_>) -> Result
    where
        'a: 'b,
    {
        self.stream(stream)
    }

    fn erased_to_ref(&self) -> Option<&'a U> {
        self.to_ref()
    }
}

impl<'a, 'b, T: ?Sized> value_ref::TypedValueRef<'a, T> for TypedValueRef<'a, 'b, T> {
    fn to_ref(self) -> Option<&'a T> {
        self.0.erased_to_ref()
    }
}

impl<'a, 'b, T: ?Sized> value_ref::UnknownValueRef<'a> for TypedValueRef<'a, 'b, T> {
    fn stream<'c, S>(self, mut stream: S) -> Result
    where
        'a: 'c,
        S: stream::Stream<'c>,
    {
        self.0.erased_stream(Stream(&mut stream))
    }
}
