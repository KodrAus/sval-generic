mod private {
    pub trait DispatchStream<'sval> {
        fn dispatch_is_text_based(&self) -> bool;

        fn dispatch_dynamic_begin(&mut self) -> sval::Result;

        fn dispatch_dynamic_end(&mut self) -> sval::Result;

        fn dispatch_unit(&mut self) -> sval::Result;

        fn dispatch_null(&mut self) -> sval::Result;

        fn dispatch_u8(&mut self, value: u8) -> sval::Result;

        fn dispatch_u16(&mut self, value: u16) -> sval::Result;

        fn dispatch_u32(&mut self, value: u32) -> sval::Result;

        fn dispatch_u64(&mut self, value: u64) -> sval::Result;

        fn dispatch_u128(&mut self, value: u128) -> sval::Result;

        fn dispatch_i8(&mut self, value: i8) -> sval::Result;

        fn dispatch_i16(&mut self, value: i16) -> sval::Result;

        fn dispatch_i32(&mut self, value: i32) -> sval::Result;

        fn dispatch_i64(&mut self, value: i64) -> sval::Result;

        fn dispatch_i128(&mut self, value: i128) -> sval::Result;

        fn dispatch_f32(&mut self, value: f32) -> sval::Result;

        fn dispatch_f64(&mut self, value: f64) -> sval::Result;

        fn dispatch_bool(&mut self, value: bool) -> sval::Result;

        fn dispatch_text_begin(&mut self, num_bytes_hint: Option<usize>) -> sval::Result;

        fn dispatch_text_end(&mut self) -> sval::Result;

        fn dispatch_text_fragment(&mut self, fragment: &'sval str) -> sval::Result;

        fn dispatch_text_fragment_computed(&mut self, fragment: &str) -> sval::Result;

        fn dispatch_binary_begin(&mut self, num_bytes_hint: Option<usize>) -> sval::Result;

        fn dispatch_binary_end(&mut self) -> sval::Result;

        fn dispatch_binary_fragment(&mut self, fragment: &'sval [u8]) -> sval::Result;

        fn dispatch_binary_fragment_computed(&mut self, fragment: &[u8]) -> sval::Result;

        fn dispatch_map_begin(&mut self, num_entries_hint: Option<usize>) -> sval::Result;

        fn dispatch_map_end(&mut self) -> sval::Result;

        fn dispatch_map_key_begin(&mut self) -> sval::Result;

        fn dispatch_map_key_end(&mut self) -> sval::Result;

        fn dispatch_map_value_begin(&mut self) -> sval::Result;

        fn dispatch_map_value_end(&mut self) -> sval::Result;

        fn dispatch_seq_begin(&mut self, num_elems_hint: Option<usize>) -> sval::Result;

        fn dispatch_seq_end(&mut self) -> sval::Result;

        fn dispatch_seq_value_begin(&mut self) -> sval::Result;

        fn dispatch_seq_value_end(&mut self) -> sval::Result;

        fn dispatch_tagged_begin(&mut self, tag: sval::Tag) -> sval::Result;

        fn dispatch_tagged_end(&mut self, tag: sval::Tag) -> sval::Result;

        fn dispatch_constant_begin(&mut self, tag: sval::Tag) -> sval::Result;

        fn dispatch_constant_end(&mut self, tag: sval::Tag) -> sval::Result;

        fn dispatch_record_begin(
            &mut self,
            tag: sval::Tag,
            num_entries: Option<usize>,
        ) -> sval::Result;

        fn dispatch_record_value_begin(&mut self, label: sval::Label) -> sval::Result;

        fn dispatch_record_value_end(&mut self, label: sval::Label) -> sval::Result;

        fn dispatch_record_end(&mut self, tag: sval::Tag) -> sval::Result;

        fn dispatch_tuple_begin(
            &mut self,
            tag: sval::Tag,
            num_entries: Option<usize>,
        ) -> sval::Result;

        fn dispatch_tuple_value_begin(&mut self, index: u32) -> sval::Result;

        fn dispatch_tuple_value_end(&mut self, index: u32) -> sval::Result;

        fn dispatch_tuple_end(&mut self, tag: sval::Tag) -> sval::Result;

        fn dispatch_enum_begin(&mut self, tag: sval::Tag) -> sval::Result;

        fn dispatch_enum_end(&mut self, tag: sval::Tag) -> sval::Result;

        fn dispatch_optional_some_begin(&mut self) -> sval::Result;

        fn dispatch_optional_some_end(&mut self) -> sval::Result;

        fn dispatch_optional_none(&mut self) -> sval::Result;

        fn dispatch_constant_size_begin(&mut self) -> sval::Result;

        fn dispatch_constant_size_end(&mut self) -> sval::Result;

        fn dispatch_int_begin(&mut self) -> sval::Result;

        fn dispatch_int_end(&mut self) -> sval::Result;

        fn dispatch_binfloat_begin(&mut self) -> sval::Result;

        fn dispatch_binfloat_end(&mut self) -> sval::Result;

        fn dispatch_decfloat_begin(&mut self) -> sval::Result;

        fn dispatch_decfloat_end(&mut self) -> sval::Result;
    }

    pub trait EraseStream<'sval> {
        fn erase_stream_ref(&self) -> crate::private::Erased<&dyn DispatchStream<'sval>>;
        fn erase_stream(&mut self) -> crate::private::Erased<&mut dyn DispatchStream<'sval>>;
    }
}

pub trait Stream<'sval>: private::EraseStream<'sval> {}

impl<'sval, R: sval::Stream<'sval>> Stream<'sval> for R {}

impl<'sval, R: sval::Stream<'sval>> private::EraseStream<'sval> for R {
    fn erase_stream_ref(&self) -> crate::private::Erased<&dyn private::DispatchStream<'sval>> {
        crate::private::Erased(self)
    }

    fn erase_stream(&mut self) -> crate::private::Erased<&mut dyn private::DispatchStream<'sval>> {
        crate::private::Erased(self)
    }
}

impl<'sval, R: sval::Stream<'sval>> private::DispatchStream<'sval> for R {
    fn dispatch_is_text_based(&self) -> bool {
        self.is_text_based()
    }

    fn dispatch_dynamic_begin(&mut self) -> sval::Result {
        self.dynamic_begin()
    }

    fn dispatch_dynamic_end(&mut self) -> sval::Result {
        self.dynamic_end()
    }

    fn dispatch_unit(&mut self) -> sval::Result {
        self.unit()
    }

    fn dispatch_null(&mut self) -> sval::Result {
        self.null()
    }

    fn dispatch_u8(&mut self, value: u8) -> sval::Result {
        self.u8(value)
    }

    fn dispatch_u16(&mut self, value: u16) -> sval::Result {
        self.u16(value)
    }

    fn dispatch_u32(&mut self, value: u32) -> sval::Result {
        self.u32(value)
    }

    fn dispatch_u64(&mut self, value: u64) -> sval::Result {
        self.u64(value)
    }

    fn dispatch_u128(&mut self, value: u128) -> sval::Result {
        self.u128(value)
    }

    fn dispatch_i8(&mut self, value: i8) -> sval::Result {
        self.i8(value)
    }

    fn dispatch_i16(&mut self, value: i16) -> sval::Result {
        self.i16(value)
    }

    fn dispatch_i32(&mut self, value: i32) -> sval::Result {
        self.i32(value)
    }

    fn dispatch_i64(&mut self, value: i64) -> sval::Result {
        self.i64(value)
    }

    fn dispatch_i128(&mut self, value: i128) -> sval::Result {
        self.i128(value)
    }

    fn dispatch_f32(&mut self, value: f32) -> sval::Result {
        self.f32(value)
    }

    fn dispatch_f64(&mut self, value: f64) -> sval::Result {
        self.f64(value)
    }

    fn dispatch_bool(&mut self, value: bool) -> sval::Result {
        self.bool(value)
    }

    fn dispatch_text_begin(&mut self, num_bytes_hint: Option<usize>) -> sval::Result {
        self.text_begin(num_bytes_hint)
    }

    fn dispatch_text_end(&mut self) -> sval::Result {
        self.text_end()
    }

    fn dispatch_text_fragment(&mut self, fragment: &'sval str) -> sval::Result {
        self.text_fragment(fragment)
    }

    fn dispatch_text_fragment_computed(&mut self, fragment: &str) -> sval::Result {
        self.text_fragment_computed(fragment)
    }

    fn dispatch_binary_begin(&mut self, num_bytes_hint: Option<usize>) -> sval::Result {
        self.binary_begin(num_bytes_hint)
    }

    fn dispatch_binary_end(&mut self) -> sval::Result {
        self.binary_end()
    }

    fn dispatch_binary_fragment(&mut self, fragment: &'sval [u8]) -> sval::Result {
        self.binary_fragment(fragment)
    }

    fn dispatch_binary_fragment_computed(&mut self, fragment: &[u8]) -> sval::Result {
        self.binary_fragment_computed(fragment)
    }

    fn dispatch_map_begin(&mut self, num_entries_hint: Option<usize>) -> sval::Result {
        self.map_begin(num_entries_hint)
    }

    fn dispatch_map_end(&mut self) -> sval::Result {
        self.map_end()
    }

    fn dispatch_map_key_begin(&mut self) -> sval::Result {
        self.map_key_begin()
    }

    fn dispatch_map_key_end(&mut self) -> sval::Result {
        self.map_key_end()
    }

    fn dispatch_map_value_begin(&mut self) -> sval::Result {
        self.map_value_begin()
    }

    fn dispatch_map_value_end(&mut self) -> sval::Result {
        self.map_value_end()
    }

    fn dispatch_seq_begin(&mut self, num_elems_hint: Option<usize>) -> sval::Result {
        self.seq_begin(num_elems_hint)
    }

    fn dispatch_seq_end(&mut self) -> sval::Result {
        self.seq_end()
    }

    fn dispatch_seq_value_begin(&mut self) -> sval::Result {
        self.seq_value_begin()
    }

    fn dispatch_seq_value_end(&mut self) -> sval::Result {
        self.seq_value_end()
    }

    fn dispatch_tagged_begin(&mut self, tag: sval::Tag) -> sval::Result {
        self.tagged_begin(tag)
    }

    fn dispatch_tagged_end(&mut self, tag: sval::Tag) -> sval::Result {
        self.tagged_end(tag)
    }

    fn dispatch_constant_begin(&mut self, tag: sval::Tag) -> sval::Result {
        self.constant_begin(tag)
    }

    fn dispatch_constant_end(&mut self, tag: sval::Tag) -> sval::Result {
        self.constant_end(tag)
    }

    fn dispatch_record_begin(
        &mut self,
        tag: sval::Tag,
        num_entries: Option<usize>,
    ) -> sval::Result {
        self.record_begin(tag, num_entries)
    }

    fn dispatch_record_value_begin(&mut self, label: sval::Label) -> sval::Result {
        self.record_value_begin(label)
    }

    fn dispatch_record_value_end(&mut self, label: sval::Label) -> sval::Result {
        self.record_value_end(label)
    }

    fn dispatch_record_end(&mut self, tag: sval::Tag) -> sval::Result {
        self.record_end(tag)
    }

    fn dispatch_tuple_begin(&mut self, tag: sval::Tag, num_entries: Option<usize>) -> sval::Result {
        self.tuple_begin(tag, num_entries)
    }

    fn dispatch_tuple_value_begin(&mut self, index: u32) -> sval::Result {
        self.tuple_value_begin(index)
    }

    fn dispatch_tuple_value_end(&mut self, index: u32) -> sval::Result {
        self.tuple_value_end(index)
    }

    fn dispatch_tuple_end(&mut self, tag: sval::Tag) -> sval::Result {
        self.tuple_end(tag)
    }

    fn dispatch_enum_begin(&mut self, tag: sval::Tag) -> sval::Result {
        self.enum_begin(tag)
    }

    fn dispatch_enum_end(&mut self, tag: sval::Tag) -> sval::Result {
        self.enum_end(tag)
    }

    fn dispatch_optional_some_begin(&mut self) -> sval::Result {
        self.optional_some_begin()
    }

    fn dispatch_optional_some_end(&mut self) -> sval::Result {
        self.optional_some_end()
    }

    fn dispatch_optional_none(&mut self) -> sval::Result {
        self.optional_none()
    }

    fn dispatch_constant_size_begin(&mut self) -> sval::Result {
        self.constant_size_begin()
    }

    fn dispatch_constant_size_end(&mut self) -> sval::Result {
        self.constant_size_end()
    }

    fn dispatch_int_begin(&mut self) -> sval::Result {
        self.int_begin()
    }

    fn dispatch_int_end(&mut self) -> sval::Result {
        self.int_end()
    }

    fn dispatch_binfloat_begin(&mut self) -> sval::Result {
        self.binfloat_begin()
    }

    fn dispatch_binfloat_end(&mut self) -> sval::Result {
        self.binfloat_end()
    }

    fn dispatch_decfloat_begin(&mut self) -> sval::Result {
        self.decfloat_begin()
    }

    fn dispatch_decfloat_end(&mut self) -> sval::Result {
        self.decfloat_end()
    }
}

macro_rules! impl_stream {
    ($($impl:tt)*) => {
        $($impl)* {
            fn is_text_based(&self) -> bool {
                self.erase_stream_ref().0.dispatch_is_text_based()
            }

            fn dynamic_begin(&mut self) -> sval::Result {
                self.erase_stream().0.dispatch_dynamic_begin()
            }

            fn dynamic_end(&mut self) -> sval::Result {
                self.erase_stream().0.dispatch_dynamic_end()
            }

            fn unit(&mut self) -> sval::Result {
                self.erase_stream().0.dispatch_unit()
            }

            fn null(&mut self) -> sval::Result {
                self.erase_stream().0.dispatch_null()
            }

            fn u8(&mut self, value: u8) -> sval::Result {
                self.erase_stream().0.dispatch_u8(value)
            }

            fn u16(&mut self, value: u16) -> sval::Result {
                self.erase_stream().0.dispatch_u16(value)
            }

            fn u32(&mut self, value: u32) -> sval::Result {
                self.erase_stream().0.dispatch_u32(value)
            }

            fn u64(&mut self, value: u64) -> sval::Result {
                self.erase_stream().0.dispatch_u64(value)
            }

            fn u128(&mut self, value: u128) -> sval::Result {
                self.erase_stream().0.dispatch_u128(value)
            }

            fn i8(&mut self, value: i8) -> sval::Result {
                self.erase_stream().0.dispatch_i8(value)
            }

            fn i16(&mut self, value: i16) -> sval::Result {
                self.erase_stream().0.dispatch_i16(value)
            }

            fn i32(&mut self, value: i32) -> sval::Result {
                self.erase_stream().0.dispatch_i32(value)
            }

            fn i64(&mut self, value: i64) -> sval::Result {
                self.erase_stream().0.dispatch_i64(value)
            }

            fn i128(&mut self, value: i128) -> sval::Result {
                self.erase_stream().0.dispatch_i128(value)
            }

            fn f32(&mut self, value: f32) -> sval::Result {
                self.erase_stream().0.dispatch_f32(value)
            }

            fn f64(&mut self, value: f64) -> sval::Result {
                self.erase_stream().0.dispatch_f64(value)
            }

            fn bool(&mut self, value: bool) -> sval::Result {
                self.erase_stream().0.dispatch_bool(value)
            }

            fn text_begin(&mut self, num_bytes_hint: Option<usize>) -> sval::Result {
                self.erase_stream().0.dispatch_text_begin(num_bytes_hint)
            }

            fn text_end(&mut self) -> sval::Result {
                self.erase_stream().0.dispatch_text_end()
            }

            fn text_fragment(&mut self, fragment: &'sval str) -> sval::Result {
                self.erase_stream().0.dispatch_text_fragment(&fragment)
            }

            fn text_fragment_computed(&mut self, fragment: &str) -> sval::Result {
                self.erase_stream().0.dispatch_text_fragment_computed(&fragment)
            }

            fn binary_begin(&mut self, num_bytes_hint: Option<usize>) -> sval::Result {
                self.erase_stream().0.dispatch_binary_begin(num_bytes_hint)
            }

            fn binary_end(&mut self) -> sval::Result {
                self.erase_stream().0.dispatch_binary_end()
            }

            fn binary_fragment(&mut self, fragment: &'sval [u8]) -> sval::Result {
                self.erase_stream().0.dispatch_binary_fragment(&fragment)
            }

            fn binary_fragment_computed(&mut self, fragment: &[u8]) -> sval::Result {
                self.erase_stream().0.dispatch_binary_fragment_computed(fragment.as_ref())
            }

            fn map_begin(&mut self, num_entries_hint: Option<usize>) -> sval::Result {
                self.erase_stream().0.dispatch_map_begin(num_entries_hint)
            }

            fn map_end(&mut self) -> sval::Result {
                self.erase_stream().0.dispatch_map_end()
            }

            fn map_key_begin(&mut self) -> sval::Result {
                self.erase_stream().0.dispatch_map_key_begin()
            }

            fn map_key_end(&mut self) -> sval::Result {
                self.erase_stream().0.dispatch_map_key_end()
            }

            fn map_value_begin(&mut self) -> sval::Result {
                self.erase_stream().0.dispatch_map_value_begin()
            }

            fn map_value_end(&mut self) -> sval::Result {
                self.erase_stream().0.dispatch_map_value_end()
            }

            fn seq_begin(&mut self, num_elems_hint: Option<usize>) -> sval::Result {
                self.erase_stream().0.dispatch_seq_begin(num_elems_hint)
            }

            fn seq_end(&mut self) -> sval::Result {
                self.erase_stream().0.dispatch_seq_end()
            }

            fn seq_value_begin(&mut self) -> sval::Result {
                self.erase_stream().0.dispatch_seq_value_begin()
            }

            fn seq_value_end(&mut self) -> sval::Result {
                self.erase_stream().0.dispatch_seq_value_end()
            }

            fn tagged_begin(&mut self, tag: sval::Tag) -> sval::Result {
                self.erase_stream().0.dispatch_tagged_begin(tag)
            }

            fn tagged_end(&mut self, tag: sval::Tag) -> sval::Result {
                self.erase_stream().0.dispatch_tagged_end(tag)
            }

            fn constant_begin(&mut self, tag: sval::Tag) -> sval::Result {
                self.erase_stream().0.dispatch_constant_begin(tag)
            }

            fn constant_end(&mut self, tag: sval::Tag) -> sval::Result {
                self.erase_stream().0.dispatch_constant_end(tag)
            }

            fn record_begin(&mut self, tag: sval::Tag, num_entries_hint: Option<usize>) -> sval::Result {
                self.erase_stream().0.dispatch_record_begin(tag, num_entries_hint)
            }

            fn record_value_begin(&mut self, label: sval::Label) -> sval::Result {
                self.erase_stream().0.dispatch_record_value_begin(label)
            }

            fn record_value_end(&mut self, label: sval::Label) -> sval::Result {
                self.erase_stream().0.dispatch_record_value_end(label)
            }

            fn record_end(&mut self, tag: sval::Tag) -> sval::Result {
                self.erase_stream().0.dispatch_record_end(tag)
            }

            fn tuple_begin(&mut self, tag: sval::Tag, num_entries_hint: Option<usize>) -> sval::Result {
                self.erase_stream().0.dispatch_tuple_begin(tag, num_entries_hint)
            }

            fn tuple_value_begin(&mut self, index: u32) -> sval::Result {
                self.erase_stream().0.dispatch_tuple_value_begin(index)
            }

            fn tuple_value_end(&mut self, index: u32) -> sval::Result {
                self.erase_stream().0.dispatch_tuple_value_end(index)
            }

            fn tuple_end(&mut self, tag: sval::Tag) -> sval::Result {
                self.erase_stream().0.dispatch_tuple_end(tag)
            }

            fn enum_begin(&mut self, tag: sval::Tag) -> sval::Result {
                self.erase_stream().0.dispatch_enum_begin(tag)
            }

            fn enum_end(&mut self, tag: sval::Tag) -> sval::Result {
                self.erase_stream().0.dispatch_enum_end(tag)
            }

            fn optional_some_begin(&mut self) -> sval::Result {
                self.erase_stream().0.dispatch_optional_some_begin()
            }

            fn optional_some_end(&mut self) -> sval::Result {
                self.erase_stream().0.dispatch_optional_some_end()
            }

            fn optional_none(&mut self) -> sval::Result {
                self.erase_stream().0.dispatch_optional_none()
            }

            fn constant_size_begin(&mut self) -> sval::Result {
                self.erase_stream().0.dispatch_constant_size_begin()
            }

            fn constant_size_end(&mut self) -> sval::Result {
                self.erase_stream().0.dispatch_constant_size_end()
            }

            fn int_begin(&mut self) -> sval::Result {
                self.erase_stream().0.dispatch_int_begin()
            }

            fn int_end(&mut self) -> sval::Result {
                self.erase_stream().0.dispatch_int_end()
            }

            fn binfloat_begin(&mut self) -> sval::Result {
                self.erase_stream().0.dispatch_binfloat_begin()
            }

            fn binfloat_end(&mut self) -> sval::Result {
                self.erase_stream().0.dispatch_binfloat_end()
            }

            fn decfloat_begin(&mut self) -> sval::Result {
                self.erase_stream().0.dispatch_decfloat_begin()
            }

            fn decfloat_end(&mut self) -> sval::Result {
                self.erase_stream().0.dispatch_decfloat_end()
            }
        }
    }
}

impl_stream!(impl<'sval, 'd> sval::Stream<'sval> for dyn Stream<'sval> + 'd);
impl_stream!(impl<'sval, 'd> sval::Stream<'sval> for dyn Stream<'sval> + Send + 'd);
impl_stream!(impl<'sval, 'd> sval::Stream<'sval> for dyn Stream<'sval> + Send + Sync + 'd);
