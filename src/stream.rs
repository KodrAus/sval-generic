use std::{cell, error, fmt, sync};

use crate::{erased, value};

#[doc(inline)]
pub use crate::{for_all::ForAll, Error, Result};

pub trait Stream<'a> {
    fn any<'a, V: ValueRef<'a>>(&mut self, value: V) -> Result {
        value.stream(self)
    }

    fn struct_map<'v, T: TypedRef<'static, str>, F: FnOnce(Map<&mut Self>) -> Result>(
        &mut self,
        tag_type: T,
        len: Option<usize>,
        map: F,
    ) -> Result
    where
        'v: 'a,
    {
        let mut map = map.into_iter();

        self.type_tagged_begin(tag_type)?;
        self.map_begin(len)?;
        map(StructMap(self))?;
        self.map_end()?;
        self.type_tagged_end()
    }

    fn struct_variant_map<
        'v,
        T: TypedRef<'static, str>,
        K: TypedRef<'static, str>,
        F: FnOnce(Map<&mut Self>) -> Result,
    >(
        &mut self,
        tag_type: T,
        tag_key: K,
        tag_index: Option<u64>,
        len: Option<usize>,
        map: F,
    ) -> Result
    where
        'v: 'a,
    {
        let mut map = map.into_iter();

        self.variant_tagged_begin(tag_type, tag_key, tag_index)?;
        self.map_begin(len)?;
        map(Map(self))?;
        self.map_end()?;
        self.type_tagged_end()
    }

    fn map<'k, 'v, I: IntoIterator<Item = (K, V)>, K: ValueRef<'k>, V: ValueRef<'v>>(
        &mut self,
        map: I,
    ) -> Result
    where
        'k: 'a,
        'v: 'a,
    {
        let mut map = map.into_iter();

        self.map_begin(map.size_hint().1)?;
        for (k, v) in map {
            self.map_entry(k, v)?;
        }
        self.map_end()
    }

    fn map_field<'v, F: TypedRef<'static, str>, V: ValueRef<'v>>(
        &mut self,
        field: F,
        value: V,
    ) -> Result
    where
        'v: 'a,
    {
        self.map_entry(field, value)
    }

    fn map_entry<'k, 'v, K: ValueRef<'k>, V: ValueRef<'v>>(&mut self, key: K, value: V) -> Result
    where
        'k: 'a,
        'v: 'a,
    {
        self.map_key(key)?;
        self.map_value(value)
    }

    fn map_key<'k, K: ValueRef<'k>>(&mut self, key: K) -> Result
    where
        'k: 'a,
    {
        self.map_key_begin()?;
        self.any(key)?;
        self.map_key_end()
    }

    fn map_value<'v, V: ValueRef<'v>>(&mut self, value: V) -> Result
    where
        'v: 'a,
    {
        self.map_value_begin()?;
        self.any(value)?;
        self.map_value_end()
    }

    fn seq<'e, I: IntoIterator<Item = E>, E: ValueRef<'e>>(&mut self, seq: I) -> Result {
        let seq = seq.into_iter();

        self.seq_begin(seq.size_hint().1)?;
        for elem in seq {
            self.seq_elem(elem)?;
        }
        self.seq_end()
    }

    fn seq_elem<'e, E: ValueRef<'e>>(&mut self, elem: E) -> Result
    where
        'e: 'a,
    {
        self.seq_elem_begin()?;
        self.any(elem)?;
        self.seq_elem_end()
    }

    fn type_tagged<'v, T: TypedRef<'static, str>, V: ValueRef<'v>>(
        &mut self,
        tag_type: T,
        value: V,
    ) -> Result
    where
        'v: 'a,
    {
        self.type_tagged_begin(tag_type)?;
        self.any(value)?;
        self.type_tagged_end()
    }

    fn variant_tagged<'v, T: TypedRef<'static, str>, K: TypedRef<'static, str>, V: ValueRef<'v>>(
        &mut self,
        tag_type: T,
        tag_key: K,
        tag_index: Option<u64>,
        value: V,
    ) -> Result
    where
        'v: 'a,
    {
        self.variant_tagged_begin(tag_type, tag_value, tag_index)?;
        self.any(value)?;
        self.variant_tagged_end()
    }

    fn display<D: fmt::Display>(&mut self, fmt: D) -> Result;

    fn u64(&mut self, value: u64) -> Result;
    fn i64(&mut self, value: i64) -> Result;
    fn u128(&mut self, value: u128) -> Result;
    fn i128(&mut self, value: i128) -> Result;
    fn f64(&mut self, value: f64) -> Result;
    fn bool(&mut self, value: bool) -> Result;
    fn none(&mut self) -> Result;

    fn str<'s, S: TypedRef<'s, str>>(&mut self, value: S) -> Result
    where
        's: 'a;

    fn error<'e, E: TypedRef<'e, dyn error::Error + 'static>>(&mut self, error: E) -> Result
    where
        'e: 'a;

    fn type_tag<T: TypedRef<'static, str>>(&mut self, tag_type: T) -> Result;
    fn value_tag<T: TypedRef<'static, str>, K: TypedRef<'static, str>>(
        tag_type: T,
        tag_key: V,
        tag_index: Option<u64>,
    ) -> Result;

    fn type_tagged_begin<T: TypedRef<'static, str>>(&mut self, tag_type: T) -> Result;
    fn type_tagged_end(&mut self) -> Result;

    fn variant_tagged_begin<T: TypedRef<'static, str>, K: TypedRef<'static, str>>(
        &mut self,
        tag_type: T,
        tag_key: K,
        tag_index: Option<u64>,
    ) -> Result;
    fn variant_tagged_end(&mut self) -> Result;

    fn map_begin(&mut self, len: Option<usize>) -> Result;
    fn map_end(&mut self) -> Result;

    fn map_key_begin(&mut self) -> Result;
    fn map_key_end(&mut self) -> Result;

    fn map_value_begin(&mut self) -> Result;
    fn map_value_end(&mut self) -> Result;

    fn seq_begin(&mut self, len: Option<usize>) -> Result;
    fn seq_end(&mut self) -> Result;

    fn seq_elem_begin(&mut self) -> Result;
    fn seq_elem_end(&mut self) -> Result;

    fn for_all(&mut self) -> ForAll<&mut Self> {
        ForAll(self)
    }

    fn erase<'b>(&'b mut self) -> erased::Stream<'a, 'b>
    where
        Self: Sized,
    {
        erased::Stream::new(self)
    }
}

impl<'a, 'b, S: ?Sized> Stream<'a> for &'b mut S
where
    S: Stream<'a>,
{
    fn any<'a, V: ValueRef<'a>>(&mut self, value: V) -> Result {
        (**self).any(value)
    }

    fn map<'k, 'v, I: IntoIterator<Item = (K, V)>, K: ValueRef<'k>, V: ValueRef<'v>>(
        &mut self,
        map: I,
    ) -> Result
    where
        'k: 'a,
        'v: 'a,
    {
        (**self).map(map)
    }

    fn map_field<'v, F: TypedRef<'static, str>, V: ValueRef<'v>>(
        &mut self,
        field: F,
        value: V,
    ) -> Result
    where
        'v: 'a,
    {
        (**self).map_field(field, value)
    }

    fn map_entry<'k, 'v, K: ValueRef<'k>, V: ValueRef<'v>>(&mut self, key: K, value: V) -> Result
    where
        'k: 'a,
        'v: 'a,
    {
        (**self).map_entry(key, value)
    }

    fn map_key<'k, K: ValueRef<'k>>(&mut self, key: K) -> Result
    where
        'k: 'a,
    {
        (**self).map_key(key)
    }

    fn map_value<'v, V: ValueRef<'v>>(&mut self, value: V) -> Result
    where
        'v: 'a,
    {
        (**self).map_value(value)
    }

    fn seq<'e, I: IntoIterator<Item = E>, E: ValueRef<'e>>(&mut self, seq: I) -> Result {
        (**self).seq(seq)
    }

    fn seq_elem<'e, E: ValueRef<'e>>(&mut self, elem: E) -> Result
    where
        'e: 'a,
    {
        (**self).seq_elem(elem)
    }

    fn type_tagged<'v, T: TypedRef<'static, str>, V: ValueRef<'v>>(
        &mut self,
        tag_type: T,
        value: V,
    ) -> Result
    where
        'v: 'a,
    {
        (**self).type_tagged(tag_type, value)
    }

    fn variant_tagged<'v, T: TypedRef<'static, str>, K: TypedRef<'static, str>, V: ValueRef<'v>>(
        &mut self,
        tag_type: T,
        tag_key: K,
        tag_index: Option<u64>,
        value: V,
    ) -> Result
    where
        'v: 'a,
    {
        (**self).variant_tagged(tag_type, tag_key, tag_index, value)
    }

    fn display<D: fmt::Display>(&mut self, fmt: D) -> Result {
        (**self).display(fmt)
    }

    fn u64(&mut self, value: u64) -> Result {
        (**self).u64(value)
    }

    fn i64(&mut self, value: i64) -> Result {
        (**self).i64(value)
    }

    fn u128(&mut self, value: u128) -> Result {
        (**self).u128(value)
    }

    fn i128(&mut self, value: i128) -> Result {
        (**self).i128(value)
    }

    fn f64(&mut self, value: f64) -> Result {
        (**self).f64(value)
    }

    fn bool(&mut self, value: bool) -> Result {
        (**self).bool(value)
    }

    fn none(&mut self) -> Result {
        (**self).none()
    }

    fn str<'s, S: TypedRef<'s, str>>(&mut self, value: S) -> Result
    where
        's: 'a,
    {
        (**self).str(value)
    }

    fn error<'e, E: TypedRef<'e, dyn error::Error + 'static>>(&mut self, error: E) -> Result
    where
        'e: 'a,
    {
        (**self).error(error)
    }

    fn type_tag<T: TypedRef<'static, str>>(&mut self, tag_type: T) -> Result {
        (**self).type_tag(tag_type)
    }

    fn value_tag<T: TypedRef<'static, str>, K: TypedRef<'static, str>>(
        tag_type: T,
        tag_key: V,
        tag_index: Option<u64>,
    ) -> Result {
        (**self).value_tag(tag_type, tag_key, tag_index)
    }

    fn type_tagged_begin<T: TypedRef<'static, str>>(&mut self, tag_type: T) -> Result {
        (**self).type_tagged_begin(tag_type)
    }

    fn type_tagged_end(&mut self) -> Result {
        (**self).type_tagged_end()
    }

    fn variant_tagged_begin<T: TypedRef<'static, str>, K: TypedRef<'static, str>>(
        &mut self,
        tag_type: T,
        tag_key: K,
        tag_index: Option<u64>,
    ) -> Result {
        (**self).variant_tagged_begin(tag_type, tag_key, tag_index)
    }

    fn variant_tagged_end(&mut self) -> Result {
        (**self).variant_tagged_end()
    }

    fn map_begin(&mut self, len: Option<usize>) -> Result {
        (**self).map_begin(len)
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

    fn seq_begin(&mut self, len: Option<usize>) -> Result {
        (**self).seq_begin(len)
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
}

pub struct Map<S>(S);

impl<S> Map<S>
where
    S: Stream,
{
    pub fn field<'v, F: TypedRef<'static, str>, V: ValueRef<'v>>(
        &mut self,
        field: F,
        value: V,
    ) -> Result
    where
        'v: 'a,
    {
        self.0.map_field(field, value)
    }

    pub fn entry<'k, 'v, K: ValueRef<'k>, V: ValueRef<'v>>(&mut self, key: K, value: V) -> Result
    where
        'k: 'a,
        'v: 'a,
    {
        self.0.map_entry(key, value)
    }

    pub fn key<'k, K: ValueRef<'k>>(&mut self, key: K) -> Result
    where
        'k: 'a,
    {
        self.0.map_key(key)
    }

    pub fn value<'v, V: ValueRef<'v>>(&mut self, value: V) -> Result
    where
        'v: 'a,
    {
        self.0.map_value(value)
    }
}

pub trait ValueRef<'a>: value::Value + Copy {
    fn stream<'b, S>(self, stream: S) -> Result
    where
        'a: 'b,
        S: Stream<'b>;

    fn to_str(self) -> Option<&'a str>;

    fn for_all(self) -> ForAll<Self>
    where
        Self: Sized,
    {
        ForAll(self)
    }
}

pub trait TypedRef<'a, T: ?Sized + value::Value + 'static>: ValueRef<'a> {
    fn get(&self) -> &T;
    fn try_unwrap(self) -> Option<&'a T>;
}

impl<'a, T: ?Sized> ValueRef<'a> for &'a T
where
    T: value::Value,
{
    fn stream<'b, S>(self, stream: S) -> Result
    where
        'a: 'b,
        S: Stream<'b>,
    {
        (*self).stream(stream)
    }

    fn to_str(self) -> Option<&'a str> {
        (*self).to_str()
    }
}

impl<'a, T: ?Sized> TypedRef<'a, T> for &'a T
where
    T: value::Value + 'static,
{
    fn get(&self) -> &T {
        self
    }

    fn try_unwrap(self) -> Option<&'a T> {
        Some(self)
    }
}
