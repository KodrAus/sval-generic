use crate::writer::{GenericWriter, Writer};
use core::fmt::Write;

pub fn to_fmt(fmt: impl Write, v: impl sval::Value) -> sval::Result {
    v.stream(&mut Formatter::new(fmt))
}

pub struct Formatter<W>(Writer<GenericWriter<W>>);

impl<W> Formatter<W> {
    pub fn new(out: W) -> Self {
        Formatter(Writer::new(GenericWriter(out)))
    }

    pub fn into_inner(self) -> W {
        self.0.into_inner().0
    }
}

impl<'sval, W: Write> sval::Stream<'sval> for Formatter<W> {
    fn is_text_based(&self) -> bool {
        self.0.is_text_based()
    }

    fn unit(&mut self) -> sval::Result {
        self.0.unit()
    }

    fn null(&mut self) -> sval::Result {
        self.0.null()
    }

    fn bool(&mut self, value: bool) -> sval::Result {
        self.0.bool(value)
    }

    fn text_begin(&mut self, num_bytes_hint: Option<usize>) -> sval::Result {
        self.0.text_begin(num_bytes_hint)
    }

    fn text_fragment(&mut self, fragment: &'sval str) -> sval::Result {
        self.0.text_fragment(fragment)
    }

    fn text_fragment_computed(&mut self, fragment: &str) -> sval::Result {
        self.0.text_fragment_computed(fragment)
    }

    fn text_end(&mut self) -> sval::Result {
        self.0.text_end()
    }

    fn binary_begin(&mut self, num_bytes_hint: Option<usize>) -> sval::Result {
        self.0.binary_begin(num_bytes_hint)
    }

    fn binary_fragment(&mut self, fragment: &'sval [u8]) -> sval::Result {
        self.0.binary_fragment(fragment)
    }

    fn binary_fragment_computed(&mut self, fragment: &[u8]) -> sval::Result {
        self.0.binary_fragment_computed(fragment)
    }

    fn binary_end(&mut self) -> sval::Result {
        self.0.binary_end()
    }

    fn u8(&mut self, value: u8) -> sval::Result {
        self.0.u8(value)
    }

    fn u16(&mut self, value: u16) -> sval::Result {
        self.0.u16(value)
    }

    fn u32(&mut self, value: u32) -> sval::Result {
        self.0.u32(value)
    }

    fn u64(&mut self, value: u64) -> sval::Result {
        self.0.u64(value)
    }

    fn u128(&mut self, value: u128) -> sval::Result {
        self.0.u128(value)
    }

    fn i8(&mut self, value: i8) -> sval::Result {
        self.0.i8(value)
    }

    fn i16(&mut self, value: i16) -> sval::Result {
        self.0.i16(value)
    }

    fn i32(&mut self, value: i32) -> sval::Result {
        self.0.i32(value)
    }

    fn i64(&mut self, value: i64) -> sval::Result {
        self.0.i64(value)
    }

    fn i128(&mut self, value: i128) -> sval::Result {
        self.0.i128(value)
    }

    fn f32(&mut self, value: f32) -> sval::Result {
        self.0.f32(value)
    }

    fn f64(&mut self, value: f64) -> sval::Result {
        self.0.f64(value)
    }

    fn map_begin(&mut self, num_entries_hint: Option<usize>) -> sval::Result {
        self.0.map_begin(num_entries_hint)
    }

    fn map_key_begin(&mut self) -> sval::Result {
        self.0.map_key_begin()
    }

    fn map_key_end(&mut self) -> sval::Result {
        self.0.map_key_end()
    }

    fn map_value_begin(&mut self) -> sval::Result {
        self.0.map_value_begin()
    }

    fn map_value_end(&mut self) -> sval::Result {
        self.0.map_value_end()
    }

    fn map_end(&mut self) -> sval::Result {
        self.0.map_end()
    }

    fn seq_begin(&mut self, num_entries_hint: Option<usize>) -> sval::Result {
        self.0.seq_begin(num_entries_hint)
    }

    fn seq_value_begin(&mut self) -> sval::Result {
        self.0.seq_value_begin()
    }

    fn seq_value_end(&mut self) -> sval::Result {
        self.0.seq_value_end()
    }

    fn seq_end(&mut self) -> sval::Result {
        self.0.seq_end()
    }

    fn dynamic_begin(&mut self) -> sval::Result {
        self.0.dynamic_begin()
    }

    fn dynamic_end(&mut self) -> sval::Result {
        self.0.dynamic_end()
    }

    fn enum_begin(&mut self, tag: sval::Tag) -> sval::Result {
        self.0.enum_begin(tag)
    }

    fn enum_end(&mut self, tag: sval::Tag) -> sval::Result {
        self.0.enum_end(tag)
    }

    fn tagged_begin(&mut self, tag: sval::Tag) -> sval::Result {
        self.0.tagged_begin(tag)
    }

    fn tagged_end(&mut self, tag: sval::Tag) -> sval::Result {
        self.0.tagged_end(tag)
    }

    fn record_begin(&mut self, tag: sval::Tag, num_entries_hint: Option<usize>) -> sval::Result {
        self.0.record_begin(tag, num_entries_hint)
    }

    fn record_value_begin(&mut self, label: sval::Label) -> sval::Result {
        self.0.record_value_begin(label)
    }

    fn record_value_end(&mut self, label: sval::Label) -> sval::Result {
        self.0.record_value_end(label)
    }

    fn record_end(&mut self, tag: sval::Tag) -> sval::Result {
        self.0.record_end(tag)
    }

    fn tuple_begin(&mut self, tag: sval::Tag, num_entries_hint: Option<usize>) -> sval::Result {
        self.0.tuple_begin(tag, num_entries_hint)
    }

    fn tuple_value_begin(&mut self, index: u32) -> sval::Result {
        self.0.tuple_value_begin(index)
    }

    fn tuple_value_end(&mut self, index: u32) -> sval::Result {
        self.0.tuple_value_end(index)
    }

    fn tuple_end(&mut self, tag: sval::Tag) -> sval::Result {
        self.0.tuple_end(tag)
    }

    fn constant_begin(&mut self, tag: sval::Tag) -> sval::Result {
        self.0.constant_begin(tag)
    }

    fn constant_end(&mut self, tag: sval::Tag) -> sval::Result {
        self.0.constant_end(tag)
    }

    fn constant_size_begin(&mut self) -> sval::Result {
        self.0.constant_size_begin()
    }

    fn constant_size_end(&mut self) -> sval::Result {
        self.0.constant_size_end()
    }

    fn optional_some_begin(&mut self) -> sval::Result {
        self.0.optional_some_begin()
    }

    fn optional_some_end(&mut self) -> sval::Result {
        self.0.optional_some_end()
    }

    fn optional_none(&mut self) -> sval::Result {
        self.0.optional_none()
    }

    fn int_begin(&mut self) -> sval::Result {
        self.0.int_begin()
    }

    fn int_end(&mut self) -> sval::Result {
        self.0.int_end()
    }

    fn binfloat_begin(&mut self) -> sval::Result {
        self.0.binfloat_begin()
    }

    fn binfloat_end(&mut self) -> sval::Result {
        self.0.binfloat_end()
    }

    fn decfloat_begin(&mut self) -> sval::Result {
        self.0.decfloat_begin()
    }

    fn decfloat_end(&mut self) -> sval::Result {
        self.0.decfloat_end()
    }
}
