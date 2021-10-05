use std::{error, fmt};

use crate::{
    erased,
    for_all::ForAll,
    source::{Source, TypedSource},
    tag::{TypeTag, VariantTag},
    Error, Result,
};

#[doc(inline)]
pub use fmt::Display;

pub trait Stream<'a> {
    fn any<'v: 'a, V: Source<'v>>(&mut self, mut value: V) -> Result {
        value.stream(self)
    }

    fn display<D: Display>(&mut self, fmt: D) -> Result;

    fn u64(&mut self, value: u64) -> Result {
        self.display(value)
    }

    fn i64(&mut self, value: i64) -> Result {
        self.display(value)
    }

    fn u128(&mut self, value: u128) -> Result {
        self.display(value)
    }

    fn i128(&mut self, value: i128) -> Result {
        self.display(value)
    }

    fn f64(&mut self, value: f64) -> Result {
        self.display(value)
    }

    fn bool(&mut self, value: bool) -> Result {
        self.display(value)
    }

    fn none(&mut self) -> Result;

    fn str<'s: 'a, S: TypedSource<'s, str>>(&mut self, mut value: S) -> Result {
        self.display(value.stream_to_value()?)
    }

    fn error<'e: 'a, E: TypedSource<'e, dyn error::Error + 'static>>(
        &mut self,
        mut error: E,
    ) -> Result {
        self.display(error.stream_to_value()?)
    }

    fn type_tag<T: TypedSource<'static, str>>(&mut self, tag: TypeTag<T>) -> Result {
        self.str(tag.ty)
    }

    fn variant_tag<T: TypedSource<'static, str>, K: TypedSource<'static, str>>(
        &mut self,
        tag: VariantTag<T, K>,
    ) -> Result {
        self.str(tag.variant_key)
    }

    fn type_tagged_begin<T: TypedSource<'static, str>>(&mut self, tag: TypeTag<T>) -> Result {
        let _ = tag;
        Ok(())
    }

    fn type_tagged_end(&mut self) -> Result {
        Ok(())
    }

    fn variant_tagged_begin<T: TypedSource<'static, str>, K: TypedSource<'static, str>>(
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

    fn type_tagged<'v: 'a, T: TypedSource<'static, str>, V: Source<'v>>(
        &mut self,
        tag: TypeTag<T>,
        value: V,
    ) -> Result {
        self.type_tagged_begin(tag)?;
        self.any(value)?;
        self.type_tagged_end()
    }

    fn variant_tagged<
        'v: 'a,
        T: TypedSource<'static, str>,
        K: TypedSource<'static, str>,
        V: Source<'v>,
    >(
        &mut self,
        tag: VariantTag<T, K>,
        value: V,
    ) -> Result {
        self.variant_tagged_begin(tag)?;
        self.any(value)?;
        self.variant_tagged_end()
    }

    fn map_begin(&mut self, len: Option<usize>) -> Result;

    fn map_end(&mut self) -> Result;

    fn map_key_begin(&mut self) -> Result;

    fn map_key_end(&mut self) -> Result;

    fn map_value_begin(&mut self) -> Result;

    fn map_value_end(&mut self) -> Result;

    fn type_tagged_map_begin<T: TypedSource<'static, str>>(
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

    fn variant_tagged_map_begin<T: TypedSource<'static, str>, K: TypedSource<'static, str>>(
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

    fn map_entry<'k: 'a, 'v: 'a, K: Source<'k>, V: Source<'v>>(
        &mut self,
        key: K,
        value: V,
    ) -> Result {
        self.map_key(key)?;
        self.map_value(value)
    }

    fn map_key<'k: 'a, K: Source<'k>>(&mut self, key: K) -> Result {
        self.map_key_begin()?;
        self.any(key)?;
        self.map_key_end()
    }

    fn map_field<F: TypedSource<'static, str>>(&mut self, field: F) -> Result {
        self.map_key(field)
    }

    fn map_value<'v: 'a, V: Source<'v>>(&mut self, value: V) -> Result {
        self.map_value_begin()?;
        self.any(value)?;
        self.map_value_end()
    }

    fn seq_begin(&mut self, len: Option<usize>) -> Result;

    fn seq_end(&mut self) -> Result;

    fn type_tagged_seq_begin<T: TypedSource<'static, str>>(
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

    fn variant_tagged_seq_begin<T: TypedSource<'static, str>, K: TypedSource<'static, str>>(
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

    fn seq_elem_begin(&mut self) -> Result;

    fn seq_elem_end(&mut self) -> Result;

    fn seq_elem<'e: 'a, E: Source<'e>>(&mut self, elem: E) -> Result {
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
    fn any<'v: 'a, V: Source<'v>>(&mut self, value: V) -> Result {
        (**self).any(value)
    }

    fn display<D: Display>(&mut self, fmt: D) -> Result {
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

    fn str<'s: 'a, T: TypedSource<'s, str>>(&mut self, value: T) -> Result {
        (**self).str(value)
    }

    fn error<'e: 'a, E: TypedSource<'e, dyn error::Error + 'static>>(
        &mut self,
        error: E,
    ) -> Result {
        (**self).error(error)
    }

    fn type_tag<T: TypedSource<'static, str>>(&mut self, tag: TypeTag<T>) -> Result {
        (**self).type_tag(tag)
    }

    fn variant_tag<T: TypedSource<'static, str>, K: TypedSource<'static, str>>(
        &mut self,
        tag: VariantTag<T, K>,
    ) -> Result {
        (**self).variant_tag(tag)
    }

    fn type_tagged_begin<T: TypedSource<'static, str>>(&mut self, tag: TypeTag<T>) -> Result {
        (**self).type_tagged_begin(tag)
    }

    fn type_tagged_end(&mut self) -> Result {
        (**self).type_tagged_end()
    }

    fn variant_tagged_begin<T: TypedSource<'static, str>, K: TypedSource<'static, str>>(
        &mut self,
        tag: VariantTag<T, K>,
    ) -> Result {
        (**self).variant_tagged_begin(tag)
    }

    fn variant_tagged_end(&mut self) -> Result {
        (**self).variant_tagged_end()
    }

    fn type_tagged<'v: 'a, T: TypedSource<'static, str>, V: Source<'v>>(
        &mut self,
        tag: TypeTag<T>,
        value: V,
    ) -> Result {
        (**self).type_tagged(tag, value)
    }

    fn variant_tagged<
        'v: 'a,
        T: TypedSource<'static, str>,
        K: TypedSource<'static, str>,
        V: Source<'v>,
    >(
        &mut self,
        tag: VariantTag<T, K>,
        value: V,
    ) -> Result {
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

    fn type_tagged_map_begin<T: TypedSource<'static, str>>(
        &mut self,
        tag: TypeTag<T>,
        len: Option<usize>,
    ) -> Result {
        (**self).type_tagged_map_begin(tag, len)
    }

    fn type_tagged_map_end(&mut self) -> Result {
        (**self).type_tagged_map_end()
    }

    fn variant_tagged_map_begin<T: TypedSource<'static, str>, K: TypedSource<'static, str>>(
        &mut self,
        tag: VariantTag<T, K>,
        len: Option<usize>,
    ) -> Result {
        (**self).variant_tagged_map_begin(tag, len)
    }

    fn variant_tagged_map_end(&mut self) -> Result {
        (**self).variant_tagged_map_end()
    }

    fn map_entry<'k: 'a, 'v: 'a, K: Source<'k>, V: Source<'v>>(
        &mut self,
        key: K,
        value: V,
    ) -> Result {
        (**self).map_entry(key, value)
    }

    fn map_key<'k: 'a, K: Source<'k>>(&mut self, key: K) -> Result {
        (**self).map_key(key)
    }

    fn map_field<F: TypedSource<'static, str>>(&mut self, field: F) -> Result {
        (**self).map_field(field)
    }

    fn map_value<'v: 'a, V: Source<'v>>(&mut self, value: V) -> Result {
        (**self).map_value(value)
    }

    fn seq_begin(&mut self, len: Option<usize>) -> Result {
        (**self).seq_begin(len)
    }

    fn seq_end(&mut self) -> Result {
        (**self).seq_end()
    }

    fn type_tagged_seq_begin<T: TypedSource<'static, str>>(
        &mut self,
        tag: TypeTag<T>,
        len: Option<usize>,
    ) -> Result {
        (**self).type_tagged_seq_begin(tag, len)
    }

    fn type_tagged_seq_end(&mut self) -> Result {
        (**self).type_tagged_seq_end()
    }

    fn variant_tagged_seq_begin<T: TypedSource<'static, str>, K: TypedSource<'static, str>>(
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

    fn seq_elem<'e: 'a, E: Source<'e>>(&mut self, elem: E) -> Result {
        (**self).seq_elem(elem)
    }
}

pub fn unsupported() -> Result {
    Err(Error)
}

/*
#[async_trait]
pub trait AsyncStream<'a> {
    async fn blocking<'v: 'a, V: Source<'v>>(self, value: V) -> Result;

    async fn str<'s: 'a, S: AsyncTypedSource<'s, str>>(&mut self, value: S) -> Result;
}
*/
