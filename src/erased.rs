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
    fn erased_str<'b, 'v>(&mut self, v: TypedValue<'v, 'b, str>) -> Result
    where
        'v: 'a;
    fn erased_map_begin(&mut self, len: Option<usize>) -> Result;
    fn erased_map_key_begin(&mut self) -> Result;
    fn erased_map_value_begin(&mut self) -> Result;
    fn erased_map_end(&mut self) -> Result;
    fn erased_map_key<'b, 'k>(&mut self, k: UntypedValue<'k, 'b>) -> Result
    where
        'k: 'a;
    fn erased_map_value<'b, 'v>(&mut self, v: UntypedValue<'v, 'b>) -> Result
    where
        'v: 'a;
    fn erased_map_entry<'b, 'k, 'v>(
        &mut self,
        k: UntypedValue<'k, 'b>,
        v: UntypedValue<'v, 'b>,
    ) -> Result
    where
        'k: 'a,
        'v: 'a;
    fn erased_map_field<'b, 'v>(
        &mut self,
        f: TypedValue<'static, 'b, str>,
        v: UntypedValue<'v, 'b>,
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

    fn erased_str<'b, 'v>(&mut self, v: TypedValue<'v, 'b, str>) -> Result
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

    fn erased_map_key<'b, 'k>(&mut self, k: UntypedValue<'k, 'b>) -> Result
    where
        'k: 'a,
    {
        self.map_key(k)
    }

    fn erased_map_value<'b, 'v>(&mut self, v: UntypedValue<'v, 'b>) -> Result
    where
        'v: 'a,
    {
        self.map_value(v)
    }

    fn erased_map_entry<'b, 'k, 'v>(
        &mut self,
        k: UntypedValue<'k, 'b>,
        v: UntypedValue<'v, 'b>,
    ) -> Result
    where
        'k: 'a,
        'v: 'a,
    {
        self.map_entry(k, v)
    }

    fn erased_map_field<'b, 'v>(
        &mut self,
        f: TypedValue<'static, 'b, str>,
        v: UntypedValue<'v, 'b>,
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

    fn str<'v, V: stream::TypedValue<'v, str>>(&mut self, v: V) -> Result
    where
        'v: 'a,
    {
        self.0.erased_str(TypedValue(&v))
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

    fn map_key<'k, K: stream::UntypedValue<'k>>(&mut self, k: K) -> Result
    where
        'k: 'a,
    {
        self.0.erased_map_key(UntypedValue(&k))
    }

    fn map_value<'v, V: stream::UntypedValue<'v>>(&mut self, v: V) -> Result
    where
        'v: 'a,
    {
        self.0.erased_map_value(UntypedValue(&v))
    }

    fn map_entry<'k, 'v, K: stream::UntypedValue<'k>, V: stream::UntypedValue<'v>>(
        &mut self,
        k: K,
        v: V,
    ) -> Result
    where
        'k: 'a,
        'v: 'a,
    {
        self.0.erased_map_entry(UntypedValue(&k), UntypedValue(&v))
    }

    fn map_field<'v, F: stream::TypedValue<'static, str>, V: stream::UntypedValue<'v>>(
        &mut self,
        f: F,
        v: V,
    ) -> Result
    where
        'v: 'a,
    {
        self.0.erased_map_field(TypedValue(&f), UntypedValue(&v))
    }
}

struct UntypedValue<'a, 'b>(&'b dyn ErasedUntypedValue<'a>);

struct UntypedForAll<'a, 'b>(&'b dyn ErasedUntypedValue<'a>);

trait ErasedUntypedValue<'a> {
    fn erased_stream<'b>(&self, stream: Stream<'b, '_>) -> Result
    where
        'a: 'b;

    fn erased_stream_for_all<'b>(&self, stream: Stream<'b, '_>) -> Result;
}

impl<'a, T> ErasedUntypedValue<'a> for T
where
    T: value_ref::UntypedValue<'a>,
{
    fn erased_stream<'b>(&self, stream: Stream<'b, '_>) -> Result
    where
        'a: 'b,
    {
        self.stream(stream)
    }

    fn erased_stream_for_all<'b>(&self, stream: Stream<'b, '_>) -> Result {
        use value_ref::UntypedValue;

        self.for_all().stream(stream)
    }
}

impl<'a, 'b> value_ref::UntypedValue<'a> for UntypedValue<'a, 'b> {
    type ForAll = UntypedForAll<'a, 'b>;

    fn for_all(&self) -> UntypedForAll<'a, 'b> {
        UntypedForAll(self.0)
    }

    fn stream<'c, S>(&self, mut stream: S) -> Result
    where
        'a: 'c,
        S: stream::Stream<'c>,
    {
        self.0.erased_stream(Stream(&mut stream))
    }
}

impl<'a, 'b, 'c> value_ref::UntypedValue<'c> for UntypedForAll<'a, 'b> {
    type ForAll = Self;

    fn for_all(&self) -> Self {
        UntypedForAll(self.0)
    }

    fn stream<'d, S>(&self, mut stream: S) -> Result
    where
        'c: 'd,
        S: stream::Stream<'d>,
    {
        self.0.erased_stream_for_all(Stream(&mut stream))
    }
}

struct TypedValue<'a, 'b, T: ?Sized>(&'b dyn ErasedTypedValue<'a, T>);

struct TypedForAll<'a, 'b, T: ?Sized>(&'b dyn ErasedTypedValue<'a, T>);

impl<'a, 'b, T: ?Sized> Deref for TypedForAll<'a, 'b, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &**self.0
    }
}

impl<'a, 'b, T: ?Sized> Deref for TypedValue<'a, 'b, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &**self.0
    }
}

trait ErasedTypedValue<'a, T: ?Sized>: Deref<Target = T> {
    fn erased_base_stream<'b>(&self, stream: Stream<'b, '_>) -> Result
    where
        'a: 'b;

    fn erased_base_stream_for_all<'b>(&self, stream: Stream<'b, '_>) -> Result;

    fn erased_stream<'b>(&self, stream: Stream<'b, '_>) -> Result
    where
        'a: 'b;

    fn erased_stream_for_all<'b>(&self, stream: Stream<'b, '_>) -> Result;

    fn erased_to_ref(&self) -> Option<&'a T>;
}

impl<'a, T, U: ?Sized> ErasedTypedValue<'a, U> for T
where
    T: value_ref::TypedValue<'a, U>,
{
    fn erased_base_stream<'b>(&self, stream: Stream<'b, '_>) -> Result
    where
        'a: 'b,
    {
        use value_ref::UntypedValue;

        self.base().stream(stream)
    }

    fn erased_base_stream_for_all<'b>(&self, stream: Stream<'b, '_>) -> Result {
        use value_ref::UntypedValue;

        self.base().for_all().stream(stream)
    }

    fn erased_stream<'b>(&self, stream: Stream<'b, '_>) -> Result
    where
        'a: 'b,
    {
        self.stream(stream)
    }

    fn erased_stream_for_all<'b>(&self, stream: Stream<'b, '_>) -> Result {
        use value_ref::TypedValue;

        self.for_all().stream(stream)
    }

    fn erased_to_ref(&self) -> Option<&'a U> {
        self.to_ref()
    }
}

impl<'a, 'b, T: ?Sized> value_ref::TypedValue<'a, T> for TypedValue<'a, 'b, T> {
    type Base = Self;
    type ForAll = TypedForAll<'a, 'b, T>;

    fn base(&self) -> Self {
        TypedValue(self.0)
    }

    fn to_ref(&self) -> Option<&'a T> {
        self.0.erased_to_ref()
    }

    fn for_all(&self) -> TypedForAll<'a, 'b, T> {
        TypedForAll(self.0)
    }

    fn stream<'c, S>(&self, mut stream: S) -> Result
    where
        'a: 'c,
        S: stream::Stream<'c>,
    {
        self.0.erased_stream(Stream(&mut stream))
    }
}

impl<'a, 'b, 'c, T: ?Sized> value_ref::UntypedValue<'c> for TypedValue<'a, 'b, T> {
    type ForAll = TypedForAll<'a, 'b, T>;

    fn for_all(&self) -> TypedForAll<'a, 'b, T> {
        TypedForAll(self.0)
    }

    fn stream<'d, S>(&self, mut stream: S) -> Result
    where
        'c: 'd,
        S: stream::Stream<'d>,
    {
        self.0.erased_base_stream_for_all(Stream(&mut stream))
    }
}

impl<'a, 'b, 'c, T: ?Sized> value_ref::TypedValue<'c, T> for TypedForAll<'a, 'b, T> {
    type Base = Self;
    type ForAll = Self;

    fn base(&self) -> Self {
        TypedForAll(self.0)
    }

    fn to_ref(&self) -> Option<&'c T> {
        self.erased_to_ref()
    }

    fn for_all(&self) -> Self {
        TypedForAll(self.0)
    }

    fn stream<'d, S>(&self, mut stream: S) -> Result
    where
        'c: 'd,
        S: stream::Stream<'d>,
    {
        self.0.erased_stream_for_all(Stream(&mut stream))
    }
}

impl<'a, 'b, 'c, T: ?Sized> value_ref::UntypedValue<'c> for TypedForAll<'a, 'b, T> {
    type ForAll = Self;

    fn for_all(&self) -> Self {
        TypedForAll(self.0)
    }

    fn stream<'d, S>(&self, mut stream: S) -> Result
    where
        'c: 'd,
        S: stream::Stream<'d>,
    {
        self.0.erased_base_stream_for_all(Stream(&mut stream))
    }
}
