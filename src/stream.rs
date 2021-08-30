pub use crate::{
    value_ref::{AnyValueRef, TypedValueRef},
    Error, Result,
};

pub trait Stream<'a> {
    fn u128(&mut self, v: u128) -> Result;
    fn i128(&mut self, v: i128) -> Result;

    fn str<'v, V: TypedValueRef<'v, str>>(&mut self, v: V) -> Result
    where
        'v: 'a;

    fn map_begin(&mut self, len: Option<usize>) -> Result;
    fn map_key_begin(&mut self) -> Result;
    fn map_value_begin(&mut self) -> Result;
    fn map_end(&mut self) -> Result;

    fn map_key<'k, K: AnyValueRef<'k>>(&mut self, k: K) -> Result
    where
        'k: 'a,
    {
        self.map_key_begin()?;
        k.stream(self)
    }

    fn map_value<'v, V: AnyValueRef<'v>>(&mut self, v: V) -> Result
    where
        'v: 'a,
    {
        self.map_value_begin()?;
        v.stream(self)
    }

    fn map_entry<'k, 'v, K: AnyValueRef<'k>, V: AnyValueRef<'v>>(&mut self, k: K, v: V) -> Result
    where
        'k: 'a,
        'v: 'a,
    {
        self.map_key(k)?;
        self.map_value(v)
    }

    fn map_field<'v, F: TypedValueRef<'static, str>, V: AnyValueRef<'v>>(&mut self, f: F, v: V) -> Result
    where
        'v: 'a,
    {
        self.map_entry(f, v)
    }
}

impl<'a, 'b, T: ?Sized> Stream<'b> for &'a mut T
where
    T: Stream<'b>,
{
    fn u128(&mut self, v: u128) -> Result {
        (**self).u128(v)
    }

    fn i128(&mut self, v: i128) -> Result {
        (**self).i128(v)
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

    fn map_key<'k, K: AnyValueRef<'k>>(&mut self, k: K) -> Result
    where
        'k: 'b,
    {
        (**self).map_key(k)
    }

    fn map_value<'v, V: AnyValueRef<'v>>(&mut self, v: V) -> Result
    where
        'v: 'b,
    {
        (**self).map_value(v)
    }

    fn map_entry<'k, 'v, K: AnyValueRef<'k>, V: AnyValueRef<'v>>(&mut self, k: K, v: V) -> Result
    where
        'k: 'b,
        'v: 'b,
    {
        (**self).map_entry(k, v)
    }

    fn map_field<'v, F: TypedValueRef<'static, str>, V: AnyValueRef<'v>>(&mut self, f: F, v: V) -> Result
    where
        'v: 'b,
    {
        (**self).map_field(f, v)
    }
}
