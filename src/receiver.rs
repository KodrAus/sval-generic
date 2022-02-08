use crate::{data, Result, Source, Value};

pub trait Receiver<'a> {
    fn is_human_readable(&self) -> bool {
        true
    }

    fn value<'v: 'a, V: Value + ?Sized + 'v>(&mut self, value: &'v V) -> Result {
        value.stream(self)
    }

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

    fn u128(&mut self, value: u128) -> Result;

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

    fn i128(&mut self, value: i128) -> Result;

    fn f32(&mut self, value: f32) -> Result {
        self.f64(value as f64)
    }

    fn f64(&mut self, value: f64) -> Result;

    fn bool(&mut self, value: bool) -> Result;

    fn char(&mut self, value: char) -> Result {
        let mut buf = [0; 4];
        data::for_all(self).str(&*value.encode_utf8(&mut buf))
    }

    fn str(&mut self, value: &'a str) -> Result {
        self.text_begin(Some(value.len() as u64))?;
        self.text_fragment(value)?;
        self.text_end()
    }

    fn text_begin(&mut self, num_bytes: Option<u64>) -> Result;

    fn text_fragment(&mut self, fragment: &str) -> Result;

    fn text_end(&mut self) -> Result;

    fn bytes(&mut self, value: &'a [u8]) -> Result {
        self.binary_begin(Some(value.len() as u64))?;
        self.binary_fragment(value)?;
        self.binary_end()
    }

    fn binary_begin(&mut self, num_bytes: Option<u64>) -> Result;

    fn binary_fragment(&mut self, fragment: &[u8]) -> Result;

    fn binary_end(&mut self) -> Result;

    fn tag(&mut self, tag: data::Tag) -> Result {
        // For human-readable formats, prefer the text label
        if self.is_human_readable() {
            if let Some(label) = tag.label() {
                self.str(label)?;
                return Ok(());
            }

            if let Some(id) = tag.id() {
                self.u64(id)?;
                return Ok(());
            }
        }
        // For non-human-readable formats, prefer the integer id
        else {
            if let Some(id) = tag.id() {
                self.u64(id)?;
                return Ok(());
            }

            if let Some(label) = tag.label() {
                self.str(label)?;
                return Ok(());
            }
        }

        self.null()
    }

    fn tagged_begin(&mut self, tag: data::Tag) -> Result {
        let _ = tag;
        Ok(())
    }

    fn tagged_end(&mut self, tag: data::Tag) -> Result {
        let _ = tag;
        Ok(())
    }

    fn tagged<'v: 'a, V: Source<'v>>(&mut self, mut tagged: data::Tagged<V>) -> Result {
        self.tagged_begin(tagged.tag())?;
        tagged.value_mut().stream_to_end(&mut *self)?;
        self.tagged_end(tagged.tag())
    }

    fn map_begin(&mut self, num_entries: Option<u64>) -> Result;

    fn map_key_begin(&mut self) -> Result;

    fn map_key_end(&mut self) -> Result;

    fn map_value_begin(&mut self) -> Result;

    fn map_value_end(&mut self) -> Result;

    fn map_end(&mut self) -> Result;

    fn map_entry<'k: 'a, 'v: 'a, K: Source<'k>, V: Source<'v>>(
        &mut self,
        key: K,
        value: V,
    ) -> Result {
        self.map_key(key)?;
        self.map_value(value)
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

    fn seq_begin(&mut self, num_elems: Option<u64>) -> Result;

    fn seq_elem_begin(&mut self) -> Result;

    fn seq_elem_end(&mut self) -> Result;

    fn seq_end(&mut self) -> Result;

    fn seq_elem<'e: 'a, E: Source<'e>>(&mut self, mut elem: E) -> Result {
        self.seq_elem_begin()?;
        elem.stream_to_end(&mut *self)?;
        self.seq_elem_end()
    }
}

macro_rules! impl_receiver_forward {
    ($($r:tt)*) => {
        $($r)* {
            fn is_human_readable(&self) -> bool {
                (**self).is_human_readable()
            }

            fn value<'v: 'a, V: Value + ?Sized + 'v>(&mut self, value: &'v V) -> Result {
                (**self).value(value)
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

            fn str(&mut self, value: &'a str) -> Result {
                (**self).str(value)
            }

            fn text_begin(&mut self, num_bytes: Option<u64>) -> Result {
                (**self).text_begin(num_bytes)
            }

            fn text_end(&mut self) -> Result {
                (**self).text_end()
            }

            fn text_fragment(&mut self, fragment: &str) -> Result {
                (**self).text_fragment(fragment)
            }

            fn bytes(&mut self, value: &'a [u8]) -> Result {
                (**self).bytes(value)
            }

            fn binary_begin(&mut self, num_bytes: Option<u64>) -> Result {
                (**self).binary_begin(num_bytes)
            }

            fn binary_end(&mut self) -> Result {
                (**self).binary_end()
            }

            fn binary_fragment(&mut self, fragment: &[u8]) -> Result {
                (**self).binary_fragment(fragment)
            }

            fn tag(&mut self, tag: data::Tag) -> Result {
                (**self).tag(tag)
            }

            fn tagged_begin(&mut self, tag: data::Tag) -> Result {
                (**self).tagged_begin(tag)
            }

            fn tagged_end(&mut self, tag: data::Tag) -> Result {
                (**self).tagged_end(tag)
            }

            fn tagged<'v: 'a, V: Source<'v>>(&mut self, tagged: data::Tagged<V>) -> Result {
                (**self).tagged(tagged)
            }

            fn map_begin(&mut self, num_entries: Option<u64>) -> Result {
                (**self).map_begin(num_entries)
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

            fn map_key<'k: 'a, K: Source<'k>>(&mut self, key: K) -> Result {
                (**self).map_key(key)
            }

            fn map_value<'v: 'a, V: Source<'v>>(&mut self, value: V) -> Result {
                (**self).map_value(value)
            }

            fn seq_begin(&mut self, num_elems: Option<u64>) -> Result {
                (**self).seq_begin(num_elems)
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
