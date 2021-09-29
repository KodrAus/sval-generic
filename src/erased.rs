use std::{error, fmt};

use crate::{reference, stream, tag, value, Result};

// TODO: This public API needs to be a trait we can add OIBIT's to
pub struct Value<'a>(&'a dyn ErasedValue);

impl<'a> Value<'a> {
    pub fn new(v: &'a impl value::Value) -> Self {
        Value(v)
    }
}

trait ErasedValue {
    fn erased_stream<'a>(&'a self, stream: Stream<'a, '_>) -> Result;
    fn erased_to_str(&self) -> Option<&str>;
}

impl<T: value::Value + ?Sized> ErasedValue for T {
    fn erased_stream<'a>(&'a self, stream: Stream<'a, '_>) -> Result {
        self.stream(stream)
    }

    fn erased_to_str(&self) -> Option<&str> {
        self.to_str()
    }
}

impl<'a> value::Value for Value<'a> {
    fn stream<'b, S: stream::Stream<'b>>(&'b self, mut stream: S) -> Result {
        self.0.erased_stream(Stream(&mut stream))
    }

    fn to_str(&self) -> Option<&str> {
        self.0.erased_to_str()
    }
}

// TODO: This public API needs to be a trait we can add OIBIT's to
pub struct Stream<'a, 'b>(&'b mut dyn ErasedStream<'a>);

impl<'a, 'b> Stream<'a, 'b> {
    pub fn new(s: &'b mut impl stream::Stream<'a>) -> Self {
        Stream(s)
    }
}

trait ErasedStream<'a> {
    fn erased_any<'b, 'v: 'a>(&mut self, v: ValueRef<'v, 'b>) -> Result;
    fn erased_u64(&mut self, v: u64) -> Result;
    fn erased_i64(&mut self, v: i64) -> Result;
    fn erased_i128(&mut self, v: i128) -> Result;
    fn erased_u128(&mut self, v: u128) -> Result;
    fn erased_f64(&mut self, v: f64) -> Result;
    fn erased_bool(&mut self, v: bool) -> Result;
    fn erased_none(&mut self) -> Result;
    fn erased_display(&mut self, v: &dyn fmt::Display) -> Result;
    fn erased_error<'b, 'v: 'a>(
        &mut self,
        e: TypedRef<'v, 'b, dyn error::Error + 'static>,
    ) -> Result;
    fn erased_str<'b, 'v: 'a>(&mut self, v: TypedRef<'v, 'b, str>) -> Result;
    fn erased_type_tag<'b>(&mut self, tag_ty: TypedRef<'static, 'b, str>) -> Result;
    fn erased_variant_tag<'b>(
        &mut self,
        tag_ty: TypedRef<'static, 'b, str>,
        tag_variant_key: TypedRef<'static, 'b, str>,
        tag_variant_index: Option<u64>,
    ) -> Result;
    fn erased_type_tagged_begin<'b>(&mut self, tag_ty: TypedRef<'static, 'b, str>) -> Result;
    fn erased_type_tagged_end(&mut self) -> Result;
    fn erased_variant_tagged_begin<'b>(
        &mut self,
        tag_ty: TypedRef<'static, 'b, str>,
        tag_variant_key: TypedRef<'static, 'b, str>,
        tag_variant_index: Option<u64>,
    ) -> Result;
    fn erased_variant_tagged_end(&mut self) -> Result;
    fn erased_type_tagged<'b, 'v: 'a>(
        &mut self,
        tag_ty: TypedRef<'static, 'b, str>,
        v: ValueRef<'v, 'b>,
    ) -> Result;
    fn erased_variant_tagged<'b, 'v: 'a>(
        &mut self,
        tag_ty: TypedRef<'static, 'b, str>,
        tag_variant_key: TypedRef<'static, 'b, str>,
        tag_variant_index: Option<u64>,
        v: ValueRef<'v, 'b>,
    ) -> Result;
    fn erased_map_begin(&mut self, len: Option<usize>) -> Result;
    fn erased_map_key_begin(&mut self) -> Result;
    fn erased_map_key_end(&mut self) -> Result;
    fn erased_map_value_begin(&mut self) -> Result;
    fn erased_map_value_end(&mut self) -> Result;
    fn erased_map_end(&mut self) -> Result;
    fn erased_type_tagged_map_begin<'b>(
        &mut self,
        tag_ty: TypedRef<'static, 'b, str>,
        len: Option<usize>,
    ) -> Result;
    fn erased_type_tagged_map_end(&mut self) -> Result;
    fn erased_variant_tagged_map_begin<'b>(
        &mut self,
        tag_ty: TypedRef<'static, 'b, str>,
        tag_variant_key: TypedRef<'static, 'b, str>,
        tag_variant_index: Option<u64>,
        len: Option<usize>,
    ) -> Result;
    fn erased_variant_tagged_map_end(&mut self) -> Result;
    fn erased_map_key<'b, 'k: 'a>(&mut self, k: ValueRef<'k, 'b>) -> Result;
    fn erased_map_value<'b, 'v: 'a>(&mut self, v: ValueRef<'v, 'b>) -> Result;
    fn erased_map_entry<'b, 'k: 'a, 'v: 'a>(
        &mut self,
        k: ValueRef<'k, 'b>,
        v: ValueRef<'v, 'b>,
    ) -> Result;
    fn erased_map_field<'b>(&mut self, f: TypedRef<'static, 'b, str>) -> Result;
    fn erased_seq_begin(&mut self, len: Option<usize>) -> Result;
    fn erased_seq_end(&mut self) -> Result;
    fn erased_type_tagged_seq_begin<'b>(
        &mut self,
        tag_ty: TypedRef<'static, 'b, str>,
        len: Option<usize>,
    ) -> Result;
    fn erased_type_tagged_seq_end(&mut self) -> Result;
    fn erased_variant_tagged_seq_begin<'b>(
        &mut self,
        tag_ty: TypedRef<'static, 'b, str>,
        tag_variant_key: TypedRef<'static, 'b, str>,
        tag_variant_index: Option<u64>,
        len: Option<usize>,
    ) -> Result;
    fn erased_variant_tagged_seq_end(&mut self) -> Result;
    fn erased_seq_elem_begin(&mut self) -> Result;
    fn erased_seq_elem_end(&mut self) -> Result;
    fn erased_seq_elem<'b, 'e: 'a>(&mut self, e: ValueRef<'e, 'b>) -> Result;
}

impl<'a, T: stream::Stream<'a> + ?Sized> ErasedStream<'a> for T {
    fn erased_any<'b, 'v: 'a>(&mut self, v: ValueRef<'v, 'b>) -> Result {
        self.any(v)
    }

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

    fn erased_error<'b, 'v: 'a>(
        &mut self,
        e: TypedRef<'v, 'b, dyn error::Error + 'static>,
    ) -> Result {
        self.error(e)
    }

    fn erased_str<'b, 'v: 'a>(&mut self, v: TypedRef<'v, 'b, str>) -> Result {
        self.str(v)
    }

    fn erased_type_tag<'b>(&mut self, tag_ty: TypedRef<'static, 'b, str>) -> Result {
        self.type_tag(tag::TypeTag::new(tag_ty))
    }

    fn erased_variant_tag<'b>(
        &mut self,
        tag_ty: TypedRef<'static, 'b, str>,
        tag_variant_key: TypedRef<'static, 'b, str>,
        tag_variant_index: Option<u64>,
    ) -> Result {
        self.variant_tag(tag::VariantTag::new(
            tag_ty,
            tag_variant_key,
            tag_variant_index,
        ))
    }

    fn erased_type_tagged_begin<'b>(&mut self, tag_ty: TypedRef<'static, 'b, str>) -> Result {
        self.type_tagged_begin(tag::TypeTag::new(tag_ty))
    }

    fn erased_type_tagged_end(&mut self) -> Result {
        self.type_tagged_end()
    }

    fn erased_variant_tagged_begin<'b>(
        &mut self,
        tag_ty: TypedRef<'static, 'b, str>,
        tag_variant_key: TypedRef<'static, 'b, str>,
        tag_variant_index: Option<u64>,
    ) -> Result {
        self.variant_tagged_begin(tag::VariantTag::new(
            tag_ty,
            tag_variant_key,
            tag_variant_index,
        ))
    }

    fn erased_variant_tagged_end(&mut self) -> Result {
        self.variant_tagged_end()
    }

    fn erased_type_tagged<'b, 'v: 'a>(
        &mut self,
        tag_ty: TypedRef<'static, 'b, str>,
        v: ValueRef<'v, 'b>,
    ) -> Result {
        self.type_tagged(tag::TypeTag::new(tag_ty), v)
    }

    fn erased_variant_tagged<'b, 'v: 'a>(
        &mut self,
        tag_ty: TypedRef<'static, 'b, str>,
        tag_variant_key: TypedRef<'static, 'b, str>,
        tag_variant_index: Option<u64>,
        v: ValueRef<'v, 'b>,
    ) -> Result {
        self.variant_tagged(
            tag::VariantTag::new(tag_ty, tag_variant_key, tag_variant_index),
            v,
        )
    }

    fn erased_map_begin(&mut self, len: Option<usize>) -> Result {
        self.map_begin(len)
    }

    fn erased_map_key_begin(&mut self) -> Result {
        self.map_key_begin()
    }

    fn erased_map_key_end(&mut self) -> Result {
        self.map_key_end()
    }

    fn erased_map_value_begin(&mut self) -> Result {
        self.map_value_begin()
    }

    fn erased_map_value_end(&mut self) -> Result {
        self.map_value_end()
    }

    fn erased_map_end(&mut self) -> Result {
        self.map_end()
    }

    fn erased_type_tagged_map_begin<'b>(
        &mut self,
        tag_ty: TypedRef<'static, 'b, str>,
        len: Option<usize>,
    ) -> Result {
        self.type_tagged_map_begin(tag::TypeTag::new(tag_ty), len)
    }

    fn erased_type_tagged_map_end(&mut self) -> Result {
        self.type_tagged_map_end()
    }

    fn erased_variant_tagged_map_begin<'b>(
        &mut self,
        tag_ty: TypedRef<'static, 'b, str>,
        tag_variant_key: TypedRef<'static, 'b, str>,
        tag_variant_index: Option<u64>,
        len: Option<usize>,
    ) -> Result {
        self.variant_tagged_map_begin(
            tag::VariantTag::new(tag_ty, tag_variant_key, tag_variant_index),
            len,
        )
    }

    fn erased_variant_tagged_map_end(&mut self) -> Result {
        self.variant_tagged_map_end()
    }

    fn erased_map_key<'b, 'k: 'a>(&mut self, k: ValueRef<'k, 'b>) -> Result {
        self.map_key(k)
    }

    fn erased_map_value<'b, 'v: 'a>(&mut self, v: ValueRef<'v, 'b>) -> Result {
        self.map_value(v)
    }

    fn erased_map_entry<'b, 'k: 'a, 'v: 'a>(
        &mut self,
        k: ValueRef<'k, 'b>,
        v: ValueRef<'v, 'b>,
    ) -> Result {
        self.map_entry(k, v)
    }

    fn erased_map_field<'b>(&mut self, f: TypedRef<'static, 'b, str>) -> Result {
        self.map_field(f)
    }

    fn erased_seq_begin(&mut self, len: Option<usize>) -> Result {
        self.seq_begin(len)
    }

    fn erased_seq_end(&mut self) -> Result {
        self.seq_end()
    }

    fn erased_type_tagged_seq_begin<'b>(
        &mut self,
        tag_ty: TypedRef<'static, 'b, str>,
        len: Option<usize>,
    ) -> Result {
        self.type_tagged_seq_begin(tag::TypeTag::new(tag_ty), len)
    }

    fn erased_type_tagged_seq_end(&mut self) -> Result {
        self.type_tagged_seq_end()
    }

    fn erased_variant_tagged_seq_begin<'b>(
        &mut self,
        tag_ty: TypedRef<'static, 'b, str>,
        tag_variant_key: TypedRef<'static, 'b, str>,
        tag_variant_index: Option<u64>,
        len: Option<usize>,
    ) -> Result {
        self.variant_tagged_seq_begin(
            tag::VariantTag::new(tag_ty, tag_variant_key, tag_variant_index),
            len,
        )
    }

    fn erased_variant_tagged_seq_end(&mut self) -> Result {
        self.variant_tagged_seq_end()
    }

    fn erased_seq_elem_begin(&mut self) -> Result {
        self.seq_elem_begin()
    }

    fn erased_seq_elem_end(&mut self) -> Result {
        self.seq_elem_end()
    }

    fn erased_seq_elem<'b, 'e: 'a>(&mut self, e: ValueRef<'e, 'b>) -> Result {
        self.seq_elem(e)
    }
}

impl<'a, 'b> stream::Stream<'a> for Stream<'a, 'b> {
    fn any<'v: 'a, V: reference::ValueRef<'v>>(&mut self, value: V) -> Result {
        self.erased_any(ValueRef(&value))
    }

    fn display<D: fmt::Display>(&mut self, fmt: D) -> Result {
        self.0.erased_display(&fmt)
    }

    fn u64(&mut self, value: u64) -> Result {
        self.0.erased_u64(value)
    }

    fn i64(&mut self, value: i64) -> Result {
        self.0.erased_i64(value)
    }

    fn u128(&mut self, value: u128) -> Result {
        self.0.erased_u128(value)
    }

    fn i128(&mut self, value: i128) -> Result {
        self.0.erased_i128(value)
    }

    fn f64(&mut self, value: f64) -> Result {
        self.0.erased_f64(value)
    }

    fn bool(&mut self, value: bool) -> Result {
        self.0.erased_bool(value)
    }

    fn none(&mut self) -> Result {
        self.0.erased_none()
    }

    fn str<'s: 'a, T: reference::TypedRef<'s, str>>(&mut self, value: T) -> Result {
        self.0.erased_str(TypedRef(&value))
    }

    fn error<'e: 'a, E: reference::TypedRef<'e, dyn error::Error + 'static>>(
        &mut self,
        error: E,
    ) -> Result {
        self.0.erased_error(TypedRef(&error))
    }

    fn type_tag<T: reference::TypedRef<'static, str>>(&mut self, tag: tag::TypeTag<T>) -> Result {
        self.0.erased_type_tag(TypedRef(&tag.ty()))
    }

    fn variant_tag<T: reference::TypedRef<'static, str>, K: reference::TypedRef<'static, str>>(
        &mut self,
        tag: tag::VariantTag<T, K>,
    ) -> Result {
        self.0.erased_variant_tag(
            TypedRef(&tag.ty()),
            TypedRef(&tag.variant_key()),
            tag.variant_index(),
        )
    }

    fn type_tagged_begin<T: reference::TypedRef<'static, str>>(
        &mut self,
        tag: tag::TypeTag<T>,
    ) -> Result {
        self.0.erased_type_tagged_begin(TypedRef(&tag.ty()))
    }

    fn type_tagged_end(&mut self) -> Result {
        self.0.erased_type_tagged_end()
    }

    fn variant_tagged_begin<
        T: reference::TypedRef<'static, str>,
        K: reference::TypedRef<'static, str>,
    >(
        &mut self,
        tag: tag::VariantTag<T, K>,
    ) -> Result {
        self.0.erased_variant_tagged_begin(
            TypedRef(&tag.ty()),
            TypedRef(&tag.variant_key()),
            tag.variant_index(),
        )
    }

    fn variant_tagged_end(&mut self) -> Result {
        self.0.erased_variant_tagged_end()
    }

    fn type_tagged<'v: 'a, T: reference::TypedRef<'static, str>, V: reference::ValueRef<'v>>(
        &mut self,
        tag: tag::TypeTag<T>,
        value: V,
    ) -> Result {
        self.0
            .erased_type_tagged(TypedRef(&tag.ty()), ValueRef(&value))
    }

    fn variant_tagged<
        'v: 'a,
        T: reference::TypedRef<'static, str>,
        K: reference::TypedRef<'static, str>,
        V: reference::ValueRef<'v>,
    >(
        &mut self,
        tag: tag::VariantTag<T, K>,
        value: V,
    ) -> Result {
        self.0.erased_variant_tagged(
            TypedRef(&tag.ty()),
            TypedRef(&tag.variant_key()),
            tag.variant_index(),
            ValueRef(&value),
        )
    }

    fn map_begin(&mut self, len: Option<usize>) -> Result {
        self.0.erased_map_begin(len)
    }

    fn map_end(&mut self) -> Result {
        self.0.erased_map_end()
    }

    fn map_key_begin(&mut self) -> Result {
        self.0.erased_map_key_begin()
    }

    fn map_key_end(&mut self) -> Result {
        self.0.erased_map_key_end()
    }

    fn map_value_begin(&mut self) -> Result {
        self.0.erased_map_value_begin()
    }

    fn map_value_end(&mut self) -> Result {
        self.0.erased_map_value_end()
    }

    fn type_tagged_map_begin<T: reference::TypedRef<'static, str>>(
        &mut self,
        tag: tag::TypeTag<T>,
        len: Option<usize>,
    ) -> Result {
        self.0
            .erased_type_tagged_map_begin(TypedRef(&tag.ty()), len)
    }

    fn type_tagged_map_end(&mut self) -> Result {
        self.0.erased_type_tagged_map_end()
    }

    fn variant_tagged_map_begin<
        T: reference::TypedRef<'static, str>,
        K: reference::TypedRef<'static, str>,
    >(
        &mut self,
        tag: tag::VariantTag<T, K>,
        len: Option<usize>,
    ) -> Result {
        self.0.erased_variant_tagged_map_begin(
            TypedRef(&tag.ty()),
            TypedRef(&tag.variant_key()),
            tag.variant_index(),
            len,
        )
    }

    fn variant_tagged_map_end(&mut self) -> Result {
        self.0.erased_variant_tagged_map_end()
    }

    fn map_entry<'k: 'a, 'v: 'a, K: reference::ValueRef<'k>, V: reference::ValueRef<'v>>(
        &mut self,
        key: K,
        value: V,
    ) -> Result {
        self.0.erased_map_entry(ValueRef(&key), ValueRef(&value))
    }

    fn map_key<'k: 'a, K: reference::ValueRef<'k>>(&mut self, key: K) -> Result {
        self.0.erased_map_key(ValueRef(&key))
    }

    fn map_field<F: reference::TypedRef<'static, str>>(&mut self, field: F) -> Result {
        self.0.erased_map_field(TypedRef(&field))
    }

    fn map_value<'v: 'a, V: reference::ValueRef<'v>>(&mut self, value: V) -> Result {
        self.0.erased_map_value(ValueRef(&value))
    }

    fn seq_begin(&mut self, len: Option<usize>) -> Result {
        self.0.erased_seq_begin(len)
    }

    fn seq_end(&mut self) -> Result {
        self.0.erased_seq_end()
    }

    fn type_tagged_seq_begin<T: reference::TypedRef<'static, str>>(
        &mut self,
        tag: tag::TypeTag<T>,
        len: Option<usize>,
    ) -> Result {
        self.0
            .erased_type_tagged_seq_begin(TypedRef(&tag.ty()), len)
    }

    fn type_tagged_seq_end(&mut self) -> Result {
        self.0.erased_type_tagged_seq_end()
    }

    fn variant_tagged_seq_begin<
        T: reference::TypedRef<'static, str>,
        K: reference::TypedRef<'static, str>,
    >(
        &mut self,
        tag: tag::VariantTag<T, K>,
        len: Option<usize>,
    ) -> Result {
        self.0.erased_variant_tagged_seq_begin(
            TypedRef(&tag.ty()),
            TypedRef(&tag.variant_key()),
            tag.variant_index(),
            len,
        )
    }

    fn variant_tagged_seq_end(&mut self) -> Result {
        self.0.erased_variant_tagged_seq_end()
    }

    fn seq_elem_begin(&mut self) -> Result {
        self.0.erased_seq_elem_begin()
    }

    fn seq_elem_end(&mut self) -> Result {
        self.0.erased_seq_elem_end()
    }

    fn seq_elem<'e: 'a, E: reference::ValueRef<'e>>(&mut self, elem: E) -> Result {
        self.0.erased_seq_elem(ValueRef(&elem))
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

    fn erased_to_str(&self) -> Option<&'a str>;
}

impl<'a, T: reference::ValueRef<'a>> ErasedValueRef<'a> for T {
    fn erased_stream<'b>(&self, stream: Stream<'b, '_>) -> Result
    where
        'a: 'b,
    {
        reference::ValueRef::stream(*self, stream)
    }

    fn erased_to_str(&self) -> Option<&'a str> {
        reference::ValueRef::to_str(*self)
    }
}

impl<'a, 'b> reference::ValueRef<'a> for ValueRef<'a, 'b> {
    fn stream<'c, S: stream::Stream<'c>>(self, mut stream: S) -> Result
    where
        'a: 'c,
    {
        self.0.erased_stream(Stream(&mut stream))
    }

    fn to_str(self) -> Option<&'a str> {
        self.0.erased_to_str()
    }
}

impl<'a, 'b> value::Value for ValueRef<'a, 'b> {
    fn stream<'c, S: stream::Stream<'c>>(&'c self, mut stream: S) -> Result {
        self.0.erased_stream(Stream(&mut stream))
    }
}

struct TypedRef<'a, 'b, T: ?Sized>(&'b dyn ErasedTypedRef<'a, T>);

impl<'a, 'b, T: ?Sized> Clone for TypedRef<'a, 'b, T> {
    fn clone(&self) -> Self {
        TypedRef(self.0)
    }
}

impl<'a, 'b, T: ?Sized> Copy for TypedRef<'a, 'b, T> {}

trait ErasedTypedRef<'a, T: ?Sized> {
    fn erased_stream<'b>(&self, stream: Stream<'b, '_>) -> Result
    where
        'a: 'b;

    fn erased_to_str(&self) -> Option<&'a str>;

    fn erased_get(&self) -> &T;
    fn erased_try_unwrap(&self) -> Option<&'a T>;
}

impl<'a, U: value::Value + ?Sized + 'static, T: reference::TypedRef<'a, U>> ErasedTypedRef<'a, U>
    for T
{
    fn erased_stream<'b>(&self, stream: Stream<'b, '_>) -> Result
    where
        'a: 'b,
    {
        reference::ValueRef::stream(*self, stream)
    }

    fn erased_to_str(&self) -> Option<&'a str> {
        reference::ValueRef::to_str(*self)
    }

    fn erased_get(&self) -> &U {
        self.get()
    }

    fn erased_try_unwrap(&self) -> Option<&'a U> {
        self.try_unwrap()
    }
}

impl<'a, 'b, T: value::Value + ?Sized + 'static> reference::TypedRef<'a, T>
    for TypedRef<'a, 'b, T>
{
    fn get(&self) -> &T {
        self.0.erased_get()
    }

    fn try_unwrap(self) -> Option<&'a T> {
        self.0.erased_try_unwrap()
    }
}

impl<'a, 'b, T: ?Sized> reference::ValueRef<'a> for TypedRef<'a, 'b, T> {
    fn stream<'c, S: stream::Stream<'c>>(self, mut stream: S) -> Result
    where
        'a: 'c,
    {
        self.0.erased_stream(Stream(&mut stream))
    }

    fn to_str(self) -> Option<&'a str> {
        self.0.erased_to_str()
    }
}

impl<'a, 'b, T: ?Sized> value::Value for TypedRef<'a, 'b, T> {
    fn stream<'c, S: stream::Stream<'c>>(&'c self, mut stream: S) -> Result {
        self.0.erased_stream(Stream(&mut stream))
    }
}
