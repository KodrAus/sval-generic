use crate::{
    data, for_all,
    source::{Source, ValueSource},
    Result, Value,
};

pub trait Receiver<'a> {
    fn unstructured<D: Display>(&mut self, fmt: D) -> Result;

    fn none(&mut self) -> Result;

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
        self.str(for_all(value.encode_utf8(&mut buf)))
    }

    fn str<'s: 'a, S: ValueSource<'s, str>>(&mut self, mut value: S) -> Result {
        self.unstructured(value.take()?)
    }

    fn digits<'n: 'a, N: ValueSource<'n, data::Digits>>(&mut self, mut value: N) -> Result {
        self.unstructured(value.take()?)
    }

    fn error<'e: 'a, E: ValueSource<'e, data::Error>>(&mut self, mut error: E) -> Result {
        self.unstructured(error.take()?)
    }

    fn bytes<'b: 'a, B: ValueSource<'b, data::Bytes>>(&mut self, mut value: B) -> Result {
        let bytes = value.take()?;

        self.seq_begin(Size::Variable(bytes.len()))?;

        for b in &**bytes {
            self.u8(*b)?;
        }

        self.seq_end()?;

        Ok(())
    }

    fn type_tag<T: ValueSource<'static, str>>(&mut self, tag: data::TypeTag<T>) -> Result {
        self.str(tag.ty)
    }

    fn variant_tag<T: ValueSource<'static, str>, K: ValueSource<'static, str>>(
        &mut self,
        tag: data::VariantTag<T, K>,
    ) -> Result {
        self.str(tag.variant_key)
    }

    fn source<'v: 'a, S: Source<'v>>(&mut self, mut source: S) -> Result {
        source.stream_to_end(self)
    }

    fn value<'v: 'a, V: Value + ?Sized + 'v>(&mut self, value: &'v V) -> Result {
        value.stream(self)
    }

    fn type_tagged_begin<T: ValueSource<'static, str>>(&mut self, tag: data::TypeTag<T>) -> Result {
        let _ = tag;
        Ok(())
    }

    fn type_tagged_end(&mut self) -> Result {
        Ok(())
    }

    fn variant_tagged_begin<T: ValueSource<'static, str>, K: ValueSource<'static, str>>(
        &mut self,
        tag: data::VariantTag<T, K>,
    ) -> Result {
        self.map_begin(Size::Fixed(1))?;
        self.map_key(tag)?;
        self.map_value_begin()
    }

    fn variant_tagged_end(&mut self) -> Result {
        self.map_value_end()?;
        self.map_end()
    }

    fn type_tagged<'v: 'a, T: ValueSource<'static, str>, V: Source<'v>>(
        &mut self,
        tag: data::TypeTag<T>,
        value: V,
    ) -> Result {
        self.type_tagged_begin(tag)?;
        self.source(value)?;
        self.type_tagged_end()
    }

    fn variant_tagged<
        'v: 'a,
        T: ValueSource<'static, str>,
        K: ValueSource<'static, str>,
        V: Source<'v>,
    >(
        &mut self,
        tag: data::VariantTag<T, K>,
        value: V,
    ) -> Result {
        self.variant_tagged_begin(tag)?;
        self.source(value)?;
        self.variant_tagged_end()
    }

    fn map_begin(&mut self, size: Size) -> Result;

    fn map_end(&mut self) -> Result;

    fn type_tagged_map_begin<T: ValueSource<'static, str>>(
        &mut self,
        tag: data::TypeTag<T>,
        size: Size,
    ) -> Result {
        self.type_tagged_begin(tag)?;
        self.map_begin(size)
    }

    fn type_tagged_map_end(&mut self) -> Result {
        self.map_end()?;
        self.type_tagged_end()
    }

    fn variant_tagged_map_begin<T: ValueSource<'static, str>, K: ValueSource<'static, str>>(
        &mut self,
        tag: data::VariantTag<T, K>,
        size: Size,
    ) -> Result {
        self.variant_tagged_begin(tag)?;
        self.map_begin(size)
    }

    fn variant_tagged_map_end(&mut self) -> Result {
        self.map_end()?;
        self.variant_tagged_end()
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

    fn seq_begin(&mut self, size: Size) -> Result;

    fn seq_end(&mut self) -> Result;

    fn type_tagged_seq_begin<T: ValueSource<'static, str>>(
        &mut self,
        tag: data::TypeTag<T>,
        size: Size,
    ) -> Result {
        self.type_tagged_begin(tag)?;
        self.seq_begin(size)
    }

    fn type_tagged_seq_end(&mut self) -> Result {
        self.seq_end()?;
        self.type_tagged_end()
    }

    fn variant_tagged_seq_begin<T: ValueSource<'static, str>, K: ValueSource<'static, str>>(
        &mut self,
        tag: data::VariantTag<T, K>,
        size: Size,
    ) -> Result {
        self.variant_tagged_begin(tag)?;
        self.seq_begin(size)
    }

    fn variant_tagged_seq_end(&mut self) -> Result {
        self.seq_end()?;
        self.variant_tagged_end()
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
    fn unstructured<D: Display>(&mut self, fmt: D) -> Result {
        (**self).unstructured(fmt)
    }

    fn none(&mut self) -> Result {
        (**self).none()
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

    fn digits<'n: 'a, N: ValueSource<'n, data::Digits>>(&mut self, value: N) -> Result {
        (**self).digits(value)
    }

    fn error<'e: 'a, E: ValueSource<'e, data::Error>>(&mut self, error: E) -> Result {
        (**self).error(error)
    }

    fn type_tag<T: ValueSource<'static, str>>(&mut self, tag: data::TypeTag<T>) -> Result {
        (**self).type_tag(tag)
    }

    fn variant_tag<T: ValueSource<'static, str>, K: ValueSource<'static, str>>(
        &mut self,
        tag: data::VariantTag<T, K>,
    ) -> Result {
        (**self).variant_tag(tag)
    }

    fn source<'v: 'a, S: Source<'v>>(&mut self, value: S) -> Result {
        (**self).source(value)
    }

    fn value<'v: 'a, V: Value + ?Sized + 'v>(&mut self, value: &'v V) -> Result {
        (**self).value(value)
    }

    fn type_tagged_begin<T: ValueSource<'static, str>>(&mut self, tag: data::TypeTag<T>) -> Result {
        (**self).type_tagged_begin(tag)
    }

    fn type_tagged_end(&mut self) -> Result {
        (**self).type_tagged_end()
    }

    fn variant_tagged_begin<T: ValueSource<'static, str>, K: ValueSource<'static, str>>(
        &mut self,
        tag: data::VariantTag<T, K>,
    ) -> Result {
        (**self).variant_tagged_begin(tag)
    }

    fn variant_tagged_end(&mut self) -> Result {
        (**self).variant_tagged_end()
    }

    fn type_tagged<'v: 'a, T: ValueSource<'static, str>, V: Source<'v>>(
        &mut self,
        tag: data::TypeTag<T>,
        value: V,
    ) -> Result {
        (**self).type_tagged(tag, value)
    }

    fn variant_tagged<
        'v: 'a,
        T: ValueSource<'static, str>,
        K: ValueSource<'static, str>,
        V: Source<'v>,
    >(
        &mut self,
        tag: data::VariantTag<T, K>,
        value: V,
    ) -> Result {
        (**self).variant_tagged(tag, value)
    }

    fn map_begin(&mut self, size: Size) -> Result {
        (**self).map_begin(size)
    }

    fn map_end(&mut self) -> Result {
        (**self).map_end()
    }

    fn type_tagged_map_begin<T: ValueSource<'static, str>>(
        &mut self,
        tag: data::TypeTag<T>,
        size: Size,
    ) -> Result {
        (**self).type_tagged_map_begin(tag, size)
    }

    fn type_tagged_map_end(&mut self) -> Result {
        (**self).type_tagged_map_end()
    }

    fn variant_tagged_map_begin<T: ValueSource<'static, str>, K: ValueSource<'static, str>>(
        &mut self,
        tag: data::VariantTag<T, K>,
        size: Size,
    ) -> Result {
        (**self).variant_tagged_map_begin(tag, size)
    }

    fn variant_tagged_map_end(&mut self) -> Result {
        (**self).variant_tagged_map_end()
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

    fn seq_begin(&mut self, size: Size) -> Result {
        (**self).seq_begin(size)
    }

    fn seq_end(&mut self) -> Result {
        (**self).seq_end()
    }

    fn type_tagged_seq_begin<T: ValueSource<'static, str>>(
        &mut self,
        tag: data::TypeTag<T>,
        size: Size,
    ) -> Result {
        (**self).type_tagged_seq_begin(tag, size)
    }

    fn type_tagged_seq_end(&mut self) -> Result {
        (**self).type_tagged_seq_end()
    }

    fn variant_tagged_seq_begin<T: ValueSource<'static, str>, K: ValueSource<'static, str>>(
        &mut self,
        tag: data::VariantTag<T, K>,
        size: Size,
    ) -> Result {
        (**self).variant_tagged_seq_begin(tag, size)
    }

    fn variant_tagged_seq_end(&mut self) -> Result {
        (**self).variant_tagged_seq_end()
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

pub enum Size {
    Unknown,
    Variable(usize),
    Fixed(usize),
}

impl Default for Size {
    fn default() -> Self {
        Size::Unknown
    }
}

impl Size {
    pub fn get(&self) -> Option<usize> {
        match self {
            Size::Unknown => None,
            Size::Variable(size) => Some(*size),
            Size::Fixed(size) => Some(*size),
        }
    }
}

pub use crate::std::fmt::Display;

pub fn unsupported() -> Result {
    Err(crate::Error)
}
