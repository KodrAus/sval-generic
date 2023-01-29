use core::fmt;

pub fn stream<'sval, S: sval::Stream<'sval> + ?Sized, V: serde::Serialize>(
    stream: &mut S,
    value: &'_ V,
) -> sval::Result {
    stream.value_computed(ToValue::new_ref(value))
}

pub fn to_value<V: serde::Serialize>(value: V) -> ToValue<V> {
    ToValue(value)
}

#[repr(transparent)]
pub struct ToValue<V: ?Sized>(V);

impl<V> ToValue<V> {
    pub fn new(value: V) -> Self {
        ToValue(value)
    }
}

impl<V: ?Sized> ToValue<V> {
    pub fn new_ref<'a>(value: &'a V) -> &'a Self {
        unsafe { &*(value as *const _ as *const ToValue<V>) }
    }
}

impl<V: serde::Serialize> sval::Value for ToValue<V> {
    fn stream<'sval, S: sval::Stream<'sval> + ?Sized>(&'sval self, stream: &mut S) -> sval::Result {
        self.0.serialize(Stream(stream))?;

        Ok(())
    }
}

struct Stream<S>(S);

#[derive(Debug)]
struct Error;

impl From<Error> for sval::Error {
    fn from(_: Error) -> sval::Error {
        sval::Error::new()
    }
}

impl From<sval::Error> for Error {
    fn from(_: sval::Error) -> Error {
        Error
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "failed to stream a value")
    }
}

impl serde::ser::Error for Error {
    fn custom<T>(_: T) -> Self
    where
        T: fmt::Display,
    {
        Error
    }
}

impl serde::ser::StdError for Error {}

impl<'sval, S: sval::Stream<'sval>> Stream<S> {
    fn stream_value(&mut self, v: impl sval::Value) -> Result<(), Error> {
        self.0.value_computed(&v)?;

        Ok(())
    }
}

impl<'sval, S: sval::Stream<'sval>> serde::Serializer for Stream<S> {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(mut self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.stream_value(v)
    }

    fn serialize_i8(mut self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.stream_value(v)
    }

    fn serialize_i16(mut self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.stream_value(v)
    }

    fn serialize_i32(mut self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.stream_value(v)
    }

    fn serialize_i64(mut self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.stream_value(v)
    }

    fn serialize_u8(mut self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.stream_value(v)
    }

    fn serialize_u16(mut self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.stream_value(v)
    }

    fn serialize_u32(mut self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.stream_value(v)
    }

    fn serialize_u64(mut self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.stream_value(v)
    }

    fn serialize_f32(mut self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.stream_value(v)
    }

    fn serialize_f64(mut self, v: f64) -> Result<Self::Ok, Self::Error> {
        self.stream_value(v)
    }

    fn serialize_char(mut self, v: char) -> Result<Self::Ok, Self::Error> {
        self.stream_value(v)
    }

    fn serialize_str(mut self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.stream_value(v)
    }

    fn serialize_bytes(mut self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        self.stream_value(Bytes(v))
    }

    fn serialize_none(mut self) -> Result<Self::Ok, Self::Error> {
        self.stream_value(None::<()>)
    }

    fn serialize_some<T: ?Sized>(mut self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        self.stream_value(Some(ToValue(value)))
    }

    fn serialize_unit(mut self) -> Result<Self::Ok, Self::Error> {
        self.stream_value(())
    }

    fn serialize_unit_struct(mut self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        todo!()
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        todo!()
    }

    fn serialize_seq(mut self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        todo!()
    }

    fn serialize_tuple(mut self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        todo!()
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        todo!()
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        todo!()
    }

    fn serialize_map(mut self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        todo!()
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        todo!()
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        todo!()
    }
}

impl<'sval, S: sval::Stream<'sval>> serde::ser::SerializeSeq for Stream<S> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        todo!()
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl<'sval, S: sval::Stream<'sval>> serde::ser::SerializeTuple for Stream<S> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        todo!()
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl<'sval, S: sval::Stream<'sval>> serde::ser::SerializeTupleStruct for Stream<S> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        todo!()
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl<'sval, S: sval::Stream<'sval>> serde::ser::SerializeTupleVariant for Stream<S> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        todo!()
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl<'sval, S: sval::Stream<'sval>> serde::ser::SerializeMap for Stream<S> {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        todo!()
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        todo!()
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl<'sval, S: sval::Stream<'sval>> serde::ser::SerializeStruct for Stream<S> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        todo!()
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl<'sval, S: sval::Stream<'sval>> serde::ser::SerializeStructVariant for Stream<S> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        todo!()
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

struct Bytes<'a>(&'a [u8]);

impl<'a> sval::Value for Bytes<'a> {
    fn stream<'sval, S: sval::Stream<'sval> + ?Sized>(&'sval self, stream: &mut S) -> sval::Result {
        stream.binary_begin(Some(self.0.len()))?;
        stream.binary_fragment(self.0)?;
        stream.binary_end()
    }
}
