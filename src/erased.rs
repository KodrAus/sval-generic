use std::{error, fmt};

use crate::{source, stream, tag, value, Error, Result};

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
    fn erased_any<'b, 'v: 'a>(&mut self, v: Source<'v, 'b>) -> Result;
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
        e: TypedSource<'v, 'b, dyn error::Error + 'static>,
    ) -> Result;
    fn erased_str<'b, 'v: 'a>(&mut self, v: TypedSource<'v, 'b, str>) -> Result;
    fn erased_type_tag<'b>(&mut self, tag_ty: TypedSource<'static, 'b, str>) -> Result;
    fn erased_variant_tag<'b>(
        &mut self,
        tag_ty: TypedSource<'static, 'b, str>,
        tag_variant_key: TypedSource<'static, 'b, str>,
        tag_variant_index: Option<u64>,
    ) -> Result;
    fn erased_type_tagged_begin<'b>(&mut self, tag_ty: TypedSource<'static, 'b, str>) -> Result;
    fn erased_type_tagged_end(&mut self) -> Result;
    fn erased_variant_tagged_begin<'b>(
        &mut self,
        tag_ty: TypedSource<'static, 'b, str>,
        tag_variant_key: TypedSource<'static, 'b, str>,
        tag_variant_index: Option<u64>,
    ) -> Result;
    fn erased_variant_tagged_end(&mut self) -> Result;
    fn erased_type_tagged<'b, 'v: 'a>(
        &mut self,
        tag_ty: TypedSource<'static, 'b, str>,
        v: Source<'v, 'b>,
    ) -> Result;
    fn erased_variant_tagged<'b, 'v: 'a>(
        &mut self,
        tag_ty: TypedSource<'static, 'b, str>,
        tag_variant_key: TypedSource<'static, 'b, str>,
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
        tag_ty: TypedSource<'static, 'b, str>,
        len: Option<usize>,
    ) -> Result;
    fn erased_type_tagged_map_end(&mut self) -> Result;
    fn erased_variant_tagged_map_begin<'b>(
        &mut self,
        tag_ty: TypedSource<'static, 'b, str>,
        tag_variant_key: TypedSource<'static, 'b, str>,
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
    fn erased_map_field<'b>(&mut self, f: TypedSource<'static, 'b, str>) -> Result;
    fn erased_seq_begin(&mut self, len: Option<usize>) -> Result;
    fn erased_seq_end(&mut self) -> Result;
    fn erased_type_tagged_seq_begin<'b>(
        &mut self,
        tag_ty: TypedSource<'static, 'b, str>,
        len: Option<usize>,
    ) -> Result;
    fn erased_type_tagged_seq_end(&mut self) -> Result;
    fn erased_variant_tagged_seq_begin<'b>(
        &mut self,
        tag_ty: TypedSource<'static, 'b, str>,
        tag_variant_key: TypedSource<'static, 'b, str>,
        tag_variant_index: Option<u64>,
        len: Option<usize>,
    ) -> Result;
    fn erased_variant_tagged_seq_end(&mut self) -> Result;
    fn erased_seq_elem_begin(&mut self) -> Result;
    fn erased_seq_elem_end(&mut self) -> Result;
    fn erased_seq_elem<'b, 'e: 'a>(&mut self, e: Source<'e, 'b>) -> Result;
}

impl<'a, T: stream::Stream<'a> + ?Sized> ErasedStream<'a> for T {
    fn erased_any<'b, 'v: 'a>(&mut self, v: Source<'v, 'b>) -> Result {
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
        e: TypedSource<'v, 'b, dyn error::Error + 'static>,
    ) -> Result {
        self.error(e)
    }

    fn erased_str<'b, 'v: 'a>(&mut self, v: TypedSource<'v, 'b, str>) -> Result {
        self.str(v)
    }

    fn erased_type_tag<'b>(&mut self, tag_ty: TypedSource<'static, 'b, str>) -> Result {
        self.type_tag(tag::TypeTag::new(tag_ty))
    }

    fn erased_variant_tag<'b>(
        &mut self,
        tag_ty: TypedSource<'static, 'b, str>,
        tag_variant_key: TypedSource<'static, 'b, str>,
        tag_variant_index: Option<u64>,
    ) -> Result {
        self.variant_tag(tag::VariantTag::new(
            tag_ty,
            tag_variant_key,
            tag_variant_index,
        ))
    }

    fn erased_type_tagged_begin<'b>(&mut self, tag_ty: TypedSource<'static, 'b, str>) -> Result {
        self.type_tagged_begin(tag::TypeTag::new(tag_ty))
    }

    fn erased_type_tagged_end(&mut self) -> Result {
        self.type_tagged_end()
    }

    fn erased_variant_tagged_begin<'b>(
        &mut self,
        tag_ty: TypedSource<'static, 'b, str>,
        tag_variant_key: TypedSource<'static, 'b, str>,
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
        tag_ty: TypedSource<'static, 'b, str>,
        v: Source<'v, 'b>,
    ) -> Result {
        self.type_tagged(tag::TypeTag::new(tag_ty), v)
    }

    fn erased_variant_tagged<'b, 'v: 'a>(
        &mut self,
        tag_ty: TypedSource<'static, 'b, str>,
        tag_variant_key: TypedSource<'static, 'b, str>,
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
        tag_ty: TypedSource<'static, 'b, str>,
        len: Option<usize>,
    ) -> Result {
        self.type_tagged_map_begin(tag::TypeTag::new(tag_ty), len)
    }

    fn erased_type_tagged_map_end(&mut self) -> Result {
        self.type_tagged_map_end()
    }

    fn erased_variant_tagged_map_begin<'b>(
        &mut self,
        tag_ty: TypedSource<'static, 'b, str>,
        tag_variant_key: TypedSource<'static, 'b, str>,
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

    fn erased_map_field<'b>(&mut self, f: TypedSource<'static, 'b, str>) -> Result {
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
        tag_ty: TypedSource<'static, 'b, str>,
        len: Option<usize>,
    ) -> Result {
        self.type_tagged_seq_begin(tag::TypeTag::new(tag_ty), len)
    }

    fn erased_type_tagged_seq_end(&mut self) -> Result {
        self.type_tagged_seq_end()
    }

    fn erased_variant_tagged_seq_begin<'b>(
        &mut self,
        tag_ty: TypedSource<'static, 'b, str>,
        tag_variant_key: TypedSource<'static, 'b, str>,
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

impl<'a, 'b> stream::Stream<'a> for Stream<'a, 'b> {
    fn any<'v: 'a, V: source::Source<'v>>(&mut self, mut value: V) -> Result {
        self.erased_any(Source(&mut value))
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

    fn str<'s: 'a, T: source::TypedSource<'s, str>>(&mut self, mut value: T) -> Result {
        self.0.erased_str(TypedSource(&mut value))
    }

    fn error<'e: 'a, E: source::TypedSource<'e, dyn error::Error + 'static>>(
        &mut self,
        mut error: E,
    ) -> Result {
        self.0.erased_error(TypedSource(&mut error))
    }

    fn type_tag<T: source::TypedSource<'static, str>>(
        &mut self,
        mut tag: tag::TypeTag<T>,
    ) -> Result {
        self.0.erased_type_tag(TypedSource(&mut tag.ty))
    }

    fn variant_tag<T: source::TypedSource<'static, str>, K: source::TypedSource<'static, str>>(
        &mut self,
        mut tag: tag::VariantTag<T, K>,
    ) -> Result {
        self.0.erased_variant_tag(
            TypedSource(&mut tag.ty),
            TypedSource(&mut tag.variant_key),
            tag.variant_index,
        )
    }

    fn type_tagged_begin<T: source::TypedSource<'static, str>>(
        &mut self,
        mut tag: tag::TypeTag<T>,
    ) -> Result {
        self.0.erased_type_tagged_begin(TypedSource(&mut tag.ty))
    }

    fn type_tagged_end(&mut self) -> Result {
        self.0.erased_type_tagged_end()
    }

    fn variant_tagged_begin<
        T: source::TypedSource<'static, str>,
        K: source::TypedSource<'static, str>,
    >(
        &mut self,
        mut tag: tag::VariantTag<T, K>,
    ) -> Result {
        self.0.erased_variant_tagged_begin(
            TypedSource(&mut tag.ty),
            TypedSource(&mut tag.variant_key),
            tag.variant_index,
        )
    }

    fn variant_tagged_end(&mut self) -> Result {
        self.0.erased_variant_tagged_end()
    }

    fn type_tagged<'v: 'a, T: source::TypedSource<'static, str>, V: source::Source<'v>>(
        &mut self,
        mut tag: tag::TypeTag<T>,
        mut value: V,
    ) -> Result {
        self.0
            .erased_type_tagged(TypedSource(&mut tag.ty), Source(&mut value))
    }

    fn variant_tagged<
        'v: 'a,
        T: source::TypedSource<'static, str>,
        K: source::TypedSource<'static, str>,
        V: source::Source<'v>,
    >(
        &mut self,
        mut tag: tag::VariantTag<T, K>,
        mut value: V,
    ) -> Result {
        self.0.erased_variant_tagged(
            TypedSource(&mut tag.ty),
            TypedSource(&mut tag.variant_key),
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

    fn type_tagged_map_begin<T: source::TypedSource<'static, str>>(
        &mut self,
        mut tag: tag::TypeTag<T>,
        len: Option<usize>,
    ) -> Result {
        self.0
            .erased_type_tagged_map_begin(TypedSource(&mut tag.ty), len)
    }

    fn type_tagged_map_end(&mut self) -> Result {
        self.0.erased_type_tagged_map_end()
    }

    fn variant_tagged_map_begin<
        T: source::TypedSource<'static, str>,
        K: source::TypedSource<'static, str>,
    >(
        &mut self,
        mut tag: tag::VariantTag<T, K>,
        len: Option<usize>,
    ) -> Result {
        self.0.erased_variant_tagged_map_begin(
            TypedSource(&mut tag.ty),
            TypedSource(&mut tag.variant_key),
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

    fn map_key<'k: 'a, K: source::Source<'k>>(&mut self, mut key: K) -> Result {
        self.0.erased_map_key(Source(&mut key))
    }

    fn map_field<F: source::TypedSource<'static, str>>(&mut self, mut field: F) -> Result {
        self.0.erased_map_field(TypedSource(&mut field))
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

    fn type_tagged_seq_begin<T: source::TypedSource<'static, str>>(
        &mut self,
        mut tag: tag::TypeTag<T>,
        len: Option<usize>,
    ) -> Result {
        self.0
            .erased_type_tagged_seq_begin(TypedSource(&mut tag.ty), len)
    }

    fn type_tagged_seq_end(&mut self) -> Result {
        self.0.erased_type_tagged_seq_end()
    }

    fn variant_tagged_seq_begin<
        T: source::TypedSource<'static, str>,
        K: source::TypedSource<'static, str>,
    >(
        &mut self,
        mut tag: tag::VariantTag<T, K>,
        len: Option<usize>,
    ) -> Result {
        self.0.erased_variant_tagged_seq_begin(
            TypedSource(&mut tag.ty),
            TypedSource(&mut tag.variant_key),
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

struct Source<'a, 'b>(&'b mut dyn ErasedSource<'a>);

trait ErasedSource<'a> {
    fn erased_stream<'b>(&mut self, stream: Stream<'b, '_>) -> Result
    where
        'a: 'b;
}

impl<'a, T: source::Source<'a>> ErasedSource<'a> for T {
    fn erased_stream<'b>(&mut self, stream: Stream<'b, '_>) -> Result
    where
        'a: 'b,
    {
        source::Source::stream(self, stream)
    }
}

impl<'a, 'b> source::Source<'a> for Source<'a, 'b> {
    fn stream<'c, S: stream::Stream<'c>>(&mut self, mut stream: S) -> Result
    where
        'a: 'c,
    {
        self.0.erased_stream(Stream(&mut stream))
    }
}

struct TypedSource<'a, 'b, T: ?Sized>(&'b mut dyn ErasedTypedSource<'a, T>);

trait ErasedTypedSource<'a, T: ?Sized> {
    fn erased_stream<'b>(&mut self, stream: Stream<'b, '_>) -> Result
    where
        'a: 'b;

    fn erased_stream_to_value(&mut self) -> Result<&T>;
    fn erased_stream_to_ref(&mut self) -> Result<&'a T, Result<&T>>;
}

impl<'a, U: value::Value + ?Sized + 'static, T: source::TypedSource<'a, U>> ErasedTypedSource<'a, U>
    for T
{
    fn erased_stream<'b>(&mut self, stream: Stream<'b, '_>) -> Result
    where
        'a: 'b,
    {
        source::Source::stream(self, stream)
    }

    fn erased_stream_to_value(&mut self) -> Result<&U> {
        self.stream_to_value().map_err(Into::into)
    }

    fn erased_stream_to_ref(&mut self) -> Result<&'a U, Result<&U>> {
        self.stream_to_ref()
            .map_err(|e| e.into_result().map_err(Into::into))
    }
}

impl<'a, 'b, T: value::Value + ?Sized + 'static> source::TypedSource<'a, T>
    for TypedSource<'a, 'b, T>
{
    type Error = Error;

    fn stream_to_value(&mut self) -> Result<&T, source::ToValueError<Self::Error>> {
        self.0
            .erased_stream_to_value()
            .map_err(source::ToValueError::from_error)
    }

    fn stream_to_ref<'c>(&'c mut self) -> Result<&'a T, source::ToRefError<&'c T, Self::Error>> {
        self.0
            .erased_stream_to_ref()
            .map_err(source::ToRefError::from_result)
    }
}

impl<'a, 'b, T: ?Sized> source::Source<'a> for TypedSource<'a, 'b, T> {
    fn stream<'c, S: stream::Stream<'c>>(&mut self, mut stream: S) -> Result
    where
        'a: 'c,
    {
        self.0.erased_stream(Stream(&mut stream))
    }
}
