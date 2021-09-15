use std::{cell, error, fmt, sync};

use crate::{erased, value};

#[doc(inline)]
pub use crate::{for_all::ForAll, Error, Result};

pub trait Stream<'a> {
    // This high-level API describes the valid ways the low-level flat API
    // can be used: whether or not pairs must contain values

    fn any<'a, V: ValueRef<'a>>(&mut self, value: V) -> Result {
        value.stream(self)
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
        key.stream(self)?;
        self.map_key_end()
    }

    fn map_value<'v, V: ValueRef<'v>>(&mut self, value: V) -> Result
    where
        'v: 'a,
    {
        self.map_value_begin()?;
        value.stream(self)?;
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
        elem.stream(self)?;
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
        value.stream(self)?;
        self.type_tagged_end()
    }

    fn value_tagged<'v, T: TypedRef<'static, str>, K: TypedRef<'static, str>, V: ValueRef<'v>>(
        &mut self,
        tag_type: T,
        tag_key: K,
        tag_index: Option<u64>,
        value: V,
    ) -> Result
    where
        'v: 'a,
    {
        self.value_tagged_begin(tag_type, tag_value, tag_index)?;
        value.stream(self)?;
        self.value_tagged_end()
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

    // Pair *must* be contained by a single value
    fn type_tagged_begin<T: TypedRef<'static, str>>(&mut self, tag_type: T) -> Result;
    fn type_tagged_end(&mut self) -> Result;

    // Pair *must* be contained by a single value
    fn value_tagged_begin<T: TypedRef<'static, str>, K: TypedRef<'static, str>>(
        &mut self,
        tag_type: T,
        tag_key: K,
        tag_index: Option<u64>,
    ) -> Result;
    fn value_tagged_end(&mut self) -> Result;

    // Pair may be empty
    fn map_begin(&mut self, len: Option<usize>) -> Result;
    fn map_end(&mut self) -> Result;

    // Pair *must* be contained by a single value
    fn map_key_begin(&mut self) -> Result;
    fn map_key_end(&mut self) -> Result;

    // Pair *must* be contained by a single value
    fn map_value_begin(&mut self) -> Result;
    fn map_value_end(&mut self) -> Result;

    // Pair may be empty
    fn seq_begin(&mut self, len: Option<usize>) -> Result;
    fn seq_end(&mut self) -> Result;

    // Pair *must* be contained by a single value
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
    fn u64(&mut self, v: u64) -> Result {
        (**self).u64(v)
    }

    fn i64(&mut self, v: i64) -> Result {
        (**self).i64(v)
    }

    fn u128(&mut self, v: u128) -> Result {
        (**self).u128(v)
    }

    fn i128(&mut self, v: i128) -> Result {
        (**self).i128(v)
    }

    fn f64(&mut self, v: f64) -> Result {
        (**self).f64(v)
    }

    fn bool(&mut self, v: bool) -> Result {
        (**self).bool(v)
    }

    fn none(&mut self) -> Result {
        (**self).none()
    }

    fn display<V: fmt::Display>(&mut self, d: V) -> Result {
        (**self).display(d)
    }

    fn error<'v, V: TypedRef<'v, dyn error::Error + 'static>>(&mut self, e: V) -> Result
    where
        'v: 'a,
    {
        (**self).error(e)
    }

    fn str<'v, V: TypedRef<'v, str>>(&mut self, v: V) -> Result
    where
        'v: 'a,
    {
        (**self).str(v)
    }

    fn type_tagged_begin<T: TypedRef<'static, str>>(&mut self, ty: T) -> Result {
        (**self).type_tagged_begin(ty)
    }

    fn value_tagged_begin<T: TypedRef<'static, str>, I: TypedRef<'static, str>>(
        &mut self,
        ty: T,
        val: I,
        i: Option<u64>,
    ) -> Result {
        (**self).value_tagged_begin(ty, val, i)
    }

    fn type_tagged<'v, T: TypedRef<'static, str>, V: ValueRef<'v>>(&mut self, ty: T, v: V) -> Result
    where
        'v: 'a,
    {
        (**self).type_tagged(ty, v)
    }

    fn value_tagged<'v, T: TypedRef<'static, str>, I: TypedRef<'static, str>, V: ValueRef<'v>>(
        &mut self,
        ty: T,
        val: I,
        i: Option<u64>,
        v: V,
    ) -> Result
    where
        'v: 'a,
    {
        (**self).value_tagged(ty, val, i, v)
    }

    fn map_begin(&mut self, len: Option<usize>) -> Result {
        (**self).map_begin(len)
    }

    fn map_key_begin(&mut self) -> Result {
        (**self).map_key_begin()
    }

    fn map_value_begin(&mut self) -> Result {
        (**self).map_value_begin()
    }

    fn map_end(&mut self) -> Result {
        (**self).map_end()
    }

    fn map_key<'k, K: ValueRef<'k>>(&mut self, k: K) -> Result
    where
        'k: 'a,
    {
        (**self).map_key(k)
    }

    fn map_value<'v, V: ValueRef<'v>>(&mut self, v: V) -> Result
    where
        'v: 'a,
    {
        (**self).map_value(v)
    }

    fn map_entry<'k, 'v, K: ValueRef<'k>, V: ValueRef<'v>>(&mut self, k: K, v: V) -> Result
    where
        'k: 'a,
        'v: 'a,
    {
        (**self).map_entry(k, v)
    }

    fn map_field<'v, F: TypedRef<'static, str>, V: ValueRef<'v>>(&mut self, f: F, v: V) -> Result
    where
        'v: 'a,
    {
        (**self).map_field(f, v)
    }

    fn type_tagged_map_begin<T: TypedRef<'static, str>>(
        &mut self,
        ty: T,
        len: Option<usize>,
    ) -> Result {
        (**self).type_tagged_map_begin(ty, len)
    }

    fn type_tagged_map_end(&mut self) -> Result {
        (**self).type_tagged_map_end()
    }

    fn value_tagged_map_begin<T: TypedRef<'static, str>, I: TypedRef<'static, str>>(
        &mut self,
        ty: T,
        val: I,
        i: Option<u64>,
        len: Option<usize>,
    ) -> Result {
        (**self).value_tagged_map_begin(ty, val, i, len)
    }

    fn value_tagged_map_end(&mut self) -> Result {
        (**self).value_tagged_map_end()
    }

    fn seq_begin(&mut self, len: Option<usize>) -> Result {
        (**self).seq_begin(len)
    }

    fn seq_elem_begin(&mut self) -> Result {
        (**self).seq_elem_begin()
    }

    fn seq_end(&mut self) -> Result {
        (**self).seq_end()
    }

    fn seq_elem<'e, E: ValueRef<'e>>(&mut self, e: E) -> Result
    where
        'e: 'a,
    {
        (**self).seq_elem(e)
    }

    fn type_tagged_seq_begin<T: TypedRef<'static, str>>(
        &mut self,
        ty: T,
        len: Option<usize>,
    ) -> Result {
        (**self).type_tagged_seq_begin(ty, len)
    }

    fn type_tagged_seq_end(&mut self) -> Result {
        (**self).type_tagged_seq_end()
    }

    fn value_tagged_seq_begin<T: TypedRef<'static, str>, I: TypedRef<'static, str>>(
        &mut self,
        ty: T,
        val: I,
        i: Option<u64>,
        len: Option<usize>,
    ) -> Result {
        (**self).value_tagged_seq_begin(ty, val, i, len)
    }

    fn value_tagged_seq_end(&mut self) -> Result {
        (**self).value_tagged_seq_end()
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
