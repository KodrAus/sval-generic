use crate::erased;

#[doc(inline)]
pub use crate::{
    for_all::ForAll,
    value_ref::{TypedValueRef, UnknownValueRef},
    Error, Result,
};

pub trait Stream<'a> {
    fn u64(&mut self, v: u64) -> Result;
    fn i64(&mut self, v: i64) -> Result;
    fn u128(&mut self, v: u128) -> Result;
    fn i128(&mut self, v: i128) -> Result;
    fn f64(&mut self, v: f64) -> Result;
    fn bool(&mut self, v: bool) -> Result;
    fn none(&mut self) -> Result;

    fn str<'v, V: TypedValueRef<'v, str>>(&mut self, v: V) -> Result
    where
        'v: 'a;

    fn map_begin(&mut self, len: Option<usize>) -> Result;
    fn map_key_begin(&mut self) -> Result;
    fn map_value_begin(&mut self) -> Result;
    fn map_end(&mut self) -> Result;

    fn map_key<'k, K: UnknownValueRef<'k>>(&mut self, k: K) -> Result
    where
        'k: 'a,
    {
        self.map_key_begin()?;
        k.stream(self)
    }

    fn map_value<'v, V: UnknownValueRef<'v>>(&mut self, v: V) -> Result
    where
        'v: 'a,
    {
        self.map_value_begin()?;
        v.stream(self)
    }

    fn map_entry<'k, 'v, K: UnknownValueRef<'k>, V: UnknownValueRef<'v>>(
        &mut self,
        k: K,
        v: V,
    ) -> Result
    where
        'k: 'a,
        'v: 'a,
    {
        self.map_key(k)?;
        self.map_value(v)
    }

    fn map_field<'v, F: TypedValueRef<'static, str>, V: UnknownValueRef<'v>>(
        &mut self,
        f: F,
        v: V,
    ) -> Result
    where
        'v: 'a,
    {
        self.map_entry(f, v)
    }

    fn seq_begin(&mut self, len: Option<usize>) -> Result;
    fn seq_elem_begin(&mut self) -> Result;
    fn seq_end(&mut self) -> Result;

    fn seq_elem<'e, E: UnknownValueRef<'e>>(&mut self, e: E) -> Result
    where
        'e: 'a,
    {
        self.seq_elem_begin()?;
        e.stream(self)
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

impl<'a, 'b, T: ?Sized> Stream<'b> for &'a mut T
where
    T: Stream<'b>,
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

    fn str<'v, V: TypedValueRef<'v, str>>(&mut self, v: V) -> Result
    where
        'v: 'b,
    {
        (**self).str(v)
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

    fn map_key<'k, K: UnknownValueRef<'k>>(&mut self, k: K) -> Result
    where
        'k: 'b,
    {
        (**self).map_key(k)
    }

    fn map_value<'v, V: UnknownValueRef<'v>>(&mut self, v: V) -> Result
    where
        'v: 'b,
    {
        (**self).map_value(v)
    }

    fn map_entry<'k, 'v, K: UnknownValueRef<'k>, V: UnknownValueRef<'v>>(
        &mut self,
        k: K,
        v: V,
    ) -> Result
    where
        'k: 'b,
        'v: 'b,
    {
        (**self).map_entry(k, v)
    }

    fn map_field<'v, F: TypedValueRef<'static, str>, V: UnknownValueRef<'v>>(
        &mut self,
        f: F,
        v: V,
    ) -> Result
    where
        'v: 'b,
    {
        (**self).map_field(f, v)
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

    fn seq_elem<'e, E: UnknownValueRef<'e>>(&mut self, e: E) -> Result
    where
        'e: 'b,
    {
        (**self).seq_elem(e)
    }
}
