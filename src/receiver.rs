use crate::error::unsupported;
use crate::{data, Result, Source, Value};

/**
An observer of structured data emitted by some source.
*/
pub trait Receiver<'a> {
    fn is_human_readable(&self) -> bool {
        true
    }

    fn value<V: Value + ?Sized + 'a>(&mut self, value: &'a V) -> Result {
        value.stream(self)
    }

    fn unit(&mut self) -> Result;

    fn null(&mut self) -> Result;

    fn u8(&mut self, value: u8) -> Result;

    fn u16(&mut self, value: u16) -> Result;

    fn u32(&mut self, value: u32) -> Result;

    fn u64(&mut self, value: u64) -> Result;

    fn u128(&mut self, value: u128) -> Result {
        let _ = value;
        unsupported()
    }

    fn i8(&mut self, value: i8) -> Result;

    fn i16(&mut self, value: i16) -> Result;

    fn i32(&mut self, value: i32) -> Result;

    fn i64(&mut self, value: i64) -> Result;

    fn i128(&mut self, value: i128) -> Result {
        let _ = value;
        unsupported()
    }

    fn f32(&mut self, value: f32) -> Result;

    fn f64(&mut self, value: f64) -> Result;

    fn bool(&mut self, value: bool) -> Result;

    fn char(&mut self, value: char) -> Result {
        let mut buf = [0; 4];
        let value = &*value.encode_utf8(&mut buf);

        self.text_begin(Some(value.len()))?;
        self.text_fragment_computed(value)?;
        self.text_end()
    }

    fn str(&mut self, value: &'a str) -> Result {
        self.text_begin(Some(value.len()))?;
        self.text_fragment(value)?;
        self.text_end()
    }

    fn text_begin(&mut self, num_bytes_hint: Option<usize>) -> Result;

    fn text_fragment(&mut self, fragment: &'a str) -> Result {
        self.text_fragment_computed(fragment)
    }

    fn text_fragment_computed(&mut self, fragment: &str) -> Result;

    fn text_end(&mut self) -> Result;

    fn bytes(&mut self, value: &'a [u8]) -> Result {
        self.binary_begin(Some(value.len()))?;
        self.binary_fragment(value)?;
        self.binary_end()
    }

    fn binary_begin(&mut self, num_bytes_hint: Option<usize>) -> Result;

    fn binary_fragment(&mut self, fragment: &'a [u8]) -> Result {
        self.binary_fragment_computed(fragment)
    }

    fn binary_fragment_computed(&mut self, fragment: &[u8]) -> Result;

    fn binary_end(&mut self) -> Result;

    fn tagged_begin(&mut self, tag: data::Tag) -> Result {
        let _ = tag;
        Ok(())
    }

    fn tagged_end(&mut self, tag: data::Tag) -> Result {
        let _ = tag;
        Ok(())
    }

    fn tagged<'v: 'a, V: Source<'v>>(&mut self, mut tagged: data::Tagged<V>) -> Result {
        self.tagged_begin(tagged.tag)?;
        tagged.value.stream_to_end(&mut *self)?;
        self.tagged_end(tagged.tag)
    }

    fn map_begin(&mut self, num_entries_hint: Option<usize>) -> Result;

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

    fn seq_begin(&mut self, num_elems_hint: Option<usize>) -> Result;

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
    ({ $($r:tt)* } => $bind:ident => { $($forward:tt)* }) => {
        $($r)* {
            fn is_human_readable(&self) -> bool {
                let $bind = self;
                ($($forward)*).is_human_readable()
            }

            fn value<V: Value + ?Sized + 'a>(&mut self, value: &'a V) -> Result {
                let $bind = self;
                ($($forward)*).value(value)
            }

            fn unit(&mut self) -> Result {
                let $bind = self;
                ($($forward)*).unit()
            }

            fn null(&mut self) -> Result {
                let $bind = self;
                ($($forward)*).null()
            }

            fn u8(&mut self, value: u8) -> Result {
                let $bind = self;
                ($($forward)*).u8(value)
            }

            fn u16(&mut self, value: u16) -> Result {
                let $bind = self;
                ($($forward)*).u16(value)
            }

            fn u32(&mut self, value: u32) -> Result {
                let $bind = self;
                ($($forward)*).u32(value)
            }

            fn u64(&mut self, value: u64) -> Result {
                let $bind = self;
                ($($forward)*).u64(value)
            }

            fn u128(&mut self, value: u128) -> Result {
                let $bind = self;
                ($($forward)*).u128(value)
            }

            fn i8(&mut self, value: i8) -> Result {
                let $bind = self;
                ($($forward)*).i8(value)
            }

            fn i16(&mut self, value: i16) -> Result {
                let $bind = self;
                ($($forward)*).i16(value)
            }

            fn i32(&mut self, value: i32) -> Result {
                let $bind = self;
                ($($forward)*).i32(value)
            }

            fn i64(&mut self, value: i64) -> Result {
                let $bind = self;
                ($($forward)*).i64(value)
            }

            fn i128(&mut self, value: i128) -> Result {
                let $bind = self;
                ($($forward)*).i128(value)
            }

            fn f32(&mut self, value: f32) -> Result {
                let $bind = self;
                ($($forward)*).f32(value)
            }

            fn f64(&mut self, value: f64) -> Result {
                let $bind = self;
                ($($forward)*).f64(value)
            }

            fn bool(&mut self, value: bool) -> Result {
                let $bind = self;
                ($($forward)*).bool(value)
            }

            fn char(&mut self, value: char) -> Result {
                let $bind = self;
                ($($forward)*).char(value)
            }

            fn str(&mut self, value: &'a str) -> Result {
                let $bind = self;
                ($($forward)*).str(value)
            }

            fn text_begin(&mut self, num_bytes_hint: Option<usize>) -> Result {
                let $bind = self;
                ($($forward)*).text_begin(num_bytes_hint)
            }

            fn text_end(&mut self) -> Result {
                let $bind = self;
                ($($forward)*).text_end()
            }

            fn text_fragment(&mut self, fragment: &'a str) -> Result {
                let $bind = self;
                ($($forward)*).text_fragment(fragment)
            }

            fn text_fragment_computed(&mut self, fragment: &str) -> Result {
                let $bind = self;
                ($($forward)*).text_fragment_computed(fragment)
            }

            fn bytes(&mut self, value: &'a [u8]) -> Result {
                let $bind = self;
                ($($forward)*).bytes(value)
            }

            fn binary_begin(&mut self, num_bytes_hint: Option<usize>) -> Result {
                let $bind = self;
                ($($forward)*).binary_begin(num_bytes_hint)
            }

            fn binary_end(&mut self) -> Result {
                let $bind = self;
                ($($forward)*).binary_end()
            }

            fn binary_fragment(&mut self, fragment: &'a [u8]) -> Result {
                let $bind = self;
                ($($forward)*).binary_fragment(fragment)
            }

            fn binary_fragment_computed(&mut self, fragment: &[u8]) -> Result {
                let $bind = self;
                ($($forward)*).binary_fragment_computed(fragment)
            }

            fn tagged_begin(&mut self, tag: data::Tag) -> Result {
                let $bind = self;
                ($($forward)*).tagged_begin(tag)
            }

            fn tagged_end(&mut self, tag: data::Tag) -> Result {
                let $bind = self;
                ($($forward)*).tagged_end(tag)
            }

            fn tagged<'v: 'a, V: Source<'v>>(&mut self, tagged: data::Tagged<V>) -> Result {
                let $bind = self;
                ($($forward)*).tagged(tagged)
            }

            fn map_begin(&mut self, num_entries_hint: Option<usize>) -> Result {
                let $bind = self;
                ($($forward)*).map_begin(num_entries_hint)
            }

            fn map_end(&mut self) -> Result {
                let $bind = self;
                ($($forward)*).map_end()
            }

            fn map_key_begin(&mut self) -> Result {
                let $bind = self;
                ($($forward)*).map_key_begin()
            }

            fn map_key_end(&mut self) -> Result {
                let $bind = self;
                ($($forward)*).map_key_end()
            }

            fn map_value_begin(&mut self) -> Result {
                let $bind = self;
                ($($forward)*).map_value_begin()
            }

            fn map_value_end(&mut self) -> Result {
                let $bind = self;
                ($($forward)*).map_value_end()
            }

            fn map_entry<'k: 'a, 'v: 'a, K: Source<'k>, V: Source<'v>>(
                &mut self,
                key: K,
                value: V,
            ) -> Result {
                let $bind = self;
                ($($forward)*).map_entry(key, value)
            }

            fn map_key<'k: 'a, K: Source<'k>>(&mut self, key: K) -> Result {
                let $bind = self;
                ($($forward)*).map_key(key)
            }

            fn map_value<'v: 'a, V: Source<'v>>(&mut self, value: V) -> Result {
                let $bind = self;
                ($($forward)*).map_value(value)
            }

            fn seq_begin(&mut self, num_elems_hint: Option<usize>) -> Result {
                let $bind = self;
                ($($forward)*).seq_begin(num_elems_hint)
            }

            fn seq_end(&mut self) -> Result {
                let $bind = self;
                ($($forward)*).seq_end()
            }

            fn seq_elem_begin(&mut self) -> Result {
                let $bind = self;
                ($($forward)*).seq_elem_begin()
            }

            fn seq_elem_end(&mut self) -> Result {
                let $bind = self;
                ($($forward)*).seq_elem_end()
            }

            fn seq_elem<'e: 'a, E: Source<'e>>(&mut self, elem: E) -> Result {
                let $bind = self;
                ($($forward)*).seq_elem(elem)
            }
        }
    };
}

// Simplifies the default receivers for extracting concrete types from values
pub(crate) trait DefaultUnsupported<'a> {
    fn as_receiver(&mut self) -> AsReceiver<&mut Self> {
        AsReceiver(self)
    }

    fn is_human_readable(&self) -> bool {
        false
    }

    fn value<V: Value + ?Sized + 'a>(&mut self, _: &'a V) -> Result {
        crate::error::unsupported()
    }

    fn unit(&mut self) -> Result {
        crate::error::unsupported()
    }

    fn null(&mut self) -> Result {
        crate::error::unsupported()
    }

    fn u8(&mut self, _: u8) -> Result {
        crate::error::unsupported()
    }

    fn u16(&mut self, _: u16) -> Result {
        crate::error::unsupported()
    }

    fn u32(&mut self, _: u32) -> Result {
        crate::error::unsupported()
    }

    fn u64(&mut self, _: u64) -> Result {
        crate::error::unsupported()
    }

    fn u128(&mut self, _: u128) -> Result {
        crate::error::unsupported()
    }

    fn i8(&mut self, _: i8) -> Result {
        crate::error::unsupported()
    }

    fn i16(&mut self, _: i16) -> Result {
        crate::error::unsupported()
    }

    fn i32(&mut self, _: i32) -> Result {
        crate::error::unsupported()
    }

    fn i64(&mut self, _: i64) -> Result {
        crate::error::unsupported()
    }

    fn i128(&mut self, _: i128) -> Result {
        crate::error::unsupported()
    }

    fn f32(&mut self, _: f32) -> Result {
        crate::error::unsupported()
    }

    fn f64(&mut self, _: f64) -> Result {
        crate::error::unsupported()
    }

    fn bool(&mut self, _: bool) -> Result {
        crate::error::unsupported()
    }

    fn char(&mut self, _: char) -> Result {
        crate::error::unsupported()
    }

    fn str(&mut self, _: &'a str) -> Result {
        crate::error::unsupported()
    }

    fn text_begin(&mut self, _: Option<usize>) -> Result {
        crate::error::unsupported()
    }

    fn text_fragment(&mut self, _: &'a str) -> Result {
        crate::error::unsupported()
    }

    fn text_fragment_computed(&mut self, _: &str) -> Result {
        crate::error::unsupported()
    }

    fn text_end(&mut self) -> Result {
        crate::error::unsupported()
    }

    fn bytes(&mut self, _: &'a [u8]) -> Result {
        crate::error::unsupported()
    }

    fn binary_begin(&mut self, _: Option<usize>) -> Result {
        crate::error::unsupported()
    }

    fn binary_fragment(&mut self, _: &'a [u8]) -> Result {
        crate::error::unsupported()
    }

    fn binary_fragment_computed(&mut self, _: &[u8]) -> Result {
        crate::error::unsupported()
    }

    fn binary_end(&mut self) -> Result {
        crate::error::unsupported()
    }

    fn tag(&mut self, _: data::Tag) -> Result {
        crate::error::unsupported()
    }

    fn tagged_begin(&mut self, _: data::Tag) -> Result {
        crate::error::unsupported()
    }

    fn tagged_end(&mut self, _: data::Tag) -> Result {
        crate::error::unsupported()
    }

    fn tagged<'v: 'a, V: Source<'v>>(&mut self, _: data::Tagged<V>) -> Result {
        crate::error::unsupported()
    }

    fn map_begin(&mut self, _: Option<usize>) -> Result {
        crate::error::unsupported()
    }

    fn map_key_begin(&mut self) -> Result {
        crate::error::unsupported()
    }

    fn map_key_end(&mut self) -> Result {
        crate::error::unsupported()
    }

    fn map_value_begin(&mut self) -> Result {
        crate::error::unsupported()
    }

    fn map_value_end(&mut self) -> Result {
        crate::error::unsupported()
    }

    fn map_end(&mut self) -> Result {
        crate::error::unsupported()
    }

    fn map_entry<'k: 'a, 'v: 'a, K: Source<'k>, V: Source<'v>>(&mut self, _: K, _: V) -> Result {
        crate::error::unsupported()
    }

    fn map_key<'k: 'a, K: Source<'k>>(&mut self, _: K) -> Result {
        crate::error::unsupported()
    }

    fn map_value<'v: 'a, V: Source<'v>>(&mut self, _: V) -> Result {
        crate::error::unsupported()
    }

    fn seq_begin(&mut self, _: Option<usize>) -> Result {
        crate::error::unsupported()
    }

    fn seq_elem_begin(&mut self) -> Result {
        crate::error::unsupported()
    }

    fn seq_elem_end(&mut self) -> Result {
        crate::error::unsupported()
    }

    fn seq_end(&mut self) -> Result {
        crate::error::unsupported()
    }

    fn seq_elem<'e: 'a, E: Source<'e>>(&mut self, _: E) -> Result {
        crate::error::unsupported()
    }
}

pub(crate) struct AsReceiver<T>(T);

impl_receiver_forward!({ impl<'a, 'b, R: ?Sized> Receiver<'a> for &'b mut R where R: Receiver<'a> } => x => { **x });
impl_receiver_forward!({ impl<'a, 'b, R> Receiver<'a> for AsReceiver<&'b mut R> where R: DefaultUnsupported<'a> } => x => { x.0 });

#[cfg(feature = "alloc")]
mod alloc_support {
    use super::*;

    use crate::std::boxed::Box;

    impl_receiver_forward!({ impl<'a, 'b, R: ?Sized> Receiver<'a> for Box<R> where R: Receiver<'a> } => x => { **x });
}
