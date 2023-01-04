use crate::{data, tags, Index, Label, Result, Tag, Value};

pub trait Stream<'sval> {
    fn value<V: Value + ?Sized>(&mut self, v: &'sval V) -> Result {
        v.stream(self)
    }

    fn value_computed<V: Value + ?Sized>(&mut self, v: &V) -> Result {
        stream_computed(self, v)
    }

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
        if let Ok(value) = value.try_into() {
            self.i64(value)
        } else {
            data::stream_u128(value, self)
        }
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

    fn i64(&mut self, value: i64) -> Result;

    fn i128(&mut self, value: i128) -> Result {
        if let Ok(value) = value.try_into() {
            self.i64(value)
        } else {
            data::stream_i128(value, self)
        }
    }

    fn f32(&mut self, value: f32) -> Result {
        self.f64(value as f64)
    }

    fn f64(&mut self, value: f64) -> Result;

    fn map_begin(&mut self, num_entries_hint: Option<usize>) -> Result {
        self.seq_begin(num_entries_hint)
    }

    fn map_key_begin(&mut self) -> Result {
        self.seq_value_begin()?;
        self.tuple_begin(None, None, None, Some(2))?;
        self.tuple_value_begin(Index::new(0))
    }

    fn map_key_end(&mut self) -> Result {
        self.tuple_value_end(Index::new(0))
    }

    fn map_value_begin(&mut self) -> Result {
        self.tuple_value_begin(Index::new(1))
    }

    fn map_value_end(&mut self) -> Result {
        self.tuple_value_end(Index::new(1))?;
        self.tuple_end(None, None, None)?;
        self.seq_value_end()
    }

    fn map_end(&mut self) -> Result {
        self.seq_end()
    }

    fn seq_begin(&mut self, num_entries_hint: Option<usize>) -> Result;

    fn seq_value_begin(&mut self) -> Result;

    fn seq_value_end(&mut self) -> Result;

    fn seq_end(&mut self) -> Result;

    fn enum_begin(
        &mut self,
        tag: Option<Tag>,
        label: Option<Label>,
        index: Option<Index>,
    ) -> Result {
        self.tagged_begin(tag, label, index)
    }

    fn enum_end(&mut self, tag: Option<Tag>, label: Option<Label>, index: Option<Index>) -> Result {
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

    fn tag(&mut self, tag: Option<Tag>, label: Option<Label>, index: Option<Index>) -> Result {
        self.tagged_begin(tag, label, index)?;

        // Rust's `Option` is fundamental enough that we handle it specially here
        if let Some(tags::RUST_OPTION_NONE) = tag {
            self.null()?;
        }
        // If the tag has a label then stream it as its value
        else if let Some(label) = label {
            if let Some(label) = label.try_get_static() {
                self.value(label)?;
            } else {
                self.value_computed(&*label)?;
            }
        }
        // If the tag doesn't have a label then stream null
        else {
            self.null()?;
        }

        self.tagged_end(tag, label, index)
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
            self.value(label)?;
        } else {
            self.value_computed(&*label)?;
        }

        self.map_key_end()?;

        self.map_value_begin()
    }

    fn record_value_end(&mut self, label: Label) -> Result {
        let _ = label;

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

        self.seq_value_begin()
    }

    fn tuple_value_end(&mut self, index: Index) -> Result {
        let _ = index;

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
}

macro_rules! impl_stream_forward {
    ({ $($r:tt)* } => $bind:ident => { $($forward:tt)* }) => {
        $($r)* {
            fn value<V: Value + ?Sized>(&mut self, v: &'sval V) -> Result {
                let $bind = self;
                ($($forward)*).value(v)
            }

            fn value_computed<V: Value + ?Sized>(&mut self, v: &V) -> Result {
                let $bind = self;
                ($($forward)*).value_computed(v)
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

            fn tag(&mut self, tag: Option<Tag>, label: Option<Label>, index: Option<Index>) -> Result {
                let $bind = self;
                ($($forward)*).tag(tag, label, index)
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
        }
    };
}

impl_stream_forward!({ impl<'sval, 'a, S: ?Sized> Stream<'sval> for &'a mut S where S: Stream<'sval> } => x => { **x });

#[cfg(feature = "alloc")]
mod alloc_support {
    use super::*;

    use crate::std::boxed::Box;

    impl_stream_forward!({ impl<'sval, 'a, S: ?Sized> Stream<'sval> for Box<S> where S: Stream<'sval> } => x => { **x });
}

pub(crate) fn stream_computed<'a, 'b>(
    stream: &mut (impl Stream<'a> + ?Sized),
    value: &'b (impl Value + ?Sized),
) -> Result {
    struct Computed<S>(S);

    impl<'a, 'b, 'c, S: Stream<'a>> Stream<'b> for Computed<S> {
        fn value_computed<V: Value + ?Sized>(&mut self, v: &V) -> Result {
            self.0.value_computed(v)
        }

        fn text_fragment(&mut self, fragment: &'b str) -> Result {
            self.0.text_fragment_computed(fragment)
        }

        fn binary_fragment(&mut self, fragment: &'b [u8]) -> Result {
            self.0.binary_fragment_computed(fragment)
        }

        fn null(&mut self) -> Result {
            self.0.null()
        }

        fn u8(&mut self, v: u8) -> Result {
            self.0.u8(v)
        }

        fn u16(&mut self, v: u16) -> Result {
            self.0.u16(v)
        }

        fn u32(&mut self, v: u32) -> Result {
            self.0.u32(v)
        }

        fn u64(&mut self, v: u64) -> Result {
            self.0.u64(v)
        }

        fn u128(&mut self, v: u128) -> Result {
            self.0.u128(v)
        }

        fn i8(&mut self, v: i8) -> Result {
            self.0.i8(v)
        }

        fn i16(&mut self, v: i16) -> Result {
            self.0.i16(v)
        }

        fn i32(&mut self, v: i32) -> Result {
            self.0.i32(v)
        }

        fn i64(&mut self, v: i64) -> Result {
            self.0.i64(v)
        }

        fn i128(&mut self, v: i128) -> Result {
            self.0.i128(v)
        }

        fn f32(&mut self, v: f32) -> Result {
            self.0.f32(v)
        }

        fn f64(&mut self, v: f64) -> Result {
            self.0.f64(v)
        }

        fn bool(&mut self, v: bool) -> Result {
            self.0.bool(v)
        }

        fn text_begin(&mut self, num_bytes_hint: Option<usize>) -> Result {
            self.0.text_begin(num_bytes_hint)
        }

        fn text_fragment_computed(&mut self, fragment: &str) -> Result {
            self.0.text_fragment_computed(fragment)
        }

        fn text_end(&mut self) -> Result {
            self.0.text_end()
        }

        fn binary_begin(&mut self, num_bytes_hint: Option<usize>) -> Result {
            self.0.binary_begin(num_bytes_hint)
        }

        fn binary_fragment_computed(&mut self, fragment: &[u8]) -> Result {
            self.0.binary_fragment_computed(fragment)
        }

        fn binary_end(&mut self) -> Result {
            self.0.binary_end()
        }

        fn map_begin(&mut self, num_entries_hint: Option<usize>) -> Result {
            self.0.map_begin(num_entries_hint)
        }

        fn map_key_begin(&mut self) -> Result {
            self.0.map_key_begin()
        }

        fn map_key_end(&mut self) -> Result {
            self.0.map_key_end()
        }

        fn map_value_begin(&mut self) -> Result {
            self.0.map_value_begin()
        }

        fn map_value_end(&mut self) -> Result {
            self.0.map_value_end()
        }

        fn map_end(&mut self) -> Result {
            self.0.map_end()
        }

        fn seq_begin(&mut self, num_entries_hint: Option<usize>) -> Result {
            self.0.seq_begin(num_entries_hint)
        }

        fn seq_value_begin(&mut self) -> Result {
            self.0.seq_value_begin()
        }

        fn seq_value_end(&mut self) -> Result {
            self.0.seq_value_end()
        }

        fn seq_end(&mut self) -> Result {
            self.0.seq_end()
        }

        fn tagged_begin(
            &mut self,
            tag: Option<Tag>,
            label: Option<Label>,
            index: Option<Index>,
        ) -> Result {
            self.0.tagged_begin(tag, label, index)
        }

        fn tagged_end(
            &mut self,
            tag: Option<Tag>,
            label: Option<Label>,
            index: Option<Index>,
        ) -> Result {
            self.0.tagged_end(tag, label, index)
        }

        fn tag(&mut self, tag: Option<Tag>, label: Option<Label>, index: Option<Index>) -> Result {
            self.0.tag(tag, label, index)
        }

        fn record_begin(
            &mut self,
            tag: Option<Tag>,
            label: Option<Label>,
            index: Option<Index>,
            num_entries: Option<usize>,
        ) -> Result {
            self.0.record_begin(tag, label, index, num_entries)
        }

        fn record_value_begin(&mut self, label: Label) -> Result {
            self.0.record_value_begin(label)
        }

        fn record_value_end(&mut self, label: Label) -> Result {
            self.0.record_value_end(label)
        }

        fn record_end(
            &mut self,
            tag: Option<Tag>,
            label: Option<Label>,
            index: Option<Index>,
        ) -> Result {
            self.0.record_end(tag, label, index)
        }

        fn tuple_begin(
            &mut self,
            tag: Option<Tag>,
            label: Option<Label>,
            index: Option<Index>,
            num_entries: Option<usize>,
        ) -> Result {
            self.0.tuple_begin(tag, label, index, num_entries)
        }

        fn tuple_value_begin(&mut self, index: Index) -> Result {
            self.0.tuple_value_begin(index)
        }

        fn tuple_value_end(&mut self, index: Index) -> Result {
            self.0.tuple_value_end(index)
        }

        fn tuple_end(
            &mut self,
            tag: Option<Tag>,
            label: Option<Label>,
            index: Option<Index>,
        ) -> Result {
            self.0.tuple_end(tag, label, index)
        }

        fn enum_begin(
            &mut self,
            tag: Option<Tag>,
            label: Option<Label>,
            index: Option<Index>,
        ) -> Result {
            self.0.enum_begin(tag, label, index)
        }

        fn enum_end(
            &mut self,
            tag: Option<Tag>,
            label: Option<Label>,
            index: Option<Index>,
        ) -> Result {
            self.0.enum_end(tag, label, index)
        }
    }

    value.stream(&mut Computed(stream))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stream_computed() {
        struct ComputedValue(usize);

        impl Value for ComputedValue {
            fn stream<'sval, S: Stream<'sval> + ?Sized>(&'sval self, stream: &mut S) -> Result {
                if self.0 == 0 {
                    stream.bool(true)
                } else {
                    stream.value_computed(&ComputedValue(self.0 - 1))
                }
            }
        }

        assert_eq!(true, ComputedValue(5).to_bool().unwrap());
    }
}
