use std::{error, fmt};

use crate::erased;

#[doc(inline)]
pub use crate::{
    for_all::ForAll,
    reference::{TypedRef, ValueRef},
    tag::{TypeTag, VariantTag},
    Error, Result,
};

pub trait Stream<'a> {
    fn any<'v, V: ValueRef<'v>>(&mut self, value: V) -> Result
    where
        'v: 'a,
    {
        value.stream(self)
    }

    fn display<D: fmt::Display>(&mut self, fmt: D) -> Result {
        let _ = fmt;
        Err(Error)
    }

    fn u64(&mut self, value: u64) -> Result {
        let _ = value;
        Err(Error)
    }

    fn i64(&mut self, value: i64) -> Result {
        let _ = value;
        Err(Error)
    }

    fn u128(&mut self, value: u128) -> Result {
        let _ = value;
        Err(Error)
    }

    fn i128(&mut self, value: i128) -> Result {
        let _ = value;
        Err(Error)
    }

    fn f64(&mut self, value: f64) -> Result {
        let _ = value;
        Err(Error)
    }

    fn bool(&mut self, value: bool) -> Result {
        let _ = value;
        Err(Error)
    }

    fn none(&mut self) -> Result {
        Err(Error)
    }

    fn str<'s, S: TypedRef<'s, str>>(&mut self, value: S) -> Result
    where
        's: 'a,
    {
        let _ = value;
        Err(Error)
    }

    fn error<'e, E: TypedRef<'e, dyn error::Error + 'static>>(&mut self, error: E) -> Result
    where
        'e: 'a,
    {
        let _ = error;
        Err(Error)
    }

    fn type_tag<T: TypedRef<'static, str>>(&mut self, tag: TypeTag<T>) -> Result {
        self.str(tag.ty())
    }

    fn variant_tag<T: TypedRef<'static, str>, K: TypedRef<'static, str>>(
        &mut self,
        tag: VariantTag<T, K>,
    ) -> Result {
        self.str(tag.variant_key())
    }

    fn type_tagged_begin<T: TypedRef<'static, str>>(&mut self, tag: TypeTag<T>) -> Result {
        let _ = tag;
        Ok(())
    }

    fn type_tagged_end(&mut self) -> Result {
        Ok(())
    }

    fn variant_tagged_begin<T: TypedRef<'static, str>, K: TypedRef<'static, str>>(
        &mut self,
        tag: VariantTag<T, K>,
    ) -> Result {
        self.map_begin(Some(1))?;
        self.map_key(tag)?;
        self.map_value_begin()
    }

    fn variant_tagged_end(&mut self) -> Result {
        self.map_value_end()?;
        self.map_end()
    }

    fn type_tagged<'v, T: TypedRef<'static, str>, V: ValueRef<'v>>(
        &mut self,
        tag: TypeTag<T>,
        value: V,
    ) -> Result
    where
        'v: 'a,
    {
        self.type_tagged_begin(tag)?;
        self.any(value)?;
        self.type_tagged_end()
    }

    fn variant_tagged<'v, T: TypedRef<'static, str>, K: TypedRef<'static, str>, V: ValueRef<'v>>(
        &mut self,
        tag: VariantTag<T, K>,
        value: V,
    ) -> Result
    where
        'v: 'a,
    {
        self.variant_tagged_begin(tag)?;
        self.any(value)?;
        self.variant_tagged_end()
    }

    fn map_begin(&mut self, len: Option<usize>) -> Result {
        let _ = len;
        Err(Error)
    }

    fn map_end(&mut self) -> Result {
        Err(Error)
    }

    fn map_key_begin(&mut self) -> Result {
        Err(Error)
    }

    fn map_key_end(&mut self) -> Result {
        Err(Error)
    }

    fn map_value_begin(&mut self) -> Result {
        Err(Error)
    }

    fn map_value_end(&mut self) -> Result {
        Err(Error)
    }

    fn type_tagged_map_begin<T: TypedRef<'static, str>>(
        &mut self,
        tag: TypeTag<T>,
        len: Option<usize>,
    ) -> Result {
        self.type_tagged_begin(tag)?;
        self.map_begin(len)
    }

    fn type_tagged_map_end(&mut self) -> Result {
        self.map_end()?;
        self.type_tagged_end()
    }

    fn variant_tagged_map_begin<T: TypedRef<'static, str>, K: TypedRef<'static, str>>(
        &mut self,
        tag: VariantTag<T, K>,
        len: Option<usize>,
    ) -> Result {
        self.variant_tagged_begin(tag)?;
        self.map_begin(len)
    }

    fn variant_tagged_map_end(&mut self) -> Result {
        self.map_end()?;
        self.variant_tagged_end()
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

    fn map_field<F: TypedRef<'static, str>>(&mut self, field: F) -> Result {
        self.map_key(field)
    }

    fn map_value<'v, V: ValueRef<'v>>(&mut self, value: V) -> Result
    where
        'v: 'a,
    {
        self.map_value_begin()?;
        self.any(value)?;
        self.map_value_end()
    }

    fn seq_begin(&mut self, len: Option<usize>) -> Result {
        let _ = len;
        Err(Error)
    }

    fn seq_end(&mut self) -> Result {
        Err(Error)
    }

    fn type_tagged_seq_begin<T: TypedRef<'static, str>>(
        &mut self,
        tag: TypeTag<T>,
        len: Option<usize>,
    ) -> Result {
        self.type_tagged_begin(tag)?;
        self.seq_begin(len)
    }

    fn type_tagged_seq_end(&mut self) -> Result {
        self.seq_end()?;
        self.type_tagged_end()
    }

    fn variant_tagged_seq_begin<T: TypedRef<'static, str>, K: TypedRef<'static, str>>(
        &mut self,
        tag: VariantTag<T, K>,
        len: Option<usize>,
    ) -> Result {
        self.variant_tagged_begin(tag)?;
        self.seq_begin(len)
    }

    fn variant_tagged_seq_end(&mut self) -> Result {
        self.seq_end()?;
        self.variant_tagged_end()
    }

    fn seq_elem_begin(&mut self) -> Result {
        Err(Error)
    }

    fn seq_elem_end(&mut self) -> Result {
        Err(Error)
    }

    fn seq_elem<'e, E: ValueRef<'e>>(&mut self, elem: E) -> Result
    where
        'e: 'a,
    {
        self.seq_elem_begin()?;
        self.any(elem)?;
        self.seq_elem_end()
    }

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
    fn any<'v, V: ValueRef<'v>>(&mut self, value: V) -> Result
    where
        'v: 'a,
    {
        (**self).any(value)
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

    fn str<'s, T: TypedRef<'s, str>>(&mut self, value: T) -> Result
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

    fn type_tag<T: TypedRef<'static, str>>(&mut self, tag: TypeTag<T>) -> Result {
        (**self).type_tag(tag)
    }

    fn variant_tag<T: TypedRef<'static, str>, K: TypedRef<'static, str>>(
        &mut self,
        tag: VariantTag<T, K>,
    ) -> Result {
        (**self).variant_tag(tag)
    }

    fn type_tagged_begin<T: TypedRef<'static, str>>(&mut self, tag: TypeTag<T>) -> Result {
        (**self).type_tagged_begin(tag)
    }

    fn type_tagged_end(&mut self) -> Result {
        (**self).type_tagged_end()
    }

    fn variant_tagged_begin<T: TypedRef<'static, str>, K: TypedRef<'static, str>>(
        &mut self,
        tag: VariantTag<T, K>,
    ) -> Result {
        (**self).variant_tagged_begin(tag)
    }

    fn variant_tagged_end(&mut self) -> Result {
        (**self).variant_tagged_end()
    }

    fn type_tagged<'v, T: TypedRef<'static, str>, V: ValueRef<'v>>(
        &mut self,
        tag: TypeTag<T>,
        value: V,
    ) -> Result
    where
        'v: 'a,
    {
        (**self).type_tagged(tag, value)
    }

    fn variant_tagged<'v, T: TypedRef<'static, str>, K: TypedRef<'static, str>, V: ValueRef<'v>>(
        &mut self,
        tag: VariantTag<T, K>,
        value: V,
    ) -> Result
    where
        'v: 'a,
    {
        (**self).variant_tagged(tag, value)
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

    fn type_tagged_map_begin<T: TypedRef<'static, str>>(
        &mut self,
        tag: TypeTag<T>,
        len: Option<usize>,
    ) -> Result {
        (**self).type_tagged_map_begin(tag, len)
    }

    fn type_tagged_map_end(&mut self) -> Result {
        (**self).type_tagged_map_end()
    }

    fn variant_tagged_map_begin<T: TypedRef<'static, str>, K: TypedRef<'static, str>>(
        &mut self,
        tag: VariantTag<T, K>,
        len: Option<usize>,
    ) -> Result {
        (**self).variant_tagged_map_begin(tag, len)
    }

    fn variant_tagged_map_end(&mut self) -> Result {
        (**self).variant_tagged_map_end()
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

    fn map_field<F: TypedRef<'static, str>>(&mut self, field: F) -> Result {
        (**self).map_field(field)
    }

    fn map_value<'v, V: ValueRef<'v>>(&mut self, value: V) -> Result
    where
        'v: 'a,
    {
        (**self).map_value(value)
    }

    fn seq_begin(&mut self, len: Option<usize>) -> Result {
        (**self).seq_begin(len)
    }

    fn seq_end(&mut self) -> Result {
        (**self).seq_end()
    }

    fn type_tagged_seq_begin<T: TypedRef<'static, str>>(
        &mut self,
        tag: TypeTag<T>,
        len: Option<usize>,
    ) -> Result {
        (**self).type_tagged_seq_begin(tag, len)
    }

    fn type_tagged_seq_end(&mut self) -> Result {
        (**self).type_tagged_seq_end()
    }

    fn variant_tagged_seq_begin<T: TypedRef<'static, str>, K: TypedRef<'static, str>>(
        &mut self,
        tag: VariantTag<T, K>,
        len: Option<usize>,
    ) -> Result {
        (**self).variant_tagged_seq_begin(tag, len)
    }

    fn variant_tagged_seq_end(&mut self) -> Result {
        (**self).variant_tagged_seq_end()
    }

    fn seq_elem_begin(&mut self) -> Result {
        (**self).seq_elem_begin()
    }

    fn seq_elem_end(&mut self) -> Result {
        (**self).seq_elem_end()
    }

    fn seq_elem<'e, E: ValueRef<'e>>(&mut self, elem: E) -> Result
    where
        'e: 'a,
    {
        (**self).seq_elem(elem)
    }
}
