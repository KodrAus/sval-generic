use crate::{data, Index, Label, Result, Tag, Value};

pub trait Stream<'sval> {
    fn null(&mut self) -> Result;

    fn bool(&mut self, value: bool) -> Result;

    fn text_begin(&mut self, num_bytes_hint: Option<usize>) -> Result;

    fn text_fragment(&mut self, fragment: &'sval str) -> Result {
        self.text_fragment_computed(fragment)
    }

    fn text_fragment_computed(&mut self, fragment: &str) -> Result;

    fn text_end(&mut self) -> Result;

    fn binary_begin(&mut self, num_bytes_hint: Option<usize>) -> Result {
        self.seq_begin(num_bytes_hint)
    }

    fn binary_fragment(&mut self, fragment: &'sval [u8]) -> Result {
        self.binary_fragment_computed(fragment)
    }

    fn binary_fragment_computed(&mut self, fragment: &[u8]) -> Result {
        for byte in fragment {
            self.seq_value_begin()?;
            self.u8(*byte)?;
            self.seq_value_end()?;
        }

        Ok(())
    }

    fn binary_end(&mut self) -> Result {
        self.seq_end()
    }

    fn u8(&mut self, value: u8) -> Result {
        data::number::u8_int(value, self)
    }

    fn u16(&mut self, value: u16) -> Result {
        if let Ok(value) = value.try_into() {
            self.u8(value)
        } else {
            data::number::u16_int(value, self)
        }
    }

    fn u32(&mut self, value: u32) -> Result {
        if let Ok(value) = value.try_into() {
            self.u16(value)
        } else {
            data::number::u32_int(value, self)
        }
    }

    fn u64(&mut self, value: u64) -> Result {
        if let Ok(value) = value.try_into() {
            self.u32(value)
        } else {
            data::number::u64_int(value, self)
        }
    }

    fn u128(&mut self, value: u128) -> Result {
        if let Ok(value) = value.try_into() {
            self.u64(value)
        } else {
            data::number::u128_int(value, self)
        }
    }

    fn i8(&mut self, value: i8) -> Result {
        data::number::i8_int(value, self)
    }

    fn i16(&mut self, value: i16) -> Result {
        if let Ok(value) = value.try_into() {
            self.i8(value)
        } else {
            data::number::i16_int(value, self)
        }
    }

    fn i32(&mut self, value: i32) -> Result {
        if let Ok(value) = value.try_into() {
            self.i16(value)
        } else {
            data::number::i32_int(value, self)
        }
    }

    fn i64(&mut self, value: i64) -> Result {
        if let Ok(value) = value.try_into() {
            self.i32(value)
        } else {
            data::number::i64_int(value, self)
        }
    }

    fn i128(&mut self, value: i128) -> Result {
        if let Ok(value) = value.try_into() {
            self.i64(value)
        } else {
            data::number::i128_int(value, self)
        }
    }

    fn f32(&mut self, value: f32) -> Result {
        data::number::f32_number(value, self)
    }

    fn f64(&mut self, value: f64) -> Result {
        data::number::f64_number(value, self)
    }

    fn map_begin(&mut self, num_entries_hint: Option<usize>) -> Result;

    fn map_key_begin(&mut self) -> Result;

    fn map_key_end(&mut self) -> Result;

    fn map_value_begin(&mut self) -> Result;

    fn map_value_end(&mut self) -> Result;

    fn map_end(&mut self) -> Result;

    fn seq_begin(&mut self, num_entries_hint: Option<usize>) -> Result;

    fn seq_value_begin(&mut self) -> Result;

    fn seq_value_end(&mut self) -> Result;

    fn seq_end(&mut self) -> Result;

    fn dynamic_begin(&mut self) -> Result {
        Ok(())
    }

    fn dynamic_end(&mut self) -> Result {
        Ok(())
    }

    fn enum_begin(
        &mut self,
        tag: Option<Tag>,
        label: Option<Label>,
        index: Option<Index>,
    ) -> Result {
        self.tagged_begin(tag, label, index)?;
        self.dynamic_begin()
    }

    fn enum_end(&mut self, tag: Option<Tag>, label: Option<Label>, index: Option<Index>) -> Result {
        self.dynamic_end()?;
        self.tagged_end(tag, label, index)
    }

    fn tagged_begin(
        &mut self,
        tag: Option<Tag>,
        label: Option<Label>,
        index: Option<Index>,
    ) -> Result {
        let _ = tag;
        let _ = label;
        let _ = index;

        Ok(())
    }

    fn tagged_end(
        &mut self,
        tag: Option<Tag>,
        label: Option<Label>,
        index: Option<Index>,
    ) -> Result {
        let _ = tag;
        let _ = label;
        let _ = index;

        Ok(())
    }

    fn record_begin(
        &mut self,
        tag: Option<Tag>,
        label: Option<Label>,
        index: Option<Index>,
        num_entries: Option<usize>,
    ) -> Result {
        self.tagged_begin(tag, label, index)?;
        self.map_begin(num_entries)
    }

    fn record_value_begin(&mut self, label: Label) -> Result {
        self.map_key_begin()?;

        if let Some(label) = label.try_get_static() {
            label.stream(&mut *self)?;
        } else {
            self.text_begin(Some(label.len()))?;
            self.text_fragment_computed(&label)?;
            self.text_end()?;
        }

        self.map_key_end()?;

        self.map_value_begin()?;
        self.dynamic_begin()
    }

    fn record_value_end(&mut self, label: Label) -> Result {
        let _ = label;

        self.dynamic_end()?;
        self.map_value_end()
    }

    fn record_end(
        &mut self,
        tag: Option<Tag>,
        label: Option<Label>,
        index: Option<Index>,
    ) -> Result {
        self.map_end()?;
        self.tagged_end(tag, label, index)
    }

    fn tuple_begin(
        &mut self,
        tag: Option<Tag>,
        label: Option<Label>,
        index: Option<Index>,
        num_entries: Option<usize>,
    ) -> Result {
        self.tagged_begin(tag, label, index)?;
        self.seq_begin(num_entries)
    }

    fn tuple_value_begin(&mut self, index: Index) -> Result {
        let _ = index;

        self.seq_value_begin()?;
        self.dynamic_begin()
    }

    fn tuple_value_end(&mut self, index: Index) -> Result {
        let _ = index;

        self.dynamic_end()?;
        self.seq_value_end()
    }

    fn tuple_end(
        &mut self,
        tag: Option<Tag>,
        label: Option<Label>,
        index: Option<Index>,
    ) -> Result {
        self.seq_end()?;
        self.tagged_end(tag, label, index)
    }

    // TODO: Move this to a tag
    fn constant_begin(
        &mut self,
        tag: Option<Tag>,
        label: Option<Label>,
        index: Option<Index>,
    ) -> Result {
        self.tagged_begin(tag, label, index)
    }

    fn constant_end(
        &mut self,
        tag: Option<Tag>,
        label: Option<Label>,
        index: Option<Index>,
    ) -> Result {
        self.tagged_end(tag, label, index)
    }

    /**
    Begin an arbitrarily sized decimal floating point number.

    # Structure

    Arbitrary sized decimal floating points wrap a text or binary blob with the encoding described below.
    A call to `number_begin` must be followed by a call to `number_end` after the floating point value:

    ```
    # fn wrap<'a>(num_bytes_hint: Option<usize>, mut stream: impl sval::Stream<'a>) -> sval::Result {
    stream.number_begin()?;

    stream.text_begin(Some(8))?;
    stream.text_fragment("1333.754")?;
    stream.text_end()?;

    stream.number_end()?;
    # Ok(())
    # }
    ```
    */
    fn number_begin(&mut self) -> Result {
        Ok(())
    }

    /**
    End an arbitrary sized decimal floating point number.

    See [`Stream::number_begin`] for details on arbitrary sized decimal floating points.
     */
    fn number_end(&mut self) -> Result {
        Ok(())
    }
}

macro_rules! impl_stream_forward {
    ({ $($r:tt)* } => $bind:ident => { $($forward:tt)* }) => {
        $($r)* {
            fn dynamic_begin(&mut self) -> Result {
                let $bind = self;
                ($($forward)*).dynamic_begin()
            }

            fn dynamic_end(&mut self) -> Result {
                let $bind = self;
                ($($forward)*).dynamic_end()
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

            fn text_begin(&mut self, num_bytes_hint: Option<usize>) -> Result {
                let $bind = self;
                ($($forward)*).text_begin(num_bytes_hint)
            }

            fn text_end(&mut self) -> Result {
                let $bind = self;
                ($($forward)*).text_end()
            }

            fn text_fragment(&mut self, fragment: &'sval str) -> Result {
                let $bind = self;
                ($($forward)*).text_fragment(fragment)
            }

            fn text_fragment_computed(&mut self, fragment: &str) -> Result {
                let $bind = self;
                ($($forward)*).text_fragment_computed(fragment)
            }

            fn binary_begin(&mut self, num_bytes_hint: Option<usize>) -> Result {
                let $bind = self;
                ($($forward)*).binary_begin(num_bytes_hint)
            }

            fn binary_end(&mut self) -> Result {
                let $bind = self;
                ($($forward)*).binary_end()
            }

            fn binary_fragment(&mut self, fragment: &'sval [u8]) -> Result {
                let $bind = self;
                ($($forward)*).binary_fragment(fragment)
            }

            fn binary_fragment_computed(&mut self, fragment: &[u8]) -> Result {
                let $bind = self;
                ($($forward)*).binary_fragment_computed(fragment)
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

            fn seq_begin(&mut self, num_entries_hint: Option<usize>) -> Result {
                let $bind = self;
                ($($forward)*).seq_begin(num_entries_hint)
            }

            fn seq_end(&mut self) -> Result {
                let $bind = self;
                ($($forward)*).seq_end()
            }

            fn seq_value_begin(&mut self) -> Result {
                let $bind = self;
                ($($forward)*).seq_value_begin()
            }

            fn seq_value_end(&mut self) -> Result {
                let $bind = self;
                ($($forward)*).seq_value_end()
            }

            fn tagged_begin(&mut self, tag: Option<Tag>, label: Option<Label>, index: Option<Index>) -> Result {
                let $bind = self;
                ($($forward)*).tagged_begin(tag, label, index)
            }

            fn tagged_end(&mut self, tag: Option<Tag>, label: Option<Label>, index: Option<Index>) -> Result {
                let $bind = self;
                ($($forward)*).tagged_end(tag, label, index)
            }

            fn constant_begin(&mut self, tag: Option<Tag>, label: Option<Label>, index: Option<Index>) -> Result {
                let $bind = self;
                ($($forward)*).constant_begin(tag, label, index)
            }

            fn constant_end(&mut self, tag: Option<Tag>, label: Option<Label>, index: Option<Index>) -> Result {
                let $bind = self;
                ($($forward)*).constant_end(tag, label, index)
            }

            fn record_begin(&mut self, tag: Option<Tag>, label: Option<Label>, index: Option<Index>, num_entries: Option<usize>) -> Result {
                let $bind = self;
                ($($forward)*).record_begin(tag, label, index, num_entries)
            }

            fn record_value_begin(&mut self, label: Label) -> Result {
                let $bind = self;
                ($($forward)*).record_value_begin(label)
            }

            fn record_value_end(&mut self, label: Label) -> Result {
                let $bind = self;
                ($($forward)*).record_value_end(label)
            }

            fn record_end(&mut self, tag: Option<Tag>, label: Option<Label>, index: Option<Index>) -> Result {
                let $bind = self;
                ($($forward)*).record_end(tag, label, index)
            }

            fn tuple_begin(&mut self, tag: Option<Tag>, label: Option<Label>, index: Option<Index>, num_entries: Option<usize>) -> Result {
                let $bind = self;
                ($($forward)*).tuple_begin(tag, label, index, num_entries)
            }

            fn tuple_value_begin(&mut self, index: Index) -> Result {
                let $bind = self;
                ($($forward)*).tuple_value_begin(index)
            }

            fn tuple_value_end(&mut self, index: Index) -> Result {
                let $bind = self;
                ($($forward)*).tuple_value_end(index)
            }

            fn tuple_end(&mut self, tag: Option<Tag>, label: Option<Label>, index: Option<Index>) -> Result {
                let $bind = self;
                ($($forward)*).tuple_end(tag, label, index)
            }

            fn enum_begin(&mut self, tag: Option<Tag>, label: Option<Label>, index: Option<Index>) -> Result {
                let $bind = self;
                ($($forward)*).enum_begin(tag, label, index)
            }

            fn enum_end(&mut self, tag: Option<Tag>, label: Option<Label>, index: Option<Index>) -> Result {
                let $bind = self;
                ($($forward)*).enum_end(tag, label, index)
            }

            fn number_begin(&mut self) -> Result {
                let $bind = self;
                ($($forward)*).number_begin()
            }

            fn number_end(&mut self) -> Result {
                let $bind = self;
                ($($forward)*).number_end()
            }
        }
    };
}

// Simplifies the default streams for extracting concrete types from values
pub(crate) trait DefaultUnsupported<'sval> {
    fn into_stream(self) -> IntoStream<Self>
    where
        Self: Sized,
    {
        IntoStream(self)
    }

    fn dynamic_begin(&mut self) -> Result {
        crate::result::unsupported()
    }

    fn dynamic_end(&mut self) -> Result {
        crate::result::unsupported()
    }

    fn null(&mut self) -> Result {
        crate::result::unsupported()
    }

    fn u8(&mut self, _: u8) -> Result {
        crate::result::unsupported()
    }

    fn u16(&mut self, _: u16) -> Result {
        crate::result::unsupported()
    }

    fn u32(&mut self, _: u32) -> Result {
        crate::result::unsupported()
    }

    fn u64(&mut self, _: u64) -> Result {
        crate::result::unsupported()
    }

    fn u128(&mut self, _: u128) -> Result {
        crate::result::unsupported()
    }

    fn i8(&mut self, _: i8) -> Result {
        crate::result::unsupported()
    }

    fn i16(&mut self, _: i16) -> Result {
        crate::result::unsupported()
    }

    fn i32(&mut self, _: i32) -> Result {
        crate::result::unsupported()
    }

    fn i64(&mut self, _: i64) -> Result {
        crate::result::unsupported()
    }

    fn i128(&mut self, _: i128) -> Result {
        crate::result::unsupported()
    }

    fn f32(&mut self, _: f32) -> Result {
        crate::result::unsupported()
    }

    fn f64(&mut self, _: f64) -> Result {
        crate::result::unsupported()
    }

    fn bool(&mut self, _: bool) -> Result {
        crate::result::unsupported()
    }

    fn text_begin(&mut self, _: Option<usize>) -> Result {
        crate::result::unsupported()
    }

    fn text_fragment(&mut self, _: &'sval str) -> Result {
        crate::result::unsupported()
    }

    fn text_fragment_computed(&mut self, _: &str) -> Result {
        crate::result::unsupported()
    }

    fn text_end(&mut self) -> Result {
        crate::result::unsupported()
    }

    fn binary_begin(&mut self, _: Option<usize>) -> Result {
        crate::result::unsupported()
    }

    fn binary_fragment(&mut self, _: &'sval [u8]) -> Result {
        crate::result::unsupported()
    }

    fn binary_fragment_computed(&mut self, _: &[u8]) -> Result {
        crate::result::unsupported()
    }

    fn binary_end(&mut self) -> Result {
        crate::result::unsupported()
    }

    fn map_begin(&mut self, _: Option<usize>) -> Result {
        crate::result::unsupported()
    }

    fn map_key_begin(&mut self) -> Result {
        crate::result::unsupported()
    }

    fn map_key_end(&mut self) -> Result {
        crate::result::unsupported()
    }

    fn map_value_begin(&mut self) -> Result {
        crate::result::unsupported()
    }

    fn map_value_end(&mut self) -> Result {
        crate::result::unsupported()
    }

    fn map_end(&mut self) -> Result {
        crate::result::unsupported()
    }

    fn seq_begin(&mut self, _: Option<usize>) -> Result {
        crate::result::unsupported()
    }

    fn seq_value_begin(&mut self) -> Result {
        crate::result::unsupported()
    }

    fn seq_value_end(&mut self) -> Result {
        crate::result::unsupported()
    }

    fn seq_end(&mut self) -> Result {
        crate::result::unsupported()
    }

    fn tagged_begin(&mut self, _: Option<Tag>, _: Option<Label>, _: Option<Index>) -> Result {
        crate::result::unsupported()
    }

    fn tagged_end(&mut self, _: Option<Tag>, _: Option<Label>, _: Option<Index>) -> Result {
        crate::result::unsupported()
    }

    fn constant_begin(&mut self, _: Option<Tag>, _: Option<Label>, _: Option<Index>) -> Result {
        crate::result::unsupported()
    }

    fn constant_end(&mut self, _: Option<Tag>, _: Option<Label>, _: Option<Index>) -> Result {
        crate::result::unsupported()
    }

    fn record_begin(
        &mut self,
        _: Option<Tag>,
        _: Option<Label>,
        _: Option<Index>,
        _: Option<usize>,
    ) -> Result {
        crate::result::unsupported()
    }

    fn record_value_begin(&mut self, _: Label) -> Result {
        crate::result::unsupported()
    }

    fn record_value_end(&mut self, _: Label) -> Result {
        crate::result::unsupported()
    }

    fn record_end(&mut self, _: Option<Tag>, _: Option<Label>, _: Option<Index>) -> Result {
        crate::result::unsupported()
    }

    fn tuple_begin(
        &mut self,
        _: Option<Tag>,
        _: Option<Label>,
        _: Option<Index>,
        _: Option<usize>,
    ) -> Result {
        crate::result::unsupported()
    }

    fn tuple_value_begin(&mut self, _: Index) -> Result {
        crate::result::unsupported()
    }

    fn tuple_value_end(&mut self, _: Index) -> Result {
        crate::result::unsupported()
    }

    fn tuple_end(&mut self, _: Option<Tag>, _: Option<Label>, _: Option<Index>) -> Result {
        crate::result::unsupported()
    }

    fn enum_begin(&mut self, _: Option<Tag>, _: Option<Label>, _: Option<Index>) -> Result {
        crate::result::unsupported()
    }

    fn enum_end(&mut self, _: Option<Tag>, _: Option<Label>, _: Option<Index>) -> Result {
        crate::result::unsupported()
    }

    fn number_begin(&mut self) -> Result {
        crate::result::unsupported()
    }

    fn number_end(&mut self) -> Result {
        crate::result::unsupported()
    }
}

pub(crate) struct IntoStream<T: ?Sized>(pub(crate) T);

impl_stream_forward!({ impl<'sval, 'a, S: ?Sized> Stream<'sval> for &'a mut S where S: Stream<'sval> } => x => { **x });
impl_stream_forward!({ impl<'sval, 'a, S> Stream<'sval> for IntoStream<S> where S: DefaultUnsupported<'sval> } => x => { x.0 });

#[cfg(feature = "alloc")]
mod alloc_support {
    use super::*;

    use crate::std::boxed::Box;

    impl_stream_forward!({ impl<'sval, 'a, S: ?Sized> Stream<'sval> for Box<S> where S: Stream<'sval> } => x => { **x });
}
