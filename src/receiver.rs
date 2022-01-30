use crate::data::{Bytes, Error, Tag, Tagged, Text};
use crate::{
    data,
    source::{self, Source, ValueSource},
    Result, Value,
};
use core::fmt::Display;

pub trait Receiver<'a> {
    fn value<'v: 'a, V: Value + ?Sized + 'v>(&mut self, value: &'v V) -> Result {
        value.stream(self)
    }

    fn unstructured<D: data::Display>(&mut self, fmt: D) -> Result;

    fn null(&mut self) -> Result;

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

    fn u128(&mut self, value: u128) -> Result {
        self.unstructured(value)
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

    fn i128(&mut self, value: i128) -> Result {
        self.unstructured(value)
    }

    fn f32(&mut self, value: f32) -> Result {
        self.f64(value as f64)
    }

    fn f64(&mut self, value: f64) -> Result {
        self.unstructured(value)
    }

    fn bool(&mut self, value: bool) -> Result {
        self.unstructured(value)
    }

    fn char(&mut self, value: char) -> Result {
        let mut buf = [0; 4];
        self.unstructured(&*value.encode_utf8(&mut buf))
    }

    fn str<'s: 'a, S: ValueSource<'s, str>>(&mut self, mut value: S) -> Result {
        self.unstructured(value.take()?)
    }

    fn text<'s: 'a, S: ValueSource<'s, data::Text>>(&mut self, mut value: S) -> Result {
        self.unstructured(value.take()?)
    }

    fn error<'e: 'a, E: ValueSource<'e, data::Error>>(&mut self, mut error: E) -> Result {
        self.unstructured(error.take()?)
    }

    fn bytes<'s: 'a, B: ValueSource<'s, data::Bytes>>(&mut self, mut bytes: B) -> Result {
        self.unstructured(bytes.take()?)
    }

    fn tag<T: ValueSource<'static, str>>(&mut self, mut tag: data::Tag<T>) -> Result {
        if let Some(label) = tag.label_mut() {
            self.str(label)?;
            return Ok(());
        }

        if let Some(id) = tag.id() {
            self.u64(id)?;
            return Ok(());
        }

        self.null()
    }

    fn tagged_begin<T: ValueSource<'static, str>>(&mut self, tag: data::Tag<T>) -> Result {
        let _ = tag;
        Ok(())
    }

    fn tagged_end<T: ValueSource<'static, str>>(&mut self, tag: data::Tag<T>) -> Result {
        let _ = tag;
        Ok(())
    }

    fn tagged<'v: 'a, T: ValueSource<'static, str>, V: Source<'v>>(
        &mut self,
        mut tagged: data::Tagged<T, V>,
    ) -> Result {
        tagged.stream_to_end(self)
    }

    fn tagged_str<'s: 'a, T: ValueSource<'static, str>, S: ValueSource<'s, str>>(
        &mut self,
        tagged: data::Tagged<T, S>,
    ) -> Result {
        self.tagged_text(tagged.map_value(|v| data::text::text_value_source(v)))
    }

    fn tagged_text<'s: 'a, T: ValueSource<'static, str>, S: ValueSource<'s, data::Text>>(
        &mut self,
        mut tagged: data::Tagged<T, S>,
    ) -> Result {
        self.tagged_begin(tagged.begin_tag_mut())?;
        tagged.value_mut().stream_to_end(&mut *self)?;
        self.tagged_end(tagged.end_tag_mut())
    }

    fn tagged_bytes<'s: 'a, T: ValueSource<'static, str>, B: ValueSource<'s, data::Bytes>>(
        &mut self,
        mut tagged: data::Tagged<T, B>,
    ) -> Result {
        self.tagged_begin(tagged.begin_tag_mut())?;
        tagged.value_mut().stream_to_end(&mut *self)?;
        self.tagged_end(tagged.end_tag_mut())
    }

    fn map_begin(&mut self, size: Option<u64>) -> Result;

    fn map_end(&mut self) -> Result;

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

    fn map_key<'k: 'a, K: Source<'k>>(&mut self, mut key: K) -> Result {
        self.map_key_begin()?;
        key.stream_to_end(&mut *self)?;
        self.map_key_end()
    }

    fn map_value<'v: 'a, V: Source<'v>>(&mut self, mut value: V) -> Result {
        self.map_value_begin()?;
        value.stream_to_end(&mut *self)?;
        self.map_value_end()
    }

    fn seq_begin(&mut self, size: Option<u64>) -> Result;

    fn seq_end(&mut self) -> Result;

    fn seq_elem_begin(&mut self) -> Result;

    fn seq_elem_end(&mut self) -> Result;

    fn seq_elem<'e: 'a, E: Source<'e>>(&mut self, mut elem: E) -> Result {
        self.seq_elem_begin()?;
        elem.stream_to_end(&mut *self)?;
        self.seq_elem_end()
    }
}

impl<'a, 'b, R: ?Sized> Receiver<'a> for &'b mut R
where
    R: Receiver<'a>,
{
    fn value<'v: 'a, V: Value + ?Sized + 'v>(&mut self, value: &'v V) -> Result {
        todo!()
    }

    fn unstructured<D: Display>(&mut self, fmt: D) -> Result {
        todo!()
    }

    fn null(&mut self) -> Result {
        todo!()
    }

    fn u8(&mut self, value: u8) -> Result {
        todo!()
    }

    fn u16(&mut self, value: u16) -> Result {
        todo!()
    }

    fn u32(&mut self, value: u32) -> Result {
        todo!()
    }

    fn u64(&mut self, value: u64) -> Result {
        todo!()
    }

    fn u128(&mut self, value: u128) -> Result {
        todo!()
    }

    fn i8(&mut self, value: i8) -> Result {
        todo!()
    }

    fn i16(&mut self, value: i16) -> Result {
        todo!()
    }

    fn i32(&mut self, value: i32) -> Result {
        todo!()
    }

    fn i64(&mut self, value: i64) -> Result {
        todo!()
    }

    fn i128(&mut self, value: i128) -> Result {
        todo!()
    }

    fn f32(&mut self, value: f32) -> Result {
        todo!()
    }

    fn f64(&mut self, value: f64) -> Result {
        todo!()
    }

    fn bool(&mut self, value: bool) -> Result {
        todo!()
    }

    fn char(&mut self, value: char) -> Result {
        todo!()
    }

    fn str<'s: 'a, S: ValueSource<'s, str>>(&mut self, value: S) -> Result {
        todo!()
    }

    fn text<'s: 'a, S: ValueSource<'s, Text>>(&mut self, value: S) -> Result {
        todo!()
    }

    fn error<'e: 'a, E: ValueSource<'e, Error>>(&mut self, error: E) -> Result {
        todo!()
    }

    fn bytes<'s: 'a, B: ValueSource<'s, Bytes>>(&mut self, bytes: B) -> Result {
        todo!()
    }

    fn tag<T: ValueSource<'static, str>>(&mut self, tag: Tag<T>) -> Result {
        todo!()
    }

    fn tagged_begin<T: ValueSource<'static, str>>(&mut self, tag: Tag<T>) -> Result {
        todo!()
    }

    fn tagged_end<T: ValueSource<'static, str>>(&mut self, tag: Tag<T>) -> Result {
        todo!()
    }

    fn tagged<'v: 'a, T: ValueSource<'static, str>, V: Source<'v>>(
        &mut self,
        tagged: Tagged<T, V>,
    ) -> Result {
        todo!()
    }

    fn tagged_str<'s: 'a, T: ValueSource<'static, str>, S: ValueSource<'s, str>>(
        &mut self,
        tagged: Tagged<T, S>,
    ) -> Result {
        todo!()
    }

    fn tagged_text<'s: 'a, T: ValueSource<'static, str>, S: ValueSource<'s, Text>>(
        &mut self,
        tagged: Tagged<T, S>,
    ) -> Result {
        todo!()
    }

    fn tagged_bytes<'s: 'a, T: ValueSource<'static, str>, B: ValueSource<'s, Bytes>>(
        &mut self,
        tagged: Tagged<T, B>,
    ) -> Result {
        todo!()
    }

    fn map_begin(&mut self, size: Option<u64>) -> Result {
        todo!()
    }

    fn map_end(&mut self) -> Result {
        todo!()
    }

    fn map_key_begin(&mut self) -> Result {
        todo!()
    }

    fn map_key_end(&mut self) -> Result {
        todo!()
    }

    fn map_value_begin(&mut self) -> Result {
        todo!()
    }

    fn map_value_end(&mut self) -> Result {
        todo!()
    }

    fn map_entry<'k: 'a, 'v: 'a, K: Source<'k>, V: Source<'v>>(
        &mut self,
        key: K,
        value: V,
    ) -> Result {
        todo!()
    }

    fn map_field_entry<'v: 'a, F: ValueSource<'static, str>, V: Source<'v>>(
        &mut self,
        field: F,
        value: V,
    ) -> Result {
        todo!()
    }

    fn map_field<F: ValueSource<'static, str>>(&mut self, field: F) -> Result {
        todo!()
    }

    fn map_key<'k: 'a, K: Source<'k>>(&mut self, key: K) -> Result {
        todo!()
    }

    fn map_value<'v: 'a, V: Source<'v>>(&mut self, value: V) -> Result {
        todo!()
    }

    fn seq_begin(&mut self, size: Option<u64>) -> Result {
        todo!()
    }

    fn seq_end(&mut self) -> Result {
        todo!()
    }

    fn seq_elem_begin(&mut self) -> Result {
        todo!()
    }

    fn seq_elem_end(&mut self) -> Result {
        todo!()
    }

    fn seq_elem<'e: 'a, E: Source<'e>>(&mut self, elem: E) -> Result {
        todo!()
    }
}

pub fn unsupported() -> Result {
    Err(crate::Error)
}
