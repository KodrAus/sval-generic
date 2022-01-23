use crate::{
    data, for_all,
    source::{Source, ValueSource},
    std::ops::Deref,
    Result, Value,
};

pub trait Receiver<'a> {
    fn source<'v: 'a, S: Source<'v>>(&mut self, mut source: S) -> Result {
        source.stream_to_end(self)
    }

    fn value<'v: 'a, V: Value + ?Sized + 'v>(&mut self, value: &'v V) -> Result {
        value.stream(self)
    }

    fn unstructured<D: data::Display>(&mut self, fmt: D) -> Result;

    fn none(&mut self) -> Result;

    fn some<'v: 'a, V: Source<'v>>(&mut self, value: V) -> Result {
        self.source(value)
    }

    fn u8(&mut self, value: u8) -> Result {
        self.u16(value as u16)
    }

    fn u16(&mut self, value: u16) -> Result {
        self.u32(value as u32)
    }

    fn u32(&mut self, value: u32) -> Result {
        self.u64(value as u64)
    }

    fn u64(&mut self, value: u64) -> Result {
        self.u128(value as u128)
    }

    fn i8(&mut self, value: i8) -> Result {
        self.i16(value as i16)
    }

    fn i16(&mut self, value: i16) -> Result {
        self.i32(value as i32)
    }

    fn i32(&mut self, value: i32) -> Result {
        self.i64(value as i64)
    }

    fn i64(&mut self, value: i64) -> Result {
        self.i128(value as i128)
    }

    fn u128(&mut self, value: u128) -> Result {
        self.digits(value)
    }

    fn i128(&mut self, value: i128) -> Result {
        self.digits(value)
    }

    fn f32(&mut self, value: f32) -> Result {
        self.f64(value as f64)
    }

    fn f64(&mut self, value: f64) -> Result {
        self.digits(value)
    }

    fn bool(&mut self, value: bool) -> Result {
        self.unstructured(value)
    }

    fn char(&mut self, value: char) -> Result {
        let mut buf = [0; 4];
        self.str(for_all(&*value.encode_utf8(&mut buf)))
    }

    fn str<'s: 'a, S: ValueSource<'s, str>>(&mut self, mut value: S) -> Result {
        self.unstructured(value.take()?)
    }

    fn tagged_str<'s: 'a, T: ValueSource<'static, str>, S: ValueSource<'s, str>>(
        &mut self,
        tag: data::Tag<T>,
        value: S,
    ) -> Result {
        self.tagged(tag, value)
    }

    fn digits<'d: 'a, D: ValueSource<'d, data::Digits>>(&mut self, mut value: D) -> Result {
        self.unstructured(value.take()?)
    }

    fn error<'e: 'a, E: ValueSource<'e, data::Error>>(&mut self, mut error: E) -> Result {
        self.unstructured(error.take()?)
    }

    fn bytes<'s: 'a, B: ValueSource<'s, data::Bytes>>(&mut self, mut bytes: B) -> Result {
        self.unstructured(bytes.take()?)
    }

    fn tagged_bytes<'s: 'a, T: ValueSource<'static, str>, B: ValueSource<'s, data::Bytes>>(
        &mut self,
        tag: data::Tag<T>,
        value: B,
    ) -> Result {
        self.tagged(tag, value)
    }

    fn tag<T: ValueSource<'static, str>>(&mut self, tag: data::Tag<T>) -> Result {
        self.str(tag.label)
    }

    fn tag_variant<T: ValueSource<'static, str>, K: ValueSource<'static, str>>(
        &mut self,
        type_tag: data::Tag<T>,
        variant_tag: data::Tag<K>,
        variant_index: Option<u64>,
    ) -> Result {
        let _ = type_tag;
        let _ = variant_index;

        self.str(variant_tag.label)
    }

    fn tagged_begin<T: ValueSource<'static, str>>(&mut self, tag: data::Tag<T>) -> Result {
        let _ = tag;
        Ok(())
    }

    fn tagged_end(&mut self) -> Result {
        Ok(())
    }

    fn tagged_variant_begin<T: ValueSource<'static, str>, K: ValueSource<'static, str>>(
        &mut self,
        mut type_tag: data::Tag<T>,
        variant_tag: data::Tag<K>,
        variant_index: Option<u64>,
    ) -> Result {
        let _ = variant_index;

        self.tagged_map_begin(type_tag, Some(1))?;
        self.map_key(variant_tag)?;
        self.map_value_begin()
    }

    fn tagged_variant_end(&mut self) -> Result {
        self.map_value_end()?;
        self.tagged_map_end()
    }

    fn tagged<'v: 'a, T: ValueSource<'static, str>, V: Source<'v>>(
        &mut self,
        tag: data::Tag<T>,
        value: V,
    ) -> Result {
        self.tagged_begin(tag)?;
        self.source(value)?;
        self.tagged_end()
    }

    fn tagged_variant<
        'v: 'a,
        T: ValueSource<'static, str>,
        K: ValueSource<'static, str>,
        V: Source<'v>,
    >(
        &mut self,
        type_tag: data::Tag<T>,
        variant_tag: data::Tag<K>,
        variant_index: Option<u64>,
        value: V,
    ) -> Result {
        self.tagged_variant_begin(type_tag, variant_tag, variant_index)?;
        self.source(value)?;
        self.tagged_variant_end()
    }

    fn map_begin(&mut self, size: Option<usize>) -> Result;

    fn map_end(&mut self) -> Result;

    fn tagged_map_begin<T: ValueSource<'static, str>>(
        &mut self,
        tag: data::Tag<T>,
        size: Option<usize>,
    ) -> Result {
        self.tagged_begin(tag)?;
        self.map_begin(size)
    }

    fn tagged_map_end(&mut self) -> Result {
        self.map_end()?;
        self.tagged_end()
    }

    fn tagged_variant_map_begin<T: ValueSource<'static, str>, K: ValueSource<'static, str>>(
        &mut self,
        type_tag: data::Tag<T>,
        variant_tag: data::Tag<K>,
        variant_index: Option<u64>,
        size: Option<usize>,
    ) -> Result {
        self.tagged_variant_begin(type_tag, variant_tag, variant_index)?;
        self.map_begin(size)
    }

    fn tagged_variant_map_end(&mut self) -> Result {
        self.map_end()?;
        self.tagged_variant_end()
    }

    fn map_key_begin(&mut self) -> Result;

    fn map_key_end(&mut self) -> Result;

    fn map_value_begin(&mut self) -> Result;

    fn map_value_end(&mut self) -> Result;

    fn map_entry<'k: 'a, 'v: 'a, K: Source<'k>, V: Source<'v>>(
        &mut self,
        key: K,
        value: V,
    ) -> Result {
        self.map_key(key)?;
        self.map_value(value)
    }

    fn map_field_entry<'v: 'a, F: ValueSource<'static, str>, V: Source<'v>>(
        &mut self,
        field: F,
        value: V,
    ) -> Result {
        self.map_field(field)?;
        self.map_value(value)
    }

    fn map_field<F: ValueSource<'static, str>>(&mut self, field: F) -> Result {
        self.map_key(field)
    }

    fn map_key<'k: 'a, K: Source<'k>>(&mut self, key: K) -> Result {
        self.map_key_begin()?;
        self.source(key)?;
        self.map_key_end()
    }

    fn map_value<'v: 'a, V: Source<'v>>(&mut self, value: V) -> Result {
        self.map_value_begin()?;
        self.source(value)?;
        self.map_value_end()
    }

    fn seq_begin(&mut self, size: Option<usize>) -> Result;

    fn seq_end(&mut self) -> Result;

    fn tagged_seq_begin<T: ValueSource<'static, str>>(
        &mut self,
        tag: data::Tag<T>,
        size: Option<usize>,
    ) -> Result {
        self.tagged_begin(tag)?;
        self.seq_begin(size)
    }

    fn tagged_seq_end(&mut self) -> Result {
        self.seq_end()?;
        self.tagged_end()
    }

    fn tagged_variant_seq_begin<T: ValueSource<'static, str>, K: ValueSource<'static, str>>(
        &mut self,
        type_tag: data::Tag<T>,
        variant_tag: data::Tag<K>,
        variant_index: Option<u64>,
        size: Option<usize>,
    ) -> Result {
        self.tagged_variant_begin(type_tag, variant_tag, variant_index)?;
        self.seq_begin(size)
    }

    fn tagged_variant_seq_end(&mut self) -> Result {
        self.seq_end()?;
        self.tagged_variant_end()
    }

    fn seq_elem_begin(&mut self) -> Result;

    fn seq_elem_end(&mut self) -> Result;

    fn seq_elem<'e: 'a, E: Source<'e>>(&mut self, elem: E) -> Result {
        self.seq_elem_begin()?;
        self.source(elem)?;
        self.seq_elem_end()
    }
}

impl<'a, 'b, R: ?Sized> Receiver<'a> for &'b mut R
where
    R: Receiver<'a>,
{
    fn unstructured<D: data::Display>(&mut self, fmt: D) -> Result {
        (**self).unstructured(fmt)
    }

    fn none(&mut self) -> Result {
        (**self).none()
    }

    fn some<'v: 'a, V: Source<'v>>(&mut self, value: V) -> Result {
        (**self).some(value)
    }

    fn u8(&mut self, value: u8) -> Result {
        (**self).u8(value)
    }

    fn u16(&mut self, value: u16) -> Result {
        (**self).u16(value)
    }

    fn u32(&mut self, value: u32) -> Result {
        (**self).u32(value)
    }

    fn u64(&mut self, value: u64) -> Result {
        (**self).u64(value)
    }

    fn i8(&mut self, value: i8) -> Result {
        (**self).i8(value)
    }

    fn i16(&mut self, value: i16) -> Result {
        (**self).i16(value)
    }

    fn i32(&mut self, value: i32) -> Result {
        (**self).i32(value)
    }

    fn i64(&mut self, value: i64) -> Result {
        (**self).i64(value)
    }

    fn u128(&mut self, value: u128) -> Result {
        (**self).u128(value)
    }

    fn i128(&mut self, value: i128) -> Result {
        (**self).i128(value)
    }

    fn f32(&mut self, value: f32) -> Result {
        (**self).f32(value)
    }

    fn f64(&mut self, value: f64) -> Result {
        (**self).f64(value)
    }

    fn bool(&mut self, value: bool) -> Result {
        (**self).bool(value)
    }

    fn char(&mut self, value: char) -> Result {
        (**self).char(value)
    }

    fn str<'s: 'a, S: ValueSource<'s, str>>(&mut self, value: S) -> Result {
        (**self).str(value)
    }

    fn digits<'d: 'a, D: ValueSource<'d, data::Digits>>(&mut self, value: D) -> Result {
        (**self).digits(value)
    }

    fn error<'e: 'a, E: ValueSource<'e, data::Error>>(&mut self, error: E) -> Result {
        (**self).error(error)
    }

    fn bytes<'s: 'a, B: ValueSource<'s, data::Bytes>>(&mut self, bytes: B) -> Result {
        (**self).bytes(bytes)
    }

    fn tag<T: ValueSource<'static, str>>(&mut self, tag: data::Tag<T>) -> Result {
        (**self).tag(tag)
    }

    fn tag_variant<T: ValueSource<'static, str>, K: ValueSource<'static, str>>(
        &mut self,
        type_tag: data::Tag<T>,
        variant_tag: data::Tag<K>,
        variant_index: Option<u64>,
    ) -> Result {
        (**self).tag_variant(type_tag, variant_tag, variant_index)
    }

    fn source<'v: 'a, S: Source<'v>>(&mut self, value: S) -> Result {
        (**self).source(value)
    }

    fn value<'v: 'a, V: Value + ?Sized + 'v>(&mut self, value: &'v V) -> Result {
        (**self).value(value)
    }

    fn tagged_begin<T: ValueSource<'static, str>>(&mut self, tag: data::Tag<T>) -> Result {
        (**self).tagged_begin(tag)
    }

    fn tagged_end(&mut self) -> Result {
        (**self).tagged_end()
    }

    fn tagged_variant_begin<T: ValueSource<'static, str>, K: ValueSource<'static, str>>(
        &mut self,
        type_tag: data::Tag<T>,
        variant_tag: data::Tag<K>,
        variant_index: Option<u64>,
    ) -> Result {
        (**self).tagged_variant_begin(type_tag, variant_tag, variant_index)
    }

    fn tagged_variant_end(&mut self) -> Result {
        (**self).tagged_variant_end()
    }

    fn tagged<'v: 'a, T: ValueSource<'static, str>, V: Source<'v>>(
        &mut self,
        tag: data::Tag<T>,
        value: V,
    ) -> Result {
        (**self).tagged(tag, value)
    }

    fn tagged_variant<
        'v: 'a,
        T: ValueSource<'static, str>,
        K: ValueSource<'static, str>,
        V: Source<'v>,
    >(
        &mut self,
        type_tag: data::Tag<T>,
        variant_tag: data::Tag<K>,
        variant_index: Option<u64>,
        value: V,
    ) -> Result {
        (**self).tagged_variant(type_tag, variant_tag, variant_index, value)
    }

    fn map_begin(&mut self, size: Option<usize>) -> Result {
        (**self).map_begin(size)
    }

    fn map_end(&mut self) -> Result {
        (**self).map_end()
    }

    fn tagged_map_begin<T: ValueSource<'static, str>>(
        &mut self,
        tag: data::Tag<T>,
        size: Option<usize>,
    ) -> Result {
        (**self).tagged_map_begin(tag, size)
    }

    fn tagged_map_end(&mut self) -> Result {
        (**self).tagged_map_end()
    }

    fn tagged_variant_map_begin<T: ValueSource<'static, str>, K: ValueSource<'static, str>>(
        &mut self,
        type_tag: data::Tag<T>,
        variant_tag: data::Tag<K>,
        variant_index: Option<u64>,
        size: Option<usize>,
    ) -> Result {
        (**self).tagged_variant_map_begin(type_tag, variant_tag, variant_index, size)
    }

    fn tagged_variant_map_end(&mut self) -> Result {
        (**self).tagged_variant_map_end()
    }

    fn map_key_begin(&mut self) -> Result {
        (**self).map_key_begin()
    }

    fn map_key_end(&mut self) -> Result {
        (**self).map_key_end()
    }

    fn map_value_begin(&mut self) -> Result {
        (**self).map_value_begin()
    }

    fn map_value_end(&mut self) -> Result {
        (**self).map_value_end()
    }

    fn map_entry<'k: 'a, 'v: 'a, K: Source<'k>, V: Source<'v>>(
        &mut self,
        key: K,
        value: V,
    ) -> Result {
        (**self).map_entry(key, value)
    }

    fn map_field_entry<'v: 'a, F: ValueSource<'static, str>, V: Source<'v>>(
        &mut self,
        field: F,
        value: V,
    ) -> Result {
        (**self).map_field_entry(field, value)
    }

    fn map_field<F: ValueSource<'static, str>>(&mut self, field: F) -> Result {
        (**self).map_field(field)
    }

    fn map_key<'k: 'a, K: Source<'k>>(&mut self, key: K) -> Result {
        (**self).map_key(key)
    }

    fn map_value<'v: 'a, V: Source<'v>>(&mut self, value: V) -> Result {
        (**self).map_value(value)
    }

    fn seq_begin(&mut self, size: Option<usize>) -> Result {
        (**self).seq_begin(size)
    }

    fn seq_end(&mut self) -> Result {
        (**self).seq_end()
    }

    fn tagged_seq_begin<T: ValueSource<'static, str>>(
        &mut self,
        tag: data::Tag<T>,
        size: Option<usize>,
    ) -> Result {
        (**self).tagged_seq_begin(tag, size)
    }

    fn tagged_seq_end(&mut self) -> Result {
        (**self).tagged_seq_end()
    }

    fn tagged_variant_seq_begin<T: ValueSource<'static, str>, K: ValueSource<'static, str>>(
        &mut self,
        type_tag: data::Tag<T>,
        variant_tag: data::Tag<K>,
        variant_index: Option<u64>,
        size: Option<usize>,
    ) -> Result {
        (**self).tagged_variant_seq_begin(type_tag, variant_tag, variant_index, size)
    }

    fn tagged_variant_seq_end(&mut self) -> Result {
        (**self).tagged_variant_seq_end()
    }

    fn seq_elem_begin(&mut self) -> Result {
        (**self).seq_elem_begin()
    }

    fn seq_elem_end(&mut self) -> Result {
        (**self).seq_elem_end()
    }

    fn seq_elem<'e: 'a, E: Source<'e>>(&mut self, elem: E) -> Result {
        (**self).seq_elem(elem)
    }
}

pub fn unsupported() -> Result {
    Err(crate::Error)
}
