use std::{borrow::ToOwned, error, fmt};

use sval_generic_api::{
    receiver,
    source::{self, Stream},
    tag, value, Error, Result,
};

// TODO: This public API needs to be a trait we can add OIBIT's to
// pub trait Value { fn erase(&'a self) -> Value<'a> }
pub struct Value<'a>(&'a dyn ErasedValue);

impl<'a> Value<'a> {
    pub fn new(v: &'a impl value::Value) -> Self {
        Value(v)
    }
}

pub fn value<'a>(v: &'a impl value::Value) -> Value<'a> {
    Value::new(v)
}

trait ErasedValue {
    fn erased_stream<'a>(&'a self, receiver: Receiver<'a, '_>) -> Result;
    fn erased_to_str(&self) -> Option<&str>;
}

impl<T: value::Value + ?Sized> ErasedValue for T {
    fn erased_stream<'a>(&'a self, receiver: Receiver<'a, '_>) -> Result {
        self.stream(receiver)
    }

    fn erased_to_str(&self) -> Option<&str> {
        self.to_str()
    }
}

impl<'a> value::Value for Value<'a> {
    fn stream<'b, S: receiver::Receiver<'b>>(&'b self, mut receiver: S) -> Result {
        self.0.erased_stream(Receiver(&mut receiver))
    }

    fn to_str(&self) -> Option<&str> {
        self.0.erased_to_str()
    }
}

// TODO: This public API needs to be a trait we can add OIBIT's to
pub struct Receiver<'a, 'b>(&'b mut dyn ErasedReceiver<'a>);

impl<'a, 'b> Receiver<'a, 'b> {
    pub fn new(s: &'b mut impl receiver::Receiver<'a>) -> Self {
        Receiver(s)
    }
}

pub fn receiver<'a, 'b>(s: &'b mut impl receiver::Receiver<'a>) -> Receiver<'a, 'b> {
    Receiver::new(s)
}

trait ErasedReceiver<'a> {
    fn erased_source<'b, 'v: 'a>(&mut self, v: Source<'v, 'b>) -> Result;
    fn erased_u8(&mut self, v: u8) -> Result;
    fn erased_u16(&mut self, v: u16) -> Result;
    fn erased_u32(&mut self, v: u32) -> Result;
    fn erased_u64(&mut self, v: u64) -> Result;
    fn erased_i8(&mut self, v: i8) -> Result;
    fn erased_i16(&mut self, v: i16) -> Result;
    fn erased_i32(&mut self, v: i32) -> Result;
    fn erased_i64(&mut self, v: i64) -> Result;
    fn erased_i128(&mut self, v: i128) -> Result;
    fn erased_u128(&mut self, v: u128) -> Result;
    fn erased_f32(&mut self, v: f32) -> Result;
    fn erased_f64(&mut self, v: f64) -> Result;
    fn erased_bool(&mut self, v: bool) -> Result;
    fn erased_none(&mut self) -> Result;
    fn erased_display(&mut self, v: &dyn fmt::Display) -> Result;
    fn erased_error<'b, 'v: 'a>(
        &mut self,
        e: ValueSource<'v, 'b, dyn error::Error + 'static>,
    ) -> Result;
    fn erased_char(&mut self, v: char) -> Result;
    fn erased_str<'b, 'v: 'a>(&mut self, v: ValueSource<'v, 'b, str>) -> Result;
    fn erased_type_tag<'b>(&mut self, tag_ty: ValueSource<'static, 'b, str>) -> Result;
    fn erased_variant_tag<'b>(
        &mut self,
        tag_ty: ValueSource<'static, 'b, str>,
        tag_variant_key: ValueSource<'static, 'b, str>,
        tag_variant_index: Option<u64>,
    ) -> Result;
    fn erased_type_tagged_begin<'b>(&mut self, tag_ty: ValueSource<'static, 'b, str>) -> Result;
    fn erased_type_tagged_end(&mut self) -> Result;
    fn erased_variant_tagged_begin<'b>(
        &mut self,
        tag_ty: ValueSource<'static, 'b, str>,
        tag_variant_key: ValueSource<'static, 'b, str>,
        tag_variant_index: Option<u64>,
    ) -> Result;
    fn erased_variant_tagged_end(&mut self) -> Result;
    fn erased_type_tagged<'b, 'v: 'a>(
        &mut self,
        tag_ty: ValueSource<'static, 'b, str>,
        v: Source<'v, 'b>,
    ) -> Result;
    fn erased_variant_tagged<'b, 'v: 'a>(
        &mut self,
        tag_ty: ValueSource<'static, 'b, str>,
        tag_variant_key: ValueSource<'static, 'b, str>,
        tag_variant_index: Option<u64>,
        v: Source<'v, 'b>,
    ) -> Result;
    fn erased_map_begin(&mut self, len: Option<usize>) -> Result;
    fn erased_map_key_begin(&mut self) -> Result;
    fn erased_map_key_end(&mut self) -> Result;
    fn erased_map_value_begin(&mut self) -> Result;
    fn erased_map_value_end(&mut self) -> Result;
    fn erased_map_end(&mut self) -> Result;
    fn erased_type_tagged_map_begin<'b>(
        &mut self,
        tag_ty: ValueSource<'static, 'b, str>,
        len: Option<usize>,
    ) -> Result;
    fn erased_type_tagged_map_end(&mut self) -> Result;
    fn erased_variant_tagged_map_begin<'b>(
        &mut self,
        tag_ty: ValueSource<'static, 'b, str>,
        tag_variant_key: ValueSource<'static, 'b, str>,
        tag_variant_index: Option<u64>,
        len: Option<usize>,
    ) -> Result;
    fn erased_variant_tagged_map_end(&mut self) -> Result;
    fn erased_map_key<'b, 'k: 'a>(&mut self, k: Source<'k, 'b>) -> Result;
    fn erased_map_value<'b, 'v: 'a>(&mut self, v: Source<'v, 'b>) -> Result;
    fn erased_map_entry<'b, 'k: 'a, 'v: 'a>(
        &mut self,
        k: Source<'k, 'b>,
        v: Source<'v, 'b>,
    ) -> Result;
    fn erased_map_field_entry<'b, 'v: 'a>(
        &mut self,
        f: ValueSource<'static, 'b, str>,
        v: Source<'v, 'b>,
    ) -> Result;
    fn erased_map_field<'b>(&mut self, f: ValueSource<'static, 'b, str>) -> Result;
    fn erased_seq_begin(&mut self, len: Option<usize>) -> Result;
    fn erased_seq_end(&mut self) -> Result;
    fn erased_type_tagged_seq_begin<'b>(
        &mut self,
        tag_ty: ValueSource<'static, 'b, str>,
        len: Option<usize>,
    ) -> Result;
    fn erased_type_tagged_seq_end(&mut self) -> Result;
    fn erased_variant_tagged_seq_begin<'b>(
        &mut self,
        tag_ty: ValueSource<'static, 'b, str>,
        tag_variant_key: ValueSource<'static, 'b, str>,
        tag_variant_index: Option<u64>,
        len: Option<usize>,
    ) -> Result;
    fn erased_variant_tagged_seq_end(&mut self) -> Result;
    fn erased_seq_elem_begin(&mut self) -> Result;
    fn erased_seq_elem_end(&mut self) -> Result;
    fn erased_seq_elem<'b, 'e: 'a>(&mut self, e: Source<'e, 'b>) -> Result;
}

impl<'a, T: receiver::Receiver<'a> + ?Sized> ErasedReceiver<'a> for T {
    fn erased_source<'b, 'v: 'a>(&mut self, v: Source<'v, 'b>) -> Result {
        self.source(v)
    }

    fn erased_u8(&mut self, value: u8) -> Result {
        self.u8(value)
    }

    fn erased_u16(&mut self, value: u16) -> Result {
        self.u16(value)
    }

    fn erased_u32(&mut self, value: u32) -> Result {
        self.u32(value)
    }

    fn erased_u64(&mut self, v: u64) -> Result {
        self.u64(v)
    }

    fn erased_i8(&mut self, value: i8) -> Result {
        self.i8(value)
    }

    fn erased_i16(&mut self, value: i16) -> Result {
        self.i16(value)
    }

    fn erased_i32(&mut self, value: i32) -> Result {
        self.i32(value)
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

    fn erased_f32(&mut self, value: f32) -> Result {
        self.f32(value)
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
        e: ValueSource<'v, 'b, dyn error::Error + 'static>,
    ) -> Result {
        self.error(e)
    }

    fn erased_char(&mut self, v: char) -> Result {
        self.char(v)
    }

    fn erased_str<'b, 'v: 'a>(&mut self, v: ValueSource<'v, 'b, str>) -> Result {
        self.str(v)
    }

    fn erased_type_tag<'b>(&mut self, tag_ty: ValueSource<'static, 'b, str>) -> Result {
        self.type_tag(tag::TypeTag::new(tag_ty))
    }

    fn erased_variant_tag<'b>(
        &mut self,
        tag_ty: ValueSource<'static, 'b, str>,
        tag_variant_key: ValueSource<'static, 'b, str>,
        tag_variant_index: Option<u64>,
    ) -> Result {
        self.variant_tag(tag::VariantTag::new(
            tag_ty,
            tag_variant_key,
            tag_variant_index,
        ))
    }

    fn erased_type_tagged_begin<'b>(&mut self, tag_ty: ValueSource<'static, 'b, str>) -> Result {
        self.type_tagged_begin(tag::TypeTag::new(tag_ty))
    }

    fn erased_type_tagged_end(&mut self) -> Result {
        self.type_tagged_end()
    }

    fn erased_variant_tagged_begin<'b>(
        &mut self,
        tag_ty: ValueSource<'static, 'b, str>,
        tag_variant_key: ValueSource<'static, 'b, str>,
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
        tag_ty: ValueSource<'static, 'b, str>,
        v: Source<'v, 'b>,
    ) -> Result {
        self.type_tagged(tag::TypeTag::new(tag_ty), v)
    }

    fn erased_variant_tagged<'b, 'v: 'a>(
        &mut self,
        tag_ty: ValueSource<'static, 'b, str>,
        tag_variant_key: ValueSource<'static, 'b, str>,
        tag_variant_index: Option<u64>,
        v: Source<'v, 'b>,
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
        tag_ty: ValueSource<'static, 'b, str>,
        len: Option<usize>,
    ) -> Result {
        self.type_tagged_map_begin(tag::TypeTag::new(tag_ty), len)
    }

    fn erased_type_tagged_map_end(&mut self) -> Result {
        self.type_tagged_map_end()
    }

    fn erased_variant_tagged_map_begin<'b>(
        &mut self,
        tag_ty: ValueSource<'static, 'b, str>,
        tag_variant_key: ValueSource<'static, 'b, str>,
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

    fn erased_map_key<'b, 'k: 'a>(&mut self, k: Source<'k, 'b>) -> Result {
        self.map_key(k)
    }

    fn erased_map_value<'b, 'v: 'a>(&mut self, v: Source<'v, 'b>) -> Result {
        self.map_value(v)
    }

    fn erased_map_entry<'b, 'k: 'a, 'v: 'a>(
        &mut self,
        k: Source<'k, 'b>,
        v: Source<'v, 'b>,
    ) -> Result {
        self.map_entry(k, v)
    }

    fn erased_map_field_entry<'b, 'v: 'a>(
        &mut self,
        f: ValueSource<'static, 'b, str>,
        v: Source<'v, 'b>,
    ) -> Result {
        self.map_field_entry(f, v)
    }

    fn erased_map_field<'b>(&mut self, f: ValueSource<'static, 'b, str>) -> Result {
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
        tag_ty: ValueSource<'static, 'b, str>,
        len: Option<usize>,
    ) -> Result {
        self.type_tagged_seq_begin(tag::TypeTag::new(tag_ty), len)
    }

    fn erased_type_tagged_seq_end(&mut self) -> Result {
        self.type_tagged_seq_end()
    }

    fn erased_variant_tagged_seq_begin<'b>(
        &mut self,
        tag_ty: ValueSource<'static, 'b, str>,
        tag_variant_key: ValueSource<'static, 'b, str>,
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

    fn erased_seq_elem<'b, 'e: 'a>(&mut self, e: Source<'e, 'b>) -> Result {
        self.seq_elem(e)
    }
}

impl<'a, 'b> receiver::Receiver<'a> for Receiver<'a, 'b> {
    fn source<'v: 'a, V: source::Source<'v>>(&mut self, mut value: V) -> Result {
        self.erased_source(Source(&mut value))
    }

    fn display<D: fmt::Display>(&mut self, fmt: D) -> Result {
        self.0.erased_display(&fmt)
    }

    fn u8(&mut self, value: u8) -> Result {
        self.0.erased_u8(value)
    }

    fn u16(&mut self, value: u16) -> Result {
        self.0.erased_u16(value)
    }

    fn u32(&mut self, value: u32) -> Result {
        self.0.erased_u32(value)
    }

    fn u64(&mut self, value: u64) -> Result {
        self.0.erased_u64(value)
    }

    fn i8(&mut self, value: i8) -> Result {
        self.0.erased_i8(value)
    }

    fn i16(&mut self, value: i16) -> Result {
        self.0.erased_i16(value)
    }

    fn i32(&mut self, value: i32) -> Result {
        self.0.erased_i32(value)
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

    fn f32(&mut self, value: f32) -> Result {
        self.0.erased_f32(value)
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

    fn str<'s: 'a, T: source::ValueSource<'s, str>>(&mut self, mut value: T) -> Result {
        self.0.erased_str(ValueSource(&mut value))
    }

    fn char(&mut self, v: char) -> Result {
        self.0.erased_char(v)
    }

    fn error<'e: 'a, E: source::ValueSource<'e, dyn error::Error + 'static>>(
        &mut self,
        mut error: E,
    ) -> Result {
        self.0.erased_error(ValueSource(&mut error))
    }

    fn type_tag<T: source::ValueSource<'static, str>>(
        &mut self,
        mut tag: tag::TypeTag<T>,
    ) -> Result {
        self.0.erased_type_tag(ValueSource(&mut tag.ty))
    }

    fn variant_tag<T: source::ValueSource<'static, str>, K: source::ValueSource<'static, str>>(
        &mut self,
        mut tag: tag::VariantTag<T, K>,
    ) -> Result {
        self.0.erased_variant_tag(
            ValueSource(&mut tag.ty),
            ValueSource(&mut tag.variant_key),
            tag.variant_index,
        )
    }

    fn type_tagged_begin<T: source::ValueSource<'static, str>>(
        &mut self,
        mut tag: tag::TypeTag<T>,
    ) -> Result {
        self.0.erased_type_tagged_begin(ValueSource(&mut tag.ty))
    }

    fn type_tagged_end(&mut self) -> Result {
        self.0.erased_type_tagged_end()
    }

    fn variant_tagged_begin<
        T: source::ValueSource<'static, str>,
        K: source::ValueSource<'static, str>,
    >(
        &mut self,
        mut tag: tag::VariantTag<T, K>,
    ) -> Result {
        self.0.erased_variant_tagged_begin(
            ValueSource(&mut tag.ty),
            ValueSource(&mut tag.variant_key),
            tag.variant_index,
        )
    }

    fn variant_tagged_end(&mut self) -> Result {
        self.0.erased_variant_tagged_end()
    }

    fn type_tagged<'v: 'a, T: source::ValueSource<'static, str>, V: source::Source<'v>>(
        &mut self,
        mut tag: tag::TypeTag<T>,
        mut value: V,
    ) -> Result {
        self.0
            .erased_type_tagged(ValueSource(&mut tag.ty), Source(&mut value))
    }

    fn variant_tagged<
        'v: 'a,
        T: source::ValueSource<'static, str>,
        K: source::ValueSource<'static, str>,
        V: source::Source<'v>,
    >(
        &mut self,
        mut tag: tag::VariantTag<T, K>,
        mut value: V,
    ) -> Result {
        self.0.erased_variant_tagged(
            ValueSource(&mut tag.ty),
            ValueSource(&mut tag.variant_key),
            tag.variant_index,
            Source(&mut value),
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

    fn type_tagged_map_begin<T: source::ValueSource<'static, str>>(
        &mut self,
        mut tag: tag::TypeTag<T>,
        len: Option<usize>,
    ) -> Result {
        self.0
            .erased_type_tagged_map_begin(ValueSource(&mut tag.ty), len)
    }

    fn type_tagged_map_end(&mut self) -> Result {
        self.0.erased_type_tagged_map_end()
    }

    fn variant_tagged_map_begin<
        T: source::ValueSource<'static, str>,
        K: source::ValueSource<'static, str>,
    >(
        &mut self,
        mut tag: tag::VariantTag<T, K>,
        len: Option<usize>,
    ) -> Result {
        self.0.erased_variant_tagged_map_begin(
            ValueSource(&mut tag.ty),
            ValueSource(&mut tag.variant_key),
            tag.variant_index,
            len,
        )
    }

    fn variant_tagged_map_end(&mut self) -> Result {
        self.0.erased_variant_tagged_map_end()
    }

    fn map_entry<'k: 'a, 'v: 'a, K: source::Source<'k>, V: source::Source<'v>>(
        &mut self,
        mut key: K,
        mut value: V,
    ) -> Result {
        self.0
            .erased_map_entry(Source(&mut key), Source(&mut value))
    }

    fn map_field_entry<'v: 'a, F: source::ValueSource<'static, str>, V: source::Source<'v>>(
        &mut self,
        mut field: F,
        mut value: V,
    ) -> Result {
        self.0
            .erased_map_field_entry(ValueSource(&mut field), Source(&mut value))
    }

    fn map_key<'k: 'a, K: source::Source<'k>>(&mut self, mut key: K) -> Result {
        self.0.erased_map_key(Source(&mut key))
    }

    fn map_field<F: source::ValueSource<'static, str>>(&mut self, mut field: F) -> Result {
        self.0.erased_map_field(ValueSource(&mut field))
    }

    fn map_value<'v: 'a, V: source::Source<'v>>(&mut self, mut value: V) -> Result {
        self.0.erased_map_value(Source(&mut value))
    }

    fn seq_begin(&mut self, len: Option<usize>) -> Result {
        self.0.erased_seq_begin(len)
    }

    fn seq_end(&mut self) -> Result {
        self.0.erased_seq_end()
    }

    fn type_tagged_seq_begin<T: source::ValueSource<'static, str>>(
        &mut self,
        mut tag: tag::TypeTag<T>,
        len: Option<usize>,
    ) -> Result {
        self.0
            .erased_type_tagged_seq_begin(ValueSource(&mut tag.ty), len)
    }

    fn type_tagged_seq_end(&mut self) -> Result {
        self.0.erased_type_tagged_seq_end()
    }

    fn variant_tagged_seq_begin<
        T: source::ValueSource<'static, str>,
        K: source::ValueSource<'static, str>,
    >(
        &mut self,
        mut tag: tag::VariantTag<T, K>,
        len: Option<usize>,
    ) -> Result {
        self.0.erased_variant_tagged_seq_begin(
            ValueSource(&mut tag.ty),
            ValueSource(&mut tag.variant_key),
            tag.variant_index,
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

    fn seq_elem<'e: 'a, E: source::Source<'e>>(&mut self, mut elem: E) -> Result {
        self.0.erased_seq_elem(Source(&mut elem))
    }
}

pub struct Source<'a, 'b>(&'b mut dyn ErasedSource<'a>);

impl<'a, 'b> Source<'a, 'b> {
    pub fn new(source: &'b mut impl source::Source<'a>) -> Self {
        Source(source)
    }
}

pub fn source<'a, 'b>(source: &'b mut impl source::Source<'a>) -> Source<'a, 'b> {
    Source::new(source)
}

trait ErasedSource<'a> {
    fn erased_stream<'b>(&mut self, receiver: Receiver<'b, '_>) -> Result<Stream>
    where
        'a: 'b;

    fn erased_stream_to_end<'b>(&mut self, receiver: Receiver<'b, '_>) -> Result
    where
        'a: 'b;
}

impl<'a, T: source::Source<'a>> ErasedSource<'a> for T {
    fn erased_stream<'b>(&mut self, receiver: Receiver<'b, '_>) -> Result<Stream>
    where
        'a: 'b,
    {
        source::Source::stream(self, receiver)
    }

    fn erased_stream_to_end<'b>(&mut self, receiver: Receiver<'b, '_>) -> Result
    where
        'a: 'b,
    {
        source::Source::stream_to_end(self, receiver)
    }
}

impl<'a, 'b> source::Source<'a> for Source<'a, 'b> {
    fn stream<'c, S: receiver::Receiver<'c>>(&mut self, mut receiver: S) -> Result<Stream>
    where
        'a: 'c,
    {
        self.0.erased_stream(Receiver(&mut receiver))
    }

    fn stream_to_end<'c, S: receiver::Receiver<'c>>(&mut self, mut receiver: S) -> Result
    where
        'a: 'c,
    {
        self.0.erased_stream_to_end(Receiver(&mut receiver))
    }
}

pub struct ValueSource<'a, 'b, T: ?Sized, U: ?Sized = T>(&'b mut dyn ErasedValueSource<'a, T, U>);

impl<'a, 'b, T: value::Value + ?Sized, U: value::Value + ?Sized> ValueSource<'a, 'b, T, U> {
    pub fn new(source: &'b mut impl source::ValueSource<'a, T, U>) -> Self {
        ValueSource(source)
    }
}

pub fn value_source<'a, 'b, T: value::Value + ?Sized, U: value::Value + ?Sized>(
    source: &'b mut impl source::ValueSource<'a, T, U>,
) -> ValueSource<'a, 'b, T, U> {
    ValueSource::new(source)
}

trait ErasedValueSource<'a, T: value::Value + ?Sized, U: value::Value + ?Sized> {
    fn erased_stream<'b>(&mut self, receiver: Receiver<'b, '_>) -> Result<Stream>
    where
        'a: 'b;
    fn erased_stream_to_end<'b>(&mut self, receiver: Receiver<'b, '_>) -> Result
    where
        'a: 'b;

    fn erased_take(&mut self) -> Result<&T>;
    fn erased_take_ref(&mut self) -> Result<&'a U, Result<&T>>;
    fn erased_take_owned(&mut self) -> Result<T::Owned>
    where
        T: ToOwned,
        T::Owned: value::Value;
}

impl<'a, U: value::Value + ?Sized, V: value::Value + ?Sized, T: source::ValueSource<'a, U, V>>
    ErasedValueSource<'a, U, V> for T
{
    fn erased_stream<'b>(&mut self, receiver: Receiver<'b, '_>) -> Result<Stream>
    where
        'a: 'b,
    {
        source::Source::stream(self, receiver)
    }

    fn erased_stream_to_end<'b>(&mut self, receiver: Receiver<'b, '_>) -> Result
    where
        'a: 'b,
    {
        source::Source::stream_to_end(self, receiver)
    }

    fn erased_take(&mut self) -> Result<&U> {
        source::ValueSource::take(self).map_err(Into::into)
    }

    fn erased_take_ref(&mut self) -> Result<&'a V, Result<&U>> {
        source::ValueSource::take_ref(self).map_err(|e| e.into_result().map_err(Into::into))
    }

    fn erased_take_owned(&mut self) -> Result<U::Owned>
    where
        U: ToOwned,
        U::Owned: value::Value,
    {
        source::ValueSource::take_owned(self).map_err(Into::into)
    }
}

impl<'a, 'b, T: value::Value + ?Sized, U: value::Value + ?Sized> source::ValueSource<'a, T, U>
    for ValueSource<'a, 'b, T, U>
{
    type Error = Error;

    fn take(&mut self) -> Result<&T, source::TakeError<Self::Error>> {
        self.0.erased_take().map_err(source::TakeError::from_error)
    }

    fn take_ref<'c>(&'c mut self) -> Result<&'a U, source::TakeRefError<&'c T, Self::Error>> {
        self.0
            .erased_take_ref()
            .map_err(source::TakeRefError::from_result)
    }

    fn take_owned(&mut self) -> Result<T::Owned, source::TakeError<Self::Error>>
    where
        T: ToOwned,
        T::Owned: value::Value,
    {
        self.0
            .erased_take_owned()
            .map_err(source::TakeError::from_error)
    }
}

impl<'a, 'b, T: value::Value + ?Sized, U: value::Value + ?Sized> source::Source<'a>
    for ValueSource<'a, 'b, T, U>
{
    fn stream<'c, S: receiver::Receiver<'c>>(&mut self, mut receiver: S) -> Result<Stream>
    where
        'a: 'c,
    {
        self.0.erased_stream(Receiver(&mut receiver))
    }

    fn stream_to_end<'c, S: receiver::Receiver<'c>>(&mut self, mut receiver: S) -> Result
    where
        'a: 'c,
    {
        self.0.erased_stream_to_end(Receiver(&mut receiver))
    }
}
