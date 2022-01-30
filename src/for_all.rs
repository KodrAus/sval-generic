use crate::{
    data::{Bytes, Error, Tag, Tagged, Text},
    source::{Stream, TakeError},
    Receiver, Result, Source, Value, ValueSource,
};
use core::fmt::Display;

#[inline]
pub fn for_all<T>(value: T) -> ForAll<T> {
    ForAll::new(value)
}

#[derive(Clone, Copy)]
pub struct ForAll<T>(pub(crate) T);

impl<T> ForAll<T> {
    pub fn new(value: T) -> Self {
        ForAll(value)
    }

    pub fn by_ref(&self) -> ForAll<&T> {
        ForAll(&self.0)
    }

    pub fn by_mut(&mut self) -> ForAll<&mut T> {
        ForAll(&mut self.0)
    }

    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T: Value> Value for ForAll<T> {
    fn stream<'a, S: Receiver<'a>>(&'a self, stream: S) -> Result {
        self.0.stream(stream)
    }
}

impl<'a, 'b, T: Source<'b>> Source<'a> for ForAll<T> {
    fn stream_resume<'c, S: Receiver<'c>>(&mut self, stream: S) -> Result<Stream>
    where
        'a: 'c,
    {
        self.0.stream_resume(for_all(stream))
    }

    fn stream_to_end<'c, S: Receiver<'c>>(&mut self, stream: S) -> Result
    where
        'a: 'c,
    {
        self.0.stream_to_end(for_all(stream))
    }
}

impl<'a, 'b, U: Value + ?Sized, V: Value + ?Sized, T: ValueSource<'b, U, V>> ValueSource<'a, U, V>
    for ForAll<T>
{
    type Error = crate::Error;

    fn take(&mut self) -> Result<&U, TakeError<Self::Error>> {
        self.0.take().map_err(|e| e.map_err(|e| e.into()))
    }
}

impl<'a, 'b, R: Receiver<'b>> Receiver<'a> for ForAll<R> {
    fn unstructured<D: Display>(&mut self, fmt: D) -> Result {
        self.0.unstructured(fmt)
    }

    fn null(&mut self) -> Result {
        self.0.null()
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

    fn bool(&mut self, value: bool) -> Result {
        self.0.bool(value)
    }

    fn char(&mut self, value: char) -> Result {
        self.0.char(value)
    }

    fn str<'s: 'a, S: ValueSource<'s, str>>(&mut self, value: S) -> Result {
        self.0.str(for_all(value))
    }

    fn text<'s: 'a, S: ValueSource<'s, Text>>(&mut self, text: S) -> Result {
        self.0.text(for_all(text))
    }

    fn error<'e: 'a, E: ValueSource<'e, Error>>(&mut self, error: E) -> Result {
        self.0.error(for_all(error))
    }

    fn bytes<'s: 'a, B: ValueSource<'s, Bytes>>(&mut self, bytes: B) -> Result {
        self.0.bytes(for_all(bytes))
    }

    fn tag<T: ValueSource<'static, str>>(&mut self, tag: Tag<T>) -> Result {
        self.0.tag(tag)
    }

    fn tagged_begin<T: ValueSource<'static, str>>(&mut self, tag: Tag<T>) -> Result {
        self.0.tagged_begin(tag)
    }

    fn tagged_end<T: ValueSource<'static, str>>(&mut self, tag: Tag<T>) -> Result {
        self.0.tagged_end(tag)
    }

    fn tagged<'v: 'a, T: ValueSource<'static, str>, V: Source<'v>>(
        &mut self,
        tagged: Tagged<T, V>,
    ) -> Result {
        self.0.tagged(tagged.map_value(for_all))
    }

    fn map_begin(&mut self, size: Option<u64>) -> Result {
        self.0.map_begin(size)
    }

    fn map_end(&mut self) -> Result {
        self.0.map_end()
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

    fn map_entry<'k: 'a, 'v: 'a, K: Source<'k>, V: Source<'v>>(
        &mut self,
        key: K,
        value: V,
    ) -> Result {
        self.0.map_entry(for_all(key), for_all(value))
    }

    fn map_field_entry<'v: 'a, F: ValueSource<'static, str>, V: Source<'v>>(
        &mut self,
        field: F,
        value: V,
    ) -> Result {
        self.0.map_field_entry(field, for_all(value))
    }

    fn map_field<F: ValueSource<'static, str>>(&mut self, field: F) -> Result {
        self.0.map_field(field)
    }

    fn map_key<'k: 'a, K: Source<'k>>(&mut self, key: K) -> Result {
        self.0.map_key(for_all(key))
    }

    fn map_value<'v: 'a, V: Source<'v>>(&mut self, value: V) -> Result {
        self.0.map_value(for_all(value))
    }

    fn seq_begin(&mut self, size: Option<u64>) -> Result {
        self.0.seq_begin(size)
    }

    fn seq_end(&mut self) -> Result {
        self.0.seq_end()
    }

    fn seq_elem_begin(&mut self) -> Result {
        self.0.seq_elem_begin()
    }

    fn seq_elem_end(&mut self) -> Result {
        self.0.seq_elem_end()
    }

    fn seq_elem<'e: 'a, E: Source<'e>>(&mut self, elem: E) -> Result {
        self.0.seq_elem(for_all(elem))
    }
}
