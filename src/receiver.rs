use crate::{data, for_all, std::fmt::Display, Result, Source, SourceRef, SourceValue};

pub trait Receiver<'a> {
    fn value<'v: 'a, V: SourceValue + ?Sized + 'v>(&mut self, value: &'v V) -> Result {
        value.stream(self)
    }

    fn unstructured<D: Display>(&mut self, fmt: D) -> Result;

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
        self.str(for_all(&*value.encode_utf8(&mut buf)))
    }

    fn str<'s: 'a, S: SourceRef<'s, str>>(&mut self, mut value: S) -> Result {
        self.text(for_all(value.take()?))
    }

    fn text<'s: 'a, S: SourceRef<'s, data::Text>>(&mut self, mut text: S) -> Result {
        self.unstructured(text.take()?)
    }

    fn error<'e: 'a, E: SourceRef<'e, data::Error>>(&mut self, mut error: E) -> Result {
        self.unstructured(error.take()?)
    }

    fn bytes<'s: 'a, B: SourceRef<'s, data::Bytes>>(&mut self, mut bytes: B) -> Result {
        self.unstructured(bytes.take()?)
    }

    fn tag<T: SourceRef<'static, str>>(&mut self, mut tag: data::Tag<T>) -> Result {
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

    fn tagged_begin<T: SourceRef<'static, str>>(&mut self, tag: data::Tag<T>) -> Result {
        let _ = tag;
        Ok(())
    }

    fn tagged_end<T: SourceRef<'static, str>>(&mut self, tag: data::Tag<T>) -> Result {
        let _ = tag;
        Ok(())
    }

    fn tagged<'v: 'a, T: SourceRef<'static, str>, V: Source<'v>>(
        &mut self,
        mut tagged: data::Tagged<T, V>,
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

    fn map_field_entry<'v: 'a, F: SourceRef<'static, str>, V: Source<'v>>(
        &mut self,
        field: F,
        value: V,
    ) -> Result {
        self.map_field(field)?;
        self.map_value(value)
    }

    fn map_field<F: SourceRef<'static, str>>(&mut self, mut field: F) -> Result {
        // TODO: Do we lose static fields from this?
        self.map_key(for_all(field.take()?))
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

macro_rules! impl_receiver_forward {
    ($($r:tt)*) => {
        $($r)* {
            fn value<'v: 'a, V: SourceValue + ?Sized + 'v>(&mut self, value: &'v V) -> Result {
                (**self).value(value)
            }

            fn unstructured<D: Display>(&mut self, fmt: D) -> Result {
                (**self).unstructured(fmt)
            }

            fn null(&mut self) -> Result {
                (**self).null()
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

            fn u128(&mut self, value: u128) -> Result {
                (**self).u128(value)
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

            fn str<'s: 'a, S: SourceRef<'s, str>>(&mut self, value: S) -> Result {
                (**self).str(value)
            }

            fn text<'s: 'a, S: SourceRef<'s, data::Text>>(&mut self, text: S) -> Result {
                (**self).text(text)
            }

            fn error<'e: 'a, E: SourceRef<'e, data::Error>>(&mut self, error: E) -> Result {
                (**self).error(error)
            }

            fn bytes<'s: 'a, B: SourceRef<'s, data::Bytes>>(&mut self, bytes: B) -> Result {
                (**self).bytes(bytes)
            }

            fn tag<T: SourceRef<'static, str>>(&mut self, tag: data::Tag<T>) -> Result {
                (**self).tag(tag)
            }

            fn tagged_begin<T: SourceRef<'static, str>>(&mut self, tag: data::Tag<T>) -> Result {
                (**self).tagged_begin(tag)
            }

            fn tagged_end<T: SourceRef<'static, str>>(&mut self, tag: data::Tag<T>) -> Result {
                (**self).tagged_end(tag)
            }

            fn tagged<'v: 'a, T: SourceRef<'static, str>, V: Source<'v>>(
                &mut self,
                tagged: data::Tagged<T, V>,
            ) -> Result {
                (**self).tagged(tagged)
            }

            fn map_begin(&mut self, size: Option<u64>) -> Result {
                (**self).map_begin(size)
            }

            fn map_end(&mut self) -> Result {
                (**self).map_end()
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

            fn map_field_entry<'v: 'a, F: SourceRef<'static, str>, V: Source<'v>>(
                &mut self,
                field: F,
                value: V,
            ) -> Result {
                (**self).map_field_entry(field, value)
            }

            fn map_field<F: SourceRef<'static, str>>(&mut self, field: F) -> Result {
                (**self).map_field(field)
            }

            fn map_key<'k: 'a, K: Source<'k>>(&mut self, key: K) -> Result {
                (**self).map_key(key)
            }

            fn map_value<'v: 'a, V: Source<'v>>(&mut self, value: V) -> Result {
                (**self).map_value(value)
            }

            fn seq_begin(&mut self, size: Option<u64>) -> Result {
                (**self).seq_begin(size)
            }

            fn seq_end(&mut self) -> Result {
                (**self).seq_end()
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
    };
}

impl_receiver_forward!(impl<'a, 'b, R: ?Sized> Receiver<'a> for &'b mut R where R: Receiver<'a>);

#[cfg(feature = "alloc")]
mod alloc_support {
    use super::*;

    use crate::std::boxed::Box;

    impl_receiver_forward!(impl<'a, 'b, R: ?Sized> Receiver<'a> for Box<R> where R: Receiver<'a>);
}
