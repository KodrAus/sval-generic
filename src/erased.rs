use crate::{stream, value, Result};

pub trait Value {
    fn erased_stream<'a>(&'a self, stream: &mut dyn Stream<'a>) -> Result;
}

impl<T: ?Sized> Value for T
where
    T: value::Value,
{
    fn erased_stream<'a>(&'a self, stream: &mut dyn Stream<'a>) -> Result {
        self.stream(stream)
    }
}

impl<'a> value::Value for (dyn Value + 'a) {
    fn stream<'b, S>(&'b self, mut stream: S) -> Result
    where
        S: stream::Stream<'b>,
    {
        self.erased_stream(&mut stream)
    }
}

pub trait Stream<'a> {
    fn erased_i128(&mut self, v: i128) -> Result;
    fn erased_u128(&mut self, v: u128) -> Result;
    fn erased_str<'o>(&mut self, v: ValueRef<'a, 'o, str>) -> Result;
}

impl<'a, T: ?Sized> Stream<'a> for T
where
    T: stream::Stream<'a>,
{
    fn erased_i128(&mut self, v: i128) -> Result {
        self.i128(v)
    }

    fn erased_u128(&mut self, v: u128) -> Result {
        self.u128(v)
    }

    fn erased_str<'o>(&mut self, v: ValueRef<'a, 'o, str>) -> Result {
        match v {
            ValueRef::Ref(v) => self.str(v),
            ValueRef::Any(v) => self.str(value::any_ref(v)),
        }
    }
}

impl<'a, 'b> stream::Stream<'a> for (dyn Stream<'a> + 'b) {
    fn u128(&mut self, v: u128) -> Result {
        self.erased_u128(v)
    }

    fn i128(&mut self, v: i128) -> Result {
        self.erased_i128(v)
    }

    fn str<V: stream::ValueRef<'a, Target = str>>(&mut self, v: V) -> Result {
        match v.try_into_ref() {
            Ok(v) => self.erased_str(ValueRef::Ref(v)),
            Err(stream::IntoRefError(v)) => self.erased_str(ValueRef::Any(&*v)),
        }
    }

    fn map_begin(&mut self, len: Option<usize>) -> Result {
        Ok(())
    }

    fn map_key_begin(&mut self) -> Result {
        Ok(())
    }

    fn map_value_begin(&mut self) -> Result {
        Ok(())
    }

    fn map_end(&mut self) -> Result {
        Ok(())
    }
}

pub enum ValueRef<'a, 'o, T: ?Sized> {
    Ref(&'a T),
    Any(&'o T),
}
