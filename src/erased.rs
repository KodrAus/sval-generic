use std::{error, fmt};

use crate::{stream, value, Result};

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
    fn erased_display(&mut self, v: &dyn fmt::Display) -> Result;
    fn erased_error<'b, 'v>(&mut self, e: Ref<'v, 'b, dyn error::Error + 'static>) -> Result
    where
        'v: 'a;
    fn erased_str<'b, 'v>(&mut self, v: Ref<'v, 'b, str>) -> Result
    where
        'v: 'a;
    fn erased_map_begin(&mut self, len: Option<usize>) -> Result;
    fn erased_map_key_begin(&mut self) -> Result;
    fn erased_map_value_begin(&mut self) -> Result;
    fn erased_map_end(&mut self) -> Result;
    fn erased_map_key<'b, 'k>(&mut self, k: ValueRef<'k, 'b>) -> Result
    where
        'k: 'a;
    fn erased_map_value<'b, 'v>(&mut self, v: ValueRef<'v, 'b>) -> Result
    where
        'v: 'a;
    fn erased_map_entry<'b, 'k, 'v>(&mut self, k: ValueRef<'k, 'b>, v: ValueRef<'v, 'b>) -> Result
    where
        'k: 'a,
        'v: 'a;
    fn erased_map_field<'b, 'v>(&mut self, f: Ref<'static, 'b, str>, v: ValueRef<'v, 'b>) -> Result
    where
        'v: 'a;
    fn erased_seq_begin(&mut self, len: Option<usize>) -> Result;
    fn erased_seq_elem_begin(&mut self) -> Result;
    fn erased_seq_end(&mut self) -> Result;
    fn erased_seq_elem<'b, 'e>(&mut self, e: ValueRef<'e, 'b>) -> Result
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

    fn erased_display(&mut self, v: &dyn fmt::Display) -> Result {
        self.display(v)
    }

    fn erased_error<'b, 'v>(&mut self, e: Ref<'v, 'b, dyn error::Error + 'static>) -> Result
    where
        'v: 'a,
    {
        self.error(e)
    }

    fn erased_str<'b, 'v>(&mut self, v: Ref<'v, 'b, str>) -> Result
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

    fn erased_map_key<'b, 'k>(&mut self, k: ValueRef<'k, 'b>) -> Result
    where
        'k: 'a,
    {
        self.map_key(k)
    }

    fn erased_map_value<'b, 'v>(&mut self, v: ValueRef<'v, 'b>) -> Result
    where
        'v: 'a,
    {
        self.map_value(v)
    }

    fn erased_map_entry<'b, 'k, 'v>(&mut self, k: ValueRef<'k, 'b>, v: ValueRef<'v, 'b>) -> Result
    where
        'k: 'a,
        'v: 'a,
    {
        self.map_entry(k, v)
    }

    fn erased_map_field<'b, 'v>(&mut self, f: Ref<'static, 'b, str>, v: ValueRef<'v, 'b>) -> Result
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

    fn erased_seq_elem<'b, 'e>(&mut self, e: ValueRef<'e, 'b>) -> Result
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

    fn display<V: fmt::Display>(&mut self, d: V) -> Result {
        self.0.erased_display(&d)
    }

    fn error<'v, V: stream::Ref<'v, dyn error::Error + 'static>>(&mut self, e: V) -> Result
    where
        'v: 'a,
    {
        self.0.erased_error(Ref(&e))
    }

    fn str<'v, V: stream::Ref<'v, str>>(&mut self, v: V) -> Result
    where
        'v: 'a,
    {
        self.0.erased_str(Ref(&v))
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

    fn map_key<'k, K: stream::ValueRef<'k>>(&mut self, k: K) -> Result
    where
        'k: 'a,
    {
        self.0.erased_map_key(ValueRef(&k))
    }

    fn map_value<'v, V: stream::ValueRef<'v>>(&mut self, v: V) -> Result
    where
        'v: 'a,
    {
        self.0.erased_map_value(ValueRef(&v))
    }

    fn map_entry<'k, 'v, K: stream::ValueRef<'k>, V: stream::ValueRef<'v>>(
        &mut self,
        k: K,
        v: V,
    ) -> Result
    where
        'k: 'a,
        'v: 'a,
    {
        self.0.erased_map_entry(ValueRef(&k), ValueRef(&v))
    }

    fn map_field<'v, F: stream::Ref<'static, str>, V: stream::ValueRef<'v>>(
        &mut self,
        f: F,
        v: V,
    ) -> Result
    where
        'v: 'a,
    {
        self.0.erased_map_field(Ref(&f), ValueRef(&v))
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

    fn seq_elem<'e, E: stream::ValueRef<'e>>(&mut self, e: E) -> Result
    where
        'e: 'a,
    {
        self.0.erased_seq_elem(ValueRef(&e))
    }
}

struct ValueRef<'a, 'b>(&'b dyn ErasedValueRef<'a>);

impl<'a, 'b> Clone for ValueRef<'a, 'b> {
    fn clone(&self) -> Self {
        ValueRef(self.0)
    }
}

impl<'a, 'b> Copy for ValueRef<'a, 'b> {}

trait ErasedValueRef<'a> {
    fn erased_stream<'b>(&self, stream: Stream<'b, '_>) -> Result
    where
        'a: 'b;
}

impl<'a, T> ErasedValueRef<'a> for T
where
    T: stream::ValueRef<'a>,
{
    fn erased_stream<'b>(&self, stream: Stream<'b, '_>) -> Result
    where
        'a: 'b,
    {
        stream::ValueRef::stream(*self, stream)
    }
}

impl<'a, 'b> stream::ValueRef<'a> for ValueRef<'a, 'b> {
    fn stream<'c, S>(self, mut stream: S) -> Result
    where
        'a: 'c,
        S: stream::Stream<'c>,
    {
        self.0.erased_stream(Stream(&mut stream))
    }
}

impl<'a, 'b> value::Value for ValueRef<'a, 'b> {
    fn stream<'c, S>(&'c self, mut stream: S) -> Result
    where
        S: stream::Stream<'c>,
    {
        self.0.erased_stream(Stream(&mut stream))
    }
}

struct Ref<'a, 'b, T: ?Sized>(&'b dyn ErasedRef<'a, T>);

impl<'a, 'b, T: ?Sized> Clone for Ref<'a, 'b, T> {
    fn clone(&self) -> Self {
        Ref(self.0)
    }
}

impl<'a, 'b, T: ?Sized> Copy for Ref<'a, 'b, T> {}

trait ErasedRef<'a, T: ?Sized> {
    fn erased_stream<'b>(&self, stream: Stream<'b, '_>) -> Result
    where
        'a: 'b;

    fn erased_get(&self) -> &T;
    fn erased_try_unwrap(&self) -> Option<&'a T>;
}

impl<'a, T, U: ?Sized> ErasedRef<'a, U> for T
where
    T: stream::Ref<'a, U>,
    U: value::Value + 'static,
{
    fn erased_stream<'b>(&self, stream: Stream<'b, '_>) -> Result
    where
        'a: 'b,
    {
        stream::ValueRef::stream(*self, stream)
    }

    fn erased_get(&self) -> &U {
        self.get()
    }

    fn erased_try_unwrap(&self) -> Option<&'a U> {
        self.try_unwrap()
    }
}

impl<'a, 'b, T: ?Sized> stream::Ref<'a, T> for Ref<'a, 'b, T>
where
    T: value::Value + 'static,
{
    fn get(&self) -> &T {
        self.0.erased_get()
    }

    fn try_unwrap(self) -> Option<&'a T> {
        self.0.erased_try_unwrap()
    }
}

impl<'a, 'b, T: ?Sized> stream::ValueRef<'a> for Ref<'a, 'b, T> {
    fn stream<'c, S>(self, mut stream: S) -> Result
    where
        'a: 'c,
        S: stream::Stream<'c>,
    {
        self.0.erased_stream(Stream(&mut stream))
    }
}

impl<'a, 'b, T: ?Sized> value::Value for Ref<'a, 'b, T> {
    fn stream<'c, S>(&'c self, mut stream: S) -> Result
    where
        S: stream::Stream<'c>,
    {
        self.0.erased_stream(Stream(&mut stream))
    }
}
