use crate::{data, source, Receiver, Result, Source, Value};

#[inline]
pub fn computed<T>(value: T) -> Computed<T> {
    Computed::new(value)
}

#[derive(Clone, Copy)]
pub struct Computed<T>(T);

impl<T> Computed<T> {
    pub fn new(value: T) -> Self {
        Computed(value)
    }

    pub fn by_ref(&self) -> Computed<&T> {
        Computed(&self.0)
    }

    pub fn by_mut(&mut self) -> Computed<&mut T> {
        Computed(&mut self.0)
    }

    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T: Value> Value for Computed<T> {
    fn stream<'a, S: Receiver<'a>>(&'a self, receiver: S) -> Result {
        self.0.stream(receiver)
    }
}

impl<'a, 'b, T: Source<'b>> Source<'a> for Computed<T> {
    fn stream_resume<'c, S: Receiver<'c>>(&mut self, receiver: S) -> Result<source::Resume>
    where
        'a: 'c,
    {
        self.0.stream_resume(computed(receiver))
    }

    fn stream_to_end<'c, S: Receiver<'c>>(&mut self, stream: S) -> Result
    where
        'a: 'c,
    {
        self.0.stream_to_end(computed(stream))
    }
}

impl<'a, 'b, R: Receiver<'b>> Receiver<'a> for Computed<R> {
    fn is_text_based(&self) -> bool {
        self.0.is_text_based()
    }

    fn dynamic_begin(&mut self) -> Result {
        self.0.dynamic_begin()
    }

    fn dynamic_end(&mut self) -> Result {
        self.0.dynamic_end()
    }

    fn unit(&mut self) -> Result {
        self.0.unit()
    }

    fn null(&mut self) -> Result {
        self.0.null()
    }

    fn bool(&mut self, value: bool) -> Result {
        self.0.bool(value)
    }

    fn char(&mut self, value: char) -> Result {
        self.0.char(value)
    }

    fn str(&mut self, value: &'a str) -> Result {
        self.0.text_begin(Some(value.len()))?;
        self.0.text_fragment_computed(value)?;
        self.0.text_end()
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

    fn bytes(&mut self, value: &'a [u8]) -> Result {
        self.0.binary_begin(Some(value.len()))?;
        self.0.binary_fragment_computed(value)?;
        self.0.binary_end()
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

    fn u8(&mut self, value: u8) -> Result {
        self.0.u8(value)
    }

    fn u16(&mut self, value: u16) -> Result {
        self.0.u16(value)
    }

    fn u32(&mut self, value: u32) -> Result {
        self.0.u32(value)
    }

    fn u64(&mut self, value: u64) -> Result {
        self.0.u64(value)
    }

    fn u128(&mut self, value: u128) -> Result {
        self.0.u128(value)
    }

    fn i8(&mut self, value: i8) -> Result {
        self.0.i8(value)
    }

    fn i16(&mut self, value: i16) -> Result {
        self.0.i16(value)
    }

    fn i32(&mut self, value: i32) -> Result {
        self.0.i32(value)
    }

    fn i64(&mut self, value: i64) -> Result {
        self.0.i64(value)
    }

    fn i128(&mut self, value: i128) -> Result {
        self.0.i128(value)
    }

    fn f32(&mut self, value: f32) -> Result {
        self.0.f32(value)
    }

    fn f64(&mut self, value: f64) -> Result {
        self.0.f64(value)
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

    fn map_key<'k: 'a, K: Source<'k>>(&mut self, key: K) -> Result {
        self.0.map_key(computed(key))
    }

    fn map_value<'v: 'a, V: Source<'v>>(&mut self, value: V) -> Result {
        self.0.map_value(computed(value))
    }

    fn seq_begin(&mut self, num_elems_hint: Option<usize>) -> Result {
        self.0.seq_begin(num_elems_hint)
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

    fn seq_value<'e: 'a, V: Source<'e>>(&mut self, value: V) -> Result {
        self.0.seq_value(computed(value))
    }

    fn tagged_begin(&mut self, tag: data::Tag) -> Result {
        self.0.tagged_begin(tag)
    }

    fn tagged_end(&mut self) -> Result {
        self.0.tagged_end()
    }

    fn constant_begin(&mut self, tag: data::Tag) -> Result {
        self.0.constant_begin(tag)
    }

    fn constant_end(&mut self) -> Result {
        self.0.constant_end()
    }

    fn struct_begin(&mut self, tag: data::Tag) -> Result {
        self.0.struct_begin(tag)
    }

    fn struct_key_begin(&mut self, tag: data::Tag) -> Result {
        self.0.struct_key_begin(tag)
    }

    fn struct_key_end(&mut self) -> Result {
        self.0.struct_key_end()
    }

    fn struct_value_begin(&mut self, tag: data::Tag) -> Result {
        self.0.struct_value_begin(tag)
    }

    fn struct_value_end(&mut self) -> Result {
        self.0.struct_value_end()
    }

    fn struct_end(&mut self) -> Result {
        self.0.struct_end()
    }

    fn enum_begin(&mut self, tag: data::Tag) -> Result {
        self.0.enum_begin(tag)
    }

    fn enum_end(&mut self) -> Result {
        self.0.enum_end()
    }

    fn nullable_begin(&mut self, tag: data::Tag) -> Result {
        self.0.nullable_begin(tag)
    }

    fn nullable_end(&mut self) -> Result {
        self.0.nullable_end()
    }

    fn fixed_size_begin(&mut self) -> Result {
        self.0.fixed_size_begin()
    }

    fn fixed_size_end(&mut self) -> Result {
        self.0.fixed_size_end()
    }

    fn int_begin(&mut self) -> Result {
        self.0.int_begin()
    }

    fn int_end(&mut self) -> Result {
        self.0.int_end()
    }

    fn number_begin(&mut self) -> Result {
        self.0.number_begin()
    }

    fn number_end(&mut self) -> Result {
        self.0.number_end()
    }

    fn app_specific_begin(&mut self, app_specific_id: u128) -> Result {
        self.0.app_specific_begin(app_specific_id)
    }

    fn app_specific_end(&mut self, app_specific_id: u128) -> Result {
        self.0.app_specific_end(app_specific_id)
    }
}
