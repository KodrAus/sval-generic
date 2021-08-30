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
    fn erased_i128(&mut self, v: i128) -> Result;
    fn erased_u128(&mut self, v: u128) -> Result;
    fn erased_str<'b, 'v>(&mut self, v: TypedValueRef<'v, 'b, str>) -> Result
    where
        'v: 'a;
    fn erased_map_begin(&mut self, len: Option<usize>) -> Result;
    fn erased_map_key_begin(&mut self) -> Result;
    fn erased_map_value_begin(&mut self) -> Result;
    fn erased_map_end(&mut self) -> Result;
    fn erased_map_key<'b, 'k>(&mut self, k: AnyValueRef<'k, 'b>) -> Result
    where
        'k: 'a;
    fn erased_map_value<'b, 'v>(&mut self, v: AnyValueRef<'v, 'b>) -> Result
    where
        'v: 'a;
    fn erased_map_entry<'b, 'k, 'v>(&mut self, k: AnyValueRef<'k, 'b>, v: AnyValueRef<'v, 'b>) -> Result
    where
        'k: 'a,
        'v: 'a;
    fn erased_map_field<'b, 'v>(
        &mut self,
        f: TypedValueRef<'static, 'b, str>,
        v: AnyValueRef<'v, 'b>,
    ) -> Result
    where
        'v: 'a;
}

impl<'a, T: ?Sized> ErasedStream<'a> for T
where
    T: stream::Stream<'a>,
{
    fn erased_i128(&mut self, v: i128) -> Result {
        self.i128(v)
    }

    fn erased_u128(&mut self, v: u128) -> Result {
        self.u128(v)
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

    fn erased_map_key<'b, 'k>(&mut self, k: AnyValueRef<'k, 'b>) -> Result
    where
        'k: 'a,
    {
        self.map_key(k)
    }

    fn erased_map_value<'b, 'v>(&mut self, v: AnyValueRef<'v, 'b>) -> Result
    where
        'v: 'a,
    {
        self.map_value(v)
    }

    fn erased_map_entry<'b, 'k, 'v>(&mut self, k: AnyValueRef<'k, 'b>, v: AnyValueRef<'v, 'b>) -> Result
    where
        'k: 'a,
        'v: 'a,
    {
        self.map_entry(k, v)
    }

    fn erased_map_field<'b, 'v>(
        &mut self,
        f: TypedValueRef<'static, 'b, str>,
        v: AnyValueRef<'v, 'b>,
    ) -> Result
    where
        'v: 'a,
    {
        self.map_field(f, v)
    }
}

impl<'a, 'b> stream::Stream<'a> for Stream<'a, 'b> {
    fn u128(&mut self, v: u128) -> Result {
        self.0.erased_u128(v)
    }

    fn i128(&mut self, v: i128) -> Result {
        self.0.erased_i128(v)
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

    fn map_key<'k, K: stream::AnyValueRef<'k>>(&mut self, k: K) -> Result
    where
        'k: 'a,
    {
        self.0.erased_map_key(AnyValueRef(&k, ()))
    }

    fn map_value<'v, V: stream::AnyValueRef<'v>>(&mut self, v: V) -> Result
    where
        'v: 'a,
    {
        self.0.erased_map_value(AnyValueRef(&v, ()))
    }

    fn map_entry<'k, 'v, K: stream::AnyValueRef<'k>, V: stream::AnyValueRef<'v>>(
        &mut self,
        k: K,
        v: V,
    ) -> Result
    where
        'k: 'a,
        'v: 'a,
    {
        self.0.erased_map_entry(AnyValueRef(&k, ()), AnyValueRef(&v, ()))
    }

    fn map_field<'v, F: stream::TypedValueRef<'static, str>, V: stream::AnyValueRef<'v>>(
        &mut self,
        f: F,
        v: V,
    ) -> Result
    where
        'v: 'a,
    {
        self.0.erased_map_field(TypedValueRef(&f), AnyValueRef(&v, ()))
    }
}

struct AnyValueRef<'a, 'b>(&'b dyn ErasedAnyValueRef<'a>, ());

impl<'a, 'b> Deref for AnyValueRef<'a, 'b> {
    type Target = ();

    fn deref(&self) -> &Self::Target {
        &self.1
    }
}

impl<'a, 'b> Clone for AnyValueRef<'a, 'b> {
    fn clone(&self) -> Self {
        AnyValueRef(self.0, ())
    }
}

impl<'a, 'b> Copy for AnyValueRef<'a, 'b> {}

struct AnyForAll<'a, 'b>(&'b dyn ErasedAnyValueRef<'a>, ());

impl<'a, 'b> Deref for AnyForAll<'a, 'b> {
    type Target = ();

    fn deref(&self) -> &Self::Target {
        &self.1
    }
}

impl<'a, 'b> Clone for AnyForAll<'a, 'b> {
    fn clone(&self) -> Self {
        AnyForAll(self.0, ())
    }
}

impl<'a, 'b> Copy for AnyForAll<'a, 'b> {}

trait ErasedAnyValueRef<'a> {
    fn erased_stream<'b>(&self, stream: Stream<'b, '_>) -> Result
    where
        'a: 'b;

    fn erased_stream_for_all<'b>(&self, stream: Stream<'b, '_>) -> Result;
}

impl<'a, T> ErasedAnyValueRef<'a> for T
where
    T: value_ref::AnyValueRef<'a>,
{
    fn erased_stream<'b>(&self, stream: Stream<'b, '_>) -> Result
    where
        'a: 'b,
    {
        self.stream(stream)
    }

    fn erased_stream_for_all<'b>(&self, stream: Stream<'b, '_>) -> Result {
        self.stream_for_all(stream)
    }
}

impl<'a, 'b> value_ref::AnyValueRef<'a> for AnyValueRef<'a, 'b> {
    fn stream<'c, S>(self, mut stream: S) -> Result
    where
        'a: 'c,
        S: stream::Stream<'c>,
    {
        self.0.erased_stream(Stream(&mut stream))
    }

    fn stream_for_all<'c, S>(self, mut stream: S) -> Result
    where
        S: stream::Stream<'c>,
    {
        self.0.erased_stream_for_all(Stream(&mut stream))
    }
}

impl<'a, 'b, 'c> value_ref::AnyValueRef<'c> for AnyForAll<'a, 'b> {
    fn stream<'d, S>(self, mut stream: S) -> Result
    where
        'c: 'd,
        S: stream::Stream<'d>,
    {
        self.0.erased_stream_for_all(Stream(&mut stream))
    }

    fn stream_for_all<'d, S>(self, mut stream: S) -> Result
    where
        S: stream::Stream<'d>,
    {
        self.0.erased_stream_for_all(Stream(&mut stream))
    }
}

struct TypedValueRef<'a, 'b, T: ?Sized>(&'b dyn ErasedTypedValueRef<'a, T>);

impl<'a, 'b, T: ?Sized> Clone for TypedValueRef<'a, 'b, T> {
    fn clone(&self) -> Self {
        TypedValueRef(self.0)
    }
}

impl<'a, 'b, T: ?Sized> Copy for TypedValueRef<'a, 'b, T> {}

struct TypedForAll<'a, 'b, T: ?Sized>(&'b dyn ErasedTypedValueRef<'a, T>);

impl<'a, 'b, T: ?Sized> Clone for TypedForAll<'a, 'b, T> {
    fn clone(&self) -> Self {
        TypedForAll(self.0)
    }
}

impl<'a, 'b, T: ?Sized> Copy for TypedForAll<'a, 'b, T> {}

impl<'a, 'b, T: ?Sized> Deref for TypedForAll<'a, 'b, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &**self.0
    }
}

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

    fn erased_stream_for_all<'b>(&self, stream: Stream<'b, '_>) -> Result;

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

    fn erased_stream_for_all<'b>(&self, stream: Stream<'b, '_>) -> Result {
        self.stream_for_all(stream)
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

impl<'a, 'b, T: ?Sized> value_ref::AnyValueRef<'a> for TypedValueRef<'a, 'b, T> {
    fn stream<'c, S>(self, mut stream: S) -> Result
    where
        'a: 'c,
        S: stream::Stream<'c>,
    {
        self.0.erased_stream(Stream(&mut stream))
    }

    fn stream_for_all<'c, S>(self, mut stream: S) -> Result
    where
        S: stream::Stream<'c>,
    {
        self.0.erased_stream_for_all(Stream(&mut stream))
    }
}

impl<'a, 'b, 'c, T: ?Sized> value_ref::TypedValueRef<'c, T> for TypedForAll<'a, 'b, T> {
    fn to_ref(self) -> Option<&'c T> {
        self.erased_to_ref()
    }
}

impl<'a, 'b, 'c, T: ?Sized> value_ref::AnyValueRef<'c> for TypedForAll<'a, 'b, T> {
    fn stream<'d, S>(self, mut stream: S) -> Result
    where
        'c: 'd,
        S: stream::Stream<'d>,
    {
        self.0.erased_stream_for_all(Stream(&mut stream))
    }

    fn stream_for_all<'d, S>(self, mut stream: S) -> Result
    where
        S: stream::Stream<'d>,
    {
        self.0.erased_stream_for_all(Stream(&mut stream))
    }
}
