pub use crate::{
    erased,
    value_ref::{IntoRefError, ValueRef},
    Error, Result,
};

pub trait Stream<'a> {
    fn u128(&mut self, v: u128) -> Result;
    fn i128(&mut self, v: i128) -> Result;

    fn str<V: ValueRef<'a, Target = str>>(&mut self, v: V) -> Result;

    fn map_begin(&mut self, len: Option<usize>) -> Result;
    fn map_key_begin(&mut self) -> Result;
    fn map_value_begin(&mut self) -> Result;
    fn map_end(&mut self) -> Result;

    fn map_key<K: ValueRef<'a>>(&mut self, k: K) -> Result {
        self.map_key_begin()?;
        k.stream_ref(self)
    }

    fn map_value<V: ValueRef<'a>>(&mut self, v: V) -> Result {
        self.map_value_begin()?;
        v.stream_ref(self)
    }

    fn map_entry<K: ValueRef<'a>, V: ValueRef<'a>>(&mut self, k: K, v: V) -> Result {
        self.map_key(k)?;
        self.map_value(v)
    }

    fn map_field<F: ValueRef<'static, Target = str>, V: ValueRef<'a>>(
        &mut self,
        f: F,
        v: V,
    ) -> Result {
        self.map_entry(f.any_ref(), v)
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

    fn str<V: ValueRef<'b, Target = str>>(&mut self, v: V) -> Result {
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

    fn map_key<K: ValueRef<'b>>(&mut self, k: K) -> Result {
        (**self).map_key(k)
    }

    fn map_value<V: ValueRef<'b>>(&mut self, v: V) -> Result {
        (**self).map_value(v)
    }

    fn map_entry<K: ValueRef<'b>, V: ValueRef<'b>>(&mut self, k: K, v: V) -> Result {
        (**self).map_entry(k, v)
    }

    fn map_field<F: ValueRef<'static, Target = str>, V: ValueRef<'b>>(
        &mut self,
        f: F,
        v: V,
    ) -> Result {
        (**self).map_field(f, v)
    }
}
