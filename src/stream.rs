pub use crate::{
    erased,
    value_ref::{TypedValue, UntypedValue},
    Error, Result,
};

pub trait Stream<'a> {
    fn u128(&mut self, v: u128) -> Result;
    fn i128(&mut self, v: i128) -> Result;

    fn str<V: TypedValue<'a, str>>(&mut self, v: V) -> Result;

    fn map_begin(&mut self, len: Option<usize>) -> Result;
    fn map_key_begin(&mut self) -> Result;
    fn map_value_begin(&mut self) -> Result;
    fn map_end(&mut self) -> Result;

    fn map_key<K: UntypedValue<'a>>(&mut self, k: K) -> Result {
        self.map_key_begin()?;
        k.stream(self)
    }

    fn map_value<V: UntypedValue<'a>>(&mut self, v: V) -> Result {
        self.map_value_begin()?;
        v.stream(self)
    }

    fn map_entry<K: UntypedValue<'a>, V: UntypedValue<'a>>(&mut self, k: K, v: V) -> Result {
        self.map_key(k)?;
        self.map_value(v)
    }

    fn map_field<F: TypedValue<'static, str>, V: UntypedValue<'a>>(
        &mut self,
        f: F,
        v: V,
    ) -> Result {
        self.map_entry(f.for_all(), v)
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

    fn str<V: TypedValue<'b, str>>(&mut self, v: V) -> Result {
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

    fn map_key<K: UntypedValue<'b>>(&mut self, k: K) -> Result {
        (**self).map_key(k)
    }

    fn map_value<V: UntypedValue<'b>>(&mut self, v: V) -> Result {
        (**self).map_value(v)
    }

    fn map_entry<K: UntypedValue<'b>, V: UntypedValue<'b>>(&mut self, k: K, v: V) -> Result {
        (**self).map_entry(k, v)
    }

    fn map_field<F: TypedValue<'static, str>, V: UntypedValue<'b>>(
        &mut self,
        f: F,
        v: V,
    ) -> Result {
        (**self).map_field(f, v)
    }
}
