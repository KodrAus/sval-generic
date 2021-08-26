use std::ops::Deref;

use crate::{stream, value, value_ref, Result};

// TODO: Consider hiding the Value and Stream traits too
pub trait Value {
    fn erased_stream<'a>(&'a self, stream: &mut dyn Stream<'a>) -> Result;
}

impl<T: ?Sized> Value for T
where
    T: value::Value,
{
    fn erased_stream<'a>(&'a self, stream: &mut dyn Stream<'a>) -> Result {
        self.stream(stream)
    }
}

impl<'a> value::Value for (dyn Value + 'a) {
    fn stream<'b, S>(&'b self, mut stream: S) -> Result
    where
        S: stream::Stream<'b>,
    {
        self.erased_stream(&mut stream)
    }
}

pub trait Stream<'a> {
    fn erased_i128(&mut self, v: i128) -> Result;
    fn erased_u128(&mut self, v: u128) -> Result;
    fn erased_str<'b>(&mut self, v: TypedValueRef<'a, 'b, str>) -> Result;
    fn erased_map_begin(&mut self, len: Option<usize>) -> Result;
    fn erased_map_key_begin(&mut self) -> Result;
    fn erased_map_value_begin(&mut self) -> Result;
    fn erased_map_end(&mut self) -> Result;
    fn erased_map_key<'b>(&mut self, k: UntypedValueRef<'a, 'b>) -> Result;
    fn erased_map_value<'b>(&mut self, v: UntypedValueRef<'a, 'b>) -> Result;
    fn erased_map_entry<'b>(
        &mut self,
        k: UntypedValueRef<'a, 'b>,
        v: UntypedValueRef<'a, 'b>,
    ) -> Result;
    fn erased_map_field<'b>(
        &mut self,
        f: TypedValueRef<'static, 'b, str>,
        v: UntypedValueRef<'a, 'b>,
    ) -> Result;
}

impl<'a, T: ?Sized> Stream<'a> for T
where
    T: stream::Stream<'a>,
{
    fn erased_i128(&mut self, v: i128) -> Result {
        self.i128(v)
    }

    fn erased_u128(&mut self, v: u128) -> Result {
        self.u128(v)
    }

    fn erased_str<'b>(&mut self, v: TypedValueRef<'a, 'b, str>) -> Result {
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

    fn erased_map_key<'b>(&mut self, k: UntypedValueRef<'a, 'b>) -> Result {
        self.map_key(k)
    }

    fn erased_map_value<'b>(&mut self, v: UntypedValueRef<'a, 'b>) -> Result {
        self.map_value(v)
    }

    fn erased_map_entry<'b>(
        &mut self,
        k: UntypedValueRef<'a, 'b>,
        v: UntypedValueRef<'a, 'b>,
    ) -> Result {
        self.map_entry(k, v)
    }

    fn erased_map_field<'b>(
        &mut self,
        f: TypedValueRef<'static, 'b, str>,
        v: UntypedValueRef<'a, 'b>,
    ) -> Result {
        self.map_field(f, v)
    }
}

impl<'a, 'b> stream::Stream<'a> for (dyn Stream<'a> + 'b) {
    fn u128(&mut self, v: u128) -> Result {
        self.erased_u128(v)
    }

    fn i128(&mut self, v: i128) -> Result {
        self.erased_i128(v)
    }

    fn str<V: stream::TypedValue<'a, str>>(&mut self, v: V) -> Result {
        self.erased_str(TypedValueRef(&v))
    }

    fn map_begin(&mut self, len: Option<usize>) -> Result {
        self.erased_map_begin(len)
    }

    fn map_key_begin(&mut self) -> Result {
        self.erased_map_key_begin()
    }

    fn map_value_begin(&mut self) -> Result {
        self.erased_map_value_begin()
    }

    fn map_end(&mut self) -> Result {
        self.erased_map_end()
    }

    fn map_key<K: stream::UntypedValue<'a>>(&mut self, k: K) -> Result {
        self.erased_map_key(UntypedValueRef(&k))
    }

    fn map_value<V: stream::UntypedValue<'a>>(&mut self, v: V) -> Result {
        self.erased_map_value(UntypedValueRef(&v))
    }

    fn map_entry<K: stream::UntypedValue<'a>, V: stream::UntypedValue<'a>>(
        &mut self,
        k: K,
        v: V,
    ) -> Result {
        self.erased_map_entry(UntypedValueRef(&k), UntypedValueRef(&v))
    }

    fn map_field<F: stream::TypedValue<'static, str>, V: stream::UntypedValue<'a>>(
        &mut self,
        f: F,
        v: V,
    ) -> Result {
        self.erased_map_field(TypedValueRef(&f), UntypedValueRef(&v))
    }
}

pub struct UntypedAnyRef<'a, 'b>(&'b dyn UntypedValue<'a>);

pub struct UntypedValueRef<'a, 'b>(&'b dyn UntypedValue<'a>);

trait UntypedValue<'a> {
    fn erased_untyped_stream<'b>(&self, stream: &mut dyn Stream<'b>) -> Result
    where
        'a: 'b;

    fn erased_untyped_stream_for_all<'b>(&self, stream: &mut dyn Stream<'b>) -> Result;
}

impl<'a, T> UntypedValue<'a> for T
where
    T: value_ref::UntypedValue<'a>,
{
    fn erased_untyped_stream<'b>(&self, stream: &mut dyn Stream<'b>) -> Result
    where
        'a: 'b,
    {
        self.stream(stream)
    }

    fn erased_untyped_stream_for_all<'b>(&self, stream: &mut dyn Stream<'b>) -> Result {
        use value_ref::UntypedValue;

        self.for_all().stream(stream)
    }
}

impl<'a, 'b> value_ref::UntypedValue<'a> for UntypedValueRef<'a, 'b> {
    type ForAll = UntypedAnyRef<'a, 'b>;

    fn for_all(&self) -> UntypedAnyRef<'a, 'b> {
        UntypedAnyRef(self.0)
    }

    fn stream<'c, S>(&self, mut stream: S) -> Result
    where
        'a: 'c,
        S: Stream<'c>,
    {
        self.0.erased_untyped_stream(&mut stream)
    }
}

impl<'a, 'b, 'c> value_ref::UntypedValue<'c> for UntypedAnyRef<'a, 'b> {
    type ForAll = Self;

    fn for_all(&self) -> Self {
        UntypedAnyRef(self.0)
    }

    fn stream<'d, S>(&self, mut stream: S) -> Result
    where
        'c: 'd,
        S: Stream<'d>,
    {
        self.0.erased_untyped_stream_for_all(&mut stream)
    }
}

pub struct TypedAnyRef<'a, 'b, T: ?Sized>(&'b dyn TypedValue<'a, T>);

pub struct TypedValueRef<'a, 'b, T: ?Sized>(&'b dyn TypedValue<'a, T>);

impl<'a, 'b, T: ?Sized> Deref for TypedAnyRef<'a, 'b, T> {
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

trait TypedValue<'a, T: ?Sized>: Deref<Target = T> {
    fn erased_untyped_stream<'b>(&self, stream: &mut dyn Stream<'b>) -> Result
    where
        'a: 'b;

    fn erased_untyped_stream_for_all<'b>(&self, stream: &mut dyn Stream<'b>) -> Result;

    fn erased_typed_stream<'b>(&self, stream: &mut dyn Stream<'b>) -> Result
    where
        'a: 'b;

    fn erased_typed_stream_for_all<'b>(&self, stream: &mut dyn Stream<'b>) -> Result;

    fn erased_typed_to_ref(&self) -> Option<&'a T>;
}

impl<'a, T, U: ?Sized> TypedValue<'a, U> for T
where
    T: value_ref::TypedValue<'a, U>,
{
    fn erased_untyped_stream<'b>(&self, stream: &mut dyn Stream<'b>) -> Result
    where
        'a: 'b,
    {
        use value_ref::UntypedValue;

        self.untype().stream(stream)
    }

    fn erased_untyped_stream_for_all<'b>(&self, stream: &mut dyn Stream<'b>) -> Result {
        use value_ref::UntypedValue;

        self.untype().for_all().stream(stream)
    }

    fn erased_typed_stream<'b>(&self, stream: &mut dyn Stream<'b>) -> Result
    where
        'a: 'b,
    {
        self.stream(stream)
    }

    fn erased_typed_stream_for_all<'b>(&self, stream: &mut dyn Stream<'b>) -> Result {
        use value_ref::TypedValue;

        self.for_all().stream(stream)
    }

    fn erased_typed_to_ref(&self) -> Option<&'a U> {
        self.to_ref()
    }
}

impl<'a, 'b, T: ?Sized> value_ref::TypedValue<'a, T> for TypedValueRef<'a, 'b, T> {
    type UntypedValue = Self;
    type ForAll = TypedAnyRef<'a, 'b, T>;

    fn untype(&self) -> Self {
        TypedValueRef(self.0)
    }

    fn to_ref(&self) -> Option<&'a T> {
        self.0.erased_typed_to_ref()
    }

    fn for_all(&self) -> TypedAnyRef<'a, 'b, T> {
        TypedAnyRef(self.0)
    }

    fn stream<'c, S>(&self, mut stream: S) -> Result
    where
        'a: 'c,
        S: Stream<'c>,
    {
        self.0.erased_typed_stream(&mut stream)
    }
}

impl<'a, 'b, 'c, T: ?Sized> value_ref::UntypedValue<'c> for TypedValueRef<'a, 'b, T> {
    type ForAll = TypedAnyRef<'a, 'b, T>;

    fn for_all(&self) -> TypedAnyRef<'a, 'b, T> {
        TypedAnyRef(self.0)
    }

    fn stream<'d, S>(&self, mut stream: S) -> Result
    where
        'c: 'd,
        S: Stream<'d>,
    {
        self.0.erased_untyped_stream_for_all(&mut stream)
    }
}

impl<'a, 'b, 'c, T: ?Sized> value_ref::TypedValue<'c, T> for TypedAnyRef<'a, 'b, T> {
    type UntypedValue = Self;
    type ForAll = Self;

    fn untype(&self) -> Self::UntypedValue {
        TypedAnyRef(self.0)
    }

    fn to_ref(&self) -> Option<&'c T> {
        self.erased_typed_to_ref()
    }

    fn for_all(&self) -> Self {
        TypedAnyRef(self.0)
    }

    fn stream<'d, S>(&self, mut stream: S) -> Result
    where
        'c: 'd,
        S: Stream<'d>,
    {
        self.0.erased_typed_stream_for_all(&mut stream)
    }
}

impl<'a, 'b, 'c, T: ?Sized> value_ref::UntypedValue<'c> for TypedAnyRef<'a, 'b, T> {
    type ForAll = Self;

    fn for_all(&self) -> Self {
        TypedAnyRef(self.0)
    }

    fn stream<'d, S>(&self, mut stream: S) -> Result
    where
        'c: 'd,
        S: Stream<'d>,
    {
        self.0.erased_untyped_stream_for_all(&mut stream)
    }
}
