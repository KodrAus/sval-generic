mod private {
    use crate::Source;

    pub trait DispatchReceiver<'a> {
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

        fn dispatch_char(&mut self, value: char) -> sval::Result;

        fn dispatch_str(&mut self, value: &'a str) -> sval::Result;

        fn dispatch_text_begin(&mut self, num_bytes_hint: Option<usize>) -> sval::Result;

        fn dispatch_text_end(&mut self) -> sval::Result;

        fn dispatch_text_fragment(&mut self, fragment: &'a str) -> sval::Result;

        fn dispatch_text_fragment_computed(&mut self, fragment: &str) -> sval::Result;

        fn dispatch_bytes(&mut self, value: &'a [u8]) -> sval::Result;

        fn dispatch_binary_begin(&mut self, num_bytes_hint: Option<usize>) -> sval::Result;

        fn dispatch_binary_end(&mut self) -> sval::Result;

        fn dispatch_binary_fragment(&mut self, fragment: &'a [u8]) -> sval::Result;

        fn dispatch_binary_fragment_computed(&mut self, fragment: &[u8]) -> sval::Result;

        fn dispatch_map_begin(&mut self, num_entries_hint: Option<usize>) -> sval::Result;

        fn dispatch_map_end(&mut self) -> sval::Result;

        fn dispatch_map_key_begin(&mut self) -> sval::Result;

        fn dispatch_map_key_end(&mut self) -> sval::Result;

        fn dispatch_map_value_begin(&mut self) -> sval::Result;

        fn dispatch_map_value_end(&mut self) -> sval::Result;

        fn dispatch_map_key<'k: 'a>(&mut self, key: &mut dyn Source<'k>) -> sval::Result;

        fn dispatch_map_value<'v: 'a>(&mut self, value: &mut dyn Source<'v>) -> sval::Result;

        fn dispatch_seq_begin(&mut self, num_elems_hint: Option<usize>) -> sval::Result;

        fn dispatch_seq_end(&mut self) -> sval::Result;

        fn dispatch_seq_value_begin(&mut self) -> sval::Result;

        fn dispatch_seq_value_end(&mut self) -> sval::Result;

        fn dispatch_seq_value<'e: 'a>(&mut self, value: &mut dyn Source<'e>) -> sval::Result;

        fn dispatch_tagged_begin(&mut self, tag: sval::data::Tag) -> sval::Result;

        fn dispatch_tagged_end(&mut self) -> sval::Result;

        fn dispatch_constant_begin(&mut self, tag: sval::data::Tag) -> sval::Result;

        fn dispatch_constant_end(&mut self) -> sval::Result;

        fn dispatch_struct_map_begin(
            &mut self,
            tag: sval::data::Tag,
            num_entries_hint: Option<usize>,
        ) -> sval::Result;

        fn dispatch_struct_map_key_begin(&mut self, tag: sval::data::Tag) -> sval::Result;

        fn dispatch_struct_map_key_end(&mut self) -> sval::Result;

        fn dispatch_struct_map_value_begin(&mut self, tag: sval::data::Tag) -> sval::Result;

        fn dispatch_struct_map_value_end(&mut self) -> sval::Result;

        fn dispatch_struct_map_end(&mut self) -> sval::Result;

        fn dispatch_struct_map_key<'k: 'a>(
            &mut self,
            tag: sval::data::Tag,
            key: &mut dyn Source<'k>,
        ) -> sval::Result;

        fn dispatch_struct_map_value<'v: 'a>(
            &mut self,
            tag: sval::data::Tag,
            value: &mut dyn Source<'v>,
        ) -> sval::Result;

        fn dispatch_struct_seq_begin(
            &mut self,
            tag: sval::data::Tag,
            num_entries_hint: Option<usize>,
        ) -> sval::Result;

        fn dispatch_struct_seq_value_begin(&mut self, tag: sval::data::Tag) -> sval::Result;

        fn dispatch_struct_seq_value_end(&mut self) -> sval::Result;

        fn dispatch_struct_seq_end(&mut self) -> sval::Result;

        fn dispatch_struct_seq_value<'v: 'a>(
            &mut self,
            tag: sval::data::Tag,
            value: &mut dyn Source<'v>,
        ) -> sval::Result;

        fn dispatch_enum_begin(&mut self, tag: sval::data::Tag) -> sval::Result;

        fn dispatch_enum_end(&mut self) -> sval::Result;

        fn dispatch_nullable_begin(&mut self) -> sval::Result;

        fn dispatch_nullable_end(&mut self) -> sval::Result;

        fn dispatch_fixed_size_begin(&mut self) -> sval::Result;

        fn dispatch_fixed_size_end(&mut self) -> sval::Result;

        fn dispatch_int_begin(&mut self) -> sval::Result;

        fn dispatch_int_end(&mut self) -> sval::Result;

        fn dispatch_binfloat_begin(&mut self) -> sval::Result;

        fn dispatch_binfloat_end(&mut self) -> sval::Result;

        fn dispatch_decfloat_begin(&mut self) -> sval::Result;

        fn dispatch_decfloat_end(&mut self) -> sval::Result;

        fn dispatch_app_specific_begin(&mut self, app_specific_id: u128) -> sval::Result;

        fn dispatch_app_specific_end(&mut self, app_specific_id: u128) -> sval::Result;
    }

    pub trait EraseReceiver<'a> {
        fn erase_receiver_ref(&self) -> crate::private::Erased<&dyn DispatchReceiver<'a>>;
        fn erase_receiver(&mut self) -> crate::private::Erased<&mut dyn DispatchReceiver<'a>>;
    }
}

use crate::Source;

pub trait Receiver<'a>: private::EraseReceiver<'a> {}

impl<'a, R: sval::Receiver<'a>> Receiver<'a> for R {}

impl<'a, R: sval::Receiver<'a>> private::EraseReceiver<'a> for R {
    fn erase_receiver_ref(&self) -> crate::private::Erased<&dyn private::DispatchReceiver<'a>> {
        crate::private::Erased(self)
    }

    fn erase_receiver(&mut self) -> crate::private::Erased<&mut dyn private::DispatchReceiver<'a>> {
        crate::private::Erased(self)
    }
}

impl<'a, R: sval::Receiver<'a>> private::DispatchReceiver<'a> for R {
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

    fn dispatch_char(&mut self, value: char) -> sval::Result {
        self.char(value)
    }

    fn dispatch_str(&mut self, value: &'a str) -> sval::Result {
        self.str(value)
    }

    fn dispatch_text_begin(&mut self, num_bytes_hint: Option<usize>) -> sval::Result {
        self.text_begin(num_bytes_hint)
    }

    fn dispatch_text_end(&mut self) -> sval::Result {
        self.text_end()
    }

    fn dispatch_text_fragment(&mut self, fragment: &'a str) -> sval::Result {
        self.text_fragment(fragment)
    }

    fn dispatch_text_fragment_computed(&mut self, fragment: &str) -> sval::Result {
        self.text_fragment_computed(fragment)
    }

    fn dispatch_bytes(&mut self, value: &'a [u8]) -> sval::Result {
        self.bytes(value)
    }

    fn dispatch_binary_begin(&mut self, num_bytes_hint: Option<usize>) -> sval::Result {
        self.binary_begin(num_bytes_hint)
    }

    fn dispatch_binary_end(&mut self) -> sval::Result {
        self.binary_end()
    }

    fn dispatch_binary_fragment(&mut self, fragment: &'a [u8]) -> sval::Result {
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

    fn dispatch_map_key<'k: 'a>(&mut self, key: &mut dyn Source<'k>) -> sval::Result {
        self.map_key(key)
    }

    fn dispatch_map_value<'v: 'a>(&mut self, value: &mut dyn Source<'v>) -> sval::Result {
        self.map_value(value)
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

    fn dispatch_seq_value<'e: 'a>(&mut self, elem: &mut dyn Source<'e>) -> sval::Result {
        self.seq_value(elem)
    }

    fn dispatch_tagged_begin(&mut self, tag: sval::data::Tag) -> sval::Result {
        self.tagged_begin(tag)
    }

    fn dispatch_tagged_end(&mut self) -> sval::Result {
        self.tagged_end()
    }

    fn dispatch_constant_begin(&mut self, tag: sval::data::Tag) -> sval::Result {
        self.constant_begin(tag)
    }

    fn dispatch_constant_end(&mut self) -> sval::Result {
        self.constant_end()
    }

    fn dispatch_struct_map_begin(
        &mut self,
        tag: sval::data::Tag,
        num_entries_hint: Option<usize>,
    ) -> sval::Result {
        self.struct_map_begin(tag, num_entries_hint)
    }

    fn dispatch_struct_map_key_begin(&mut self, tag: sval::data::Tag) -> sval::Result {
        self.struct_map_key_begin(tag)
    }

    fn dispatch_struct_map_key_end(&mut self) -> sval::Result {
        self.struct_map_key_end()
    }

    fn dispatch_struct_map_value_begin(&mut self, tag: sval::data::Tag) -> sval::Result {
        self.struct_map_value_begin(tag)
    }

    fn dispatch_struct_map_value_end(&mut self) -> sval::Result {
        self.struct_map_value_end()
    }

    fn dispatch_struct_map_end(&mut self) -> sval::Result {
        self.struct_map_end()
    }

    fn dispatch_struct_map_key<'k: 'a>(
        &mut self,
        tag: sval::data::Tag,
        key: &mut dyn Source<'k>,
    ) -> sval::Result {
        self.struct_map_key(tag, key)
    }

    fn dispatch_struct_map_value<'v: 'a>(
        &mut self,
        tag: sval::data::Tag,
        value: &mut dyn Source<'v>,
    ) -> sval::Result {
        self.struct_map_value(tag, value)
    }

    fn dispatch_struct_seq_begin(
        &mut self,
        tag: sval::data::Tag,
        num_entries_hint: Option<usize>,
    ) -> sval::Result {
        self.struct_seq_begin(tag, num_entries_hint)
    }

    fn dispatch_struct_seq_value_begin(&mut self, tag: sval::data::Tag) -> sval::Result {
        self.struct_seq_value_begin(tag)
    }

    fn dispatch_struct_seq_value_end(&mut self) -> sval::Result {
        self.struct_seq_value_end()
    }

    fn dispatch_struct_seq_end(&mut self) -> sval::Result {
        self.struct_seq_end()
    }

    fn dispatch_struct_seq_value<'v: 'a>(
        &mut self,
        tag: sval::data::Tag,
        value: &mut dyn Source<'v>,
    ) -> sval::Result {
        self.struct_seq_value(tag, value)
    }

    fn dispatch_enum_begin(&mut self, tag: sval::data::Tag) -> sval::Result {
        self.enum_begin(tag)
    }

    fn dispatch_enum_end(&mut self) -> sval::Result {
        self.enum_end()
    }

    fn dispatch_nullable_begin(&mut self) -> sval::Result {
        self.nullable_begin()
    }

    fn dispatch_nullable_end(&mut self) -> sval::Result {
        self.nullable_end()
    }

    fn dispatch_fixed_size_begin(&mut self) -> sval::Result {
        self.fixed_size_begin()
    }

    fn dispatch_fixed_size_end(&mut self) -> sval::Result {
        self.fixed_size_end()
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

    fn dispatch_app_specific_begin(&mut self, app_specific_id: u128) -> sval::Result {
        self.app_specific_begin(app_specific_id)
    }

    fn dispatch_app_specific_end(&mut self, app_specific_id: u128) -> sval::Result {
        self.app_specific_end(app_specific_id)
    }
}

macro_rules! impl_receiver {
    ($($impl:tt)*) => {
        $($impl)* {
            fn is_text_based(&self) -> bool {
                self.erase_receiver_ref().0.dispatch_is_text_based()
            }

            fn dynamic_begin(&mut self) -> sval::Result {
                self.erase_receiver().0.dispatch_dynamic_begin()
            }

            fn dynamic_end(&mut self) -> sval::Result {
                self.erase_receiver().0.dispatch_dynamic_end()
            }

            fn unit(&mut self) -> sval::Result {
                self.erase_receiver().0.dispatch_unit()
            }

            fn null(&mut self) -> sval::Result {
                self.erase_receiver().0.dispatch_null()
            }

            fn u8(&mut self, value: u8) -> sval::Result {
                self.erase_receiver().0.dispatch_u8(value)
            }

            fn u16(&mut self, value: u16) -> sval::Result {
                self.erase_receiver().0.dispatch_u16(value)
            }

            fn u32(&mut self, value: u32) -> sval::Result {
                self.erase_receiver().0.dispatch_u32(value)
            }

            fn u64(&mut self, value: u64) -> sval::Result {
                self.erase_receiver().0.dispatch_u64(value)
            }

            fn u128(&mut self, value: u128) -> sval::Result {
                self.erase_receiver().0.dispatch_u128(value)
            }

            fn i8(&mut self, value: i8) -> sval::Result {
                self.erase_receiver().0.dispatch_i8(value)
            }

            fn i16(&mut self, value: i16) -> sval::Result {
                self.erase_receiver().0.dispatch_i16(value)
            }

            fn i32(&mut self, value: i32) -> sval::Result {
                self.erase_receiver().0.dispatch_i32(value)
            }

            fn i64(&mut self, value: i64) -> sval::Result {
                self.erase_receiver().0.dispatch_i64(value)
            }

            fn i128(&mut self, value: i128) -> sval::Result {
                self.erase_receiver().0.dispatch_i128(value)
            }

            fn f32(&mut self, value: f32) -> sval::Result {
                self.erase_receiver().0.dispatch_f32(value)
            }

            fn f64(&mut self, value: f64) -> sval::Result {
                self.erase_receiver().0.dispatch_f64(value)
            }

            fn bool(&mut self, value: bool) -> sval::Result {
                self.erase_receiver().0.dispatch_bool(value)
            }

            fn char(&mut self, value: char) -> sval::Result {
                self.erase_receiver().0.dispatch_char(value)
            }

            fn str(&mut self, value: &'a str) -> sval::Result {
                self.erase_receiver().0.dispatch_str(value)
            }

            fn text_begin(&mut self, num_bytes_hint: Option<usize>) -> sval::Result {
                self.erase_receiver().0.dispatch_text_begin(num_bytes_hint)
            }

            fn text_end(&mut self) -> sval::Result {
                self.erase_receiver().0.dispatch_text_end()
            }

            fn text_fragment_computed(&mut self, fragment: &str) -> sval::Result {
                self.erase_receiver().0.dispatch_text_fragment_computed(&fragment)
            }

            fn bytes(&mut self, value: &'a [u8]) -> sval::Result {
                self.erase_receiver().0.dispatch_bytes(value)
            }

            fn binary_begin(&mut self, num_bytes_hint: Option<usize>) -> sval::Result {
                self.erase_receiver().0.dispatch_binary_begin(num_bytes_hint)
            }

            fn binary_end(&mut self) -> sval::Result {
                self.erase_receiver().0.dispatch_binary_end()
            }

            fn binary_fragment_computed(&mut self, fragment: &[u8]) -> sval::Result {
                self.erase_receiver().0.dispatch_binary_fragment_computed(fragment.as_ref())
            }

            fn map_begin(&mut self, num_entries_hint: Option<usize>) -> sval::Result {
                self.erase_receiver().0.dispatch_map_begin(num_entries_hint)
            }

            fn map_end(&mut self) -> sval::Result {
                self.erase_receiver().0.dispatch_map_end()
            }

            fn map_key_begin(&mut self) -> sval::Result {
                self.erase_receiver().0.dispatch_map_key_begin()
            }

            fn map_key_end(&mut self) -> sval::Result {
                self.erase_receiver().0.dispatch_map_key_end()
            }

            fn map_value_begin(&mut self) -> sval::Result {
                self.erase_receiver().0.dispatch_map_value_begin()
            }

            fn map_value_end(&mut self) -> sval::Result {
                self.erase_receiver().0.dispatch_map_value_end()
            }

            fn map_key<'k: 'a, K: sval::Source<'k>>(&mut self, mut key: K) -> sval::Result {
                self.erase_receiver().0.dispatch_map_key(&mut key)
            }

            fn map_value<'v: 'a, V: sval::Source<'v>>(&mut self, mut value: V) -> sval::Result {
                self.erase_receiver().0.dispatch_map_value(&mut value)
            }

            fn seq_begin(&mut self, num_elems_hint: Option<usize>) -> sval::Result {
                self.erase_receiver().0.dispatch_seq_begin(num_elems_hint)
            }

            fn seq_end(&mut self) -> sval::Result {
                self.erase_receiver().0.dispatch_seq_end()
            }

            fn seq_value_begin(&mut self) -> sval::Result {
                self.erase_receiver().0.dispatch_seq_value_begin()
            }

            fn seq_value_end(&mut self) -> sval::Result {
                self.erase_receiver().0.dispatch_seq_value_end()
            }

            fn seq_value<'e: 'a, V: sval::Source<'e>>(&mut self, mut value: V) -> sval::Result {
                self.erase_receiver().0.dispatch_seq_value(&mut value)
            }

            fn tagged_begin(&mut self, tag: sval::data::Tag) -> sval::Result {
                self.erase_receiver().0.dispatch_tagged_begin(tag)
            }

            fn tagged_end(&mut self) -> sval::Result {
                self.erase_receiver().0.dispatch_tagged_end()
            }

            fn constant_begin(&mut self, tag: sval::data::Tag) -> sval::Result {
                self.erase_receiver().0.dispatch_constant_begin(tag)
            }

            fn constant_end(&mut self) -> sval::Result {
                self.erase_receiver().0.dispatch_constant_end()
            }

            fn struct_map_begin(&mut self, tag: sval::data::Tag, num_entries_hint: Option<usize>) -> sval::Result {
                self.erase_receiver().0.dispatch_struct_map_begin(tag, num_entries_hint)
            }

            fn struct_map_key_begin(&mut self, tag: sval::data::Tag) -> sval::Result {
                self.erase_receiver().0.dispatch_struct_map_key_begin(tag)
            }

            fn struct_map_key_end(&mut self) -> sval::Result {
                self.erase_receiver().0.dispatch_struct_map_key_end()
            }

            fn struct_map_value_begin(&mut self, tag: sval::data::Tag) -> sval::Result {
                self.erase_receiver().0.dispatch_struct_map_value_begin(tag)
            }

            fn struct_map_value_end(&mut self) -> sval::Result {
                self.erase_receiver().0.dispatch_struct_map_value_end()
            }

            fn struct_map_end(&mut self) -> sval::Result {
                self.erase_receiver().0.dispatch_struct_map_end()
            }

            fn struct_map_key<'k: 'a, K: sval::Source<'k>>(&mut self, tag: sval::data::Tag, mut key: K) -> sval::Result {
                self.erase_receiver().0.dispatch_struct_map_key(tag, &mut key)
            }

            fn struct_map_value<'v: 'a, V: sval::Source<'v>>(&mut self, tag: sval::data::Tag, mut value: V) -> sval::Result {
                self.erase_receiver().0.dispatch_struct_map_value(tag, &mut value)
            }

            fn struct_seq_begin(&mut self, tag: sval::data::Tag, num_entries_hint: Option<usize>) -> sval::Result {
                self.erase_receiver().0.dispatch_struct_seq_begin(tag, num_entries_hint)
            }

            fn struct_seq_value_begin(&mut self, tag: sval::data::Tag) -> sval::Result {
                self.erase_receiver().0.dispatch_struct_seq_value_begin(tag)
            }

            fn struct_seq_value_end(&mut self) -> sval::Result {
                self.erase_receiver().0.dispatch_struct_seq_value_end()
            }

            fn struct_seq_end(&mut self) -> sval::Result {
                self.erase_receiver().0.dispatch_struct_seq_end()
            }

            fn struct_seq_value<'v: 'a, V: sval::Source<'v>>(&mut self, tag: sval::data::Tag, mut value: V) -> sval::Result {
                self.erase_receiver().0.dispatch_struct_seq_value(tag, &mut value)
            }

            fn enum_begin(&mut self, tag: sval::data::Tag) -> sval::Result {
                self.erase_receiver().0.dispatch_enum_begin(tag)
            }

            fn enum_end(&mut self) -> sval::Result {
                self.erase_receiver().0.dispatch_enum_end()
            }

            fn nullable_begin(&mut self) -> sval::Result {
                self.erase_receiver().0.dispatch_nullable_begin()
            }

            fn nullable_end(&mut self) -> sval::Result {
                self.erase_receiver().0.dispatch_nullable_end()
            }

            fn fixed_size_begin(&mut self) -> sval::Result {
                self.erase_receiver().0.dispatch_fixed_size_begin()
            }

            fn fixed_size_end(&mut self) -> sval::Result {
                self.erase_receiver().0.dispatch_fixed_size_end()
            }

            fn int_begin(&mut self) -> sval::Result {
                self.erase_receiver().0.dispatch_int_begin()
            }

            fn int_end(&mut self) -> sval::Result {
                self.erase_receiver().0.dispatch_int_end()
            }

            fn binfloat_begin(&mut self) -> sval::Result {
                self.erase_receiver().0.dispatch_binfloat_begin()
            }

            fn binfloat_end(&mut self) -> sval::Result {
                self.erase_receiver().0.dispatch_binfloat_end()
            }

            fn decfloat_begin(&mut self) -> sval::Result {
                self.erase_receiver().0.dispatch_decfloat_begin()
            }

            fn decfloat_end(&mut self) -> sval::Result {
                self.erase_receiver().0.dispatch_decfloat_end()
            }

            fn app_specific_begin(&mut self, app_specific_id: u128) -> sval::Result {
                self.erase_receiver().0.dispatch_app_specific_begin(app_specific_id)
            }

            fn app_specific_end(&mut self, app_specific_id: u128) -> sval::Result {
                self.erase_receiver().0.dispatch_app_specific_end(app_specific_id)
            }
        }
    }
}

impl_receiver!(impl<'a, 'd> sval::Receiver<'a> for dyn Receiver<'a> + 'd);
impl_receiver!(impl<'a, 'd> sval::Receiver<'a> for dyn Receiver<'a> + Send + 'd);
impl_receiver!(impl<'a, 'd> sval::Receiver<'a> for dyn Receiver<'a> + Send + Sync + 'd);
