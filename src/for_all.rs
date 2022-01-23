use core::fmt::Display;
use crate::{
    data,
    source::{Stream, TakeError},
    std::fmt,
    Receiver, Result, Source, Value, ValueSource,
};
use crate::data::{Bytes, Digits, Error, Tag};

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
        self.0.stream_resume(ForAll(stream))
    }

    fn stream_to_end<'c, S: Receiver<'c>>(&mut self, stream: S) -> Result
    where
        'a: 'c,
    {
        self.0.stream_to_end(ForAll(stream))
    }
}

impl<'a, 'b, U: Value + ?Sized, V: Value + ?Sized, T: ValueSource<'b, U, V>> ValueSource<'a, U, V>
    for ForAll<T>
{
    type Error = crate::Error;

    fn take(&mut self) -> Result<&U, TakeError<Self::Error>> {
        self.0
            .take()
            .map_err(|e| TakeError::from_error(e.into_error().into()))
    }
}

impl<'a, 'b, R: Receiver<'b>> Receiver<'a> for ForAll<R> {
    fn source<'v: 'a, S: Source<'v>>(&mut self, source: S) -> Result {
        todo!()
    }

    fn value<'v: 'a, V: Value + ?Sized + 'v>(&mut self, value: &'v V) -> Result {
        todo!()
    }

    fn unstructured<D: Display>(&mut self, fmt: D) -> Result {
        todo!()
    }

    fn null(&mut self) -> Result {
        todo!()
    }

    fn u8(&mut self, value: u8) -> Result {
        todo!()
    }

    fn u16(&mut self, value: u16) -> Result {
        todo!()
    }

    fn u32(&mut self, value: u32) -> Result {
        todo!()
    }

    fn u64(&mut self, value: u64) -> Result {
        todo!()
    }

    fn i8(&mut self, value: i8) -> Result {
        todo!()
    }

    fn i16(&mut self, value: i16) -> Result {
        todo!()
    }

    fn i32(&mut self, value: i32) -> Result {
        todo!()
    }

    fn i64(&mut self, value: i64) -> Result {
        todo!()
    }

    fn u128(&mut self, value: u128) -> Result {
        todo!()
    }

    fn i128(&mut self, value: i128) -> Result {
        todo!()
    }

    fn f32(&mut self, value: f32) -> Result {
        todo!()
    }

    fn f64(&mut self, value: f64) -> Result {
        todo!()
    }

    fn bool(&mut self, value: bool) -> Result {
        todo!()
    }

    fn char(&mut self, value: char) -> Result {
        todo!()
    }

    fn str<'s: 'a, S: ValueSource<'s, str>>(&mut self, value: S) -> Result {
        todo!()
    }

    fn digits<'d: 'a, D: ValueSource<'d, Digits>>(&mut self, value: D) -> Result {
        todo!()
    }

    fn error<'e: 'a, E: ValueSource<'e, Error>>(&mut self, error: E) -> Result {
        todo!()
    }

    fn bytes<'s: 'a, B: ValueSource<'s, Bytes>>(&mut self, bytes: B) -> Result {
        todo!()
    }

    fn tag<T: ValueSource<'static, str>>(&mut self, tag: Tag<T>) -> Result {
        todo!()
    }

    fn tag_variant<T: ValueSource<'static, str>, K: ValueSource<'static, str>>(&mut self, type_tag: Tag<T>, variant_tag: Tag<K>, variant_index: Option<u64>) -> Result {
        todo!()
    }

    fn tagged_begin<T: ValueSource<'static, str>>(&mut self, tag: Tag<T>) -> Result {
        todo!()
    }

    fn tagged_end(&mut self) -> Result {
        todo!()
    }

    fn tagged_variant_begin<T: ValueSource<'static, str>, K: ValueSource<'static, str>>(&mut self, type_tag: Tag<T>, variant_tag: Tag<K>, variant_index: Option<u64>) -> Result {
        todo!()
    }

    fn tagged_variant_end(&mut self) -> Result {
        todo!()
    }

    fn tagged<'v: 'a, T: ValueSource<'static, str>, V: Source<'v>>(&mut self, tag: Tag<T>, value: V) -> Result {
        todo!()
    }

    fn tagged_variant<'v: 'a, T: ValueSource<'static, str>, K: ValueSource<'static, str>, V: Source<'v>>(&mut self, type_tag: Tag<T>, variant_tag: Tag<K>, variant_index: Option<u64>, value: V) -> Result {
        todo!()
    }

    fn tagged_str<'s: 'a, T: ValueSource<'static, str>, S: ValueSource<'s, str>>(&mut self, tag: Tag<T>, value: S) -> Result {
        todo!()
    }

    fn tagged_bytes<'s: 'a, T: ValueSource<'static, str>, B: ValueSource<'s, Bytes>>(&mut self, tag: Tag<T>, value: B) -> Result {
        todo!()
    }

    fn map_begin(&mut self, size: Option<u64>) -> Result {
        todo!()
    }

    fn map_end(&mut self) -> Result {
        todo!()
    }

    fn tagged_map_begin<T: ValueSource<'static, str>>(&mut self, tag: Tag<T>, size: Option<u64>) -> Result {
        todo!()
    }

    fn tagged_map_end(&mut self) -> Result {
        todo!()
    }

    fn tagged_variant_map_begin<T: ValueSource<'static, str>, K: ValueSource<'static, str>>(&mut self, type_tag: Tag<T>, variant_tag: Tag<K>, variant_index: Option<u64>, size: Option<u64>) -> Result {
        todo!()
    }

    fn tagged_variant_map_end(&mut self) -> Result {
        todo!()
    }

    fn map_key_begin(&mut self) -> Result {
        todo!()
    }

    fn map_key_end(&mut self) -> Result {
        todo!()
    }

    fn map_value_begin(&mut self) -> Result {
        todo!()
    }

    fn map_value_end(&mut self) -> Result {
        todo!()
    }

    fn map_entry<'k: 'a, 'v: 'a, K: Source<'k>, V: Source<'v>>(&mut self, key: K, value: V) -> Result {
        todo!()
    }

    fn map_field_entry<'v: 'a, F: ValueSource<'static, str>, V: Source<'v>>(&mut self, field: F, value: V) -> Result {
        todo!()
    }

    fn map_field<F: ValueSource<'static, str>>(&mut self, field: F) -> Result {
        todo!()
    }

    fn map_key<'k: 'a, K: Source<'k>>(&mut self, key: K) -> Result {
        todo!()
    }

    fn map_value<'v: 'a, V: Source<'v>>(&mut self, value: V) -> Result {
        todo!()
    }

    fn seq_begin(&mut self, size: Option<u64>) -> Result {
        todo!()
    }

    fn seq_end(&mut self) -> Result {
        todo!()
    }

    fn tagged_seq_begin<T: ValueSource<'static, str>>(&mut self, tag: Tag<T>, size: Option<u64>) -> Result {
        todo!()
    }

    fn tagged_seq_end(&mut self) -> Result {
        todo!()
    }

    fn tagged_variant_seq_begin<T: ValueSource<'static, str>, K: ValueSource<'static, str>>(&mut self, type_tag: Tag<T>, variant_tag: Tag<K>, variant_index: Option<u64>, size: Option<u64>) -> Result {
        todo!()
    }

    fn tagged_variant_seq_end(&mut self) -> Result {
        todo!()
    }

    fn seq_elem_begin(&mut self) -> Result {
        todo!()
    }

    fn seq_elem_end(&mut self) -> Result {
        todo!()
    }

    fn seq_elem<'e: 'a, E: Source<'e>>(&mut self, elem: E) -> Result {
        todo!()
    }
}
