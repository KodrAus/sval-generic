use std::convert::TryInto;

use sval_generic_api::{receiver, source, tag, value, Error, Receiver, Result};

use sval_generic_api_buffer::{self as buffer, BufferReceiver};

use serde::ser::{
    Error as _, Serialize, SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant,
    SerializeTupleStruct, SerializeTupleVariant, Serializer,
};

pub struct Value<V>(V);

impl<V> Value<V> {
    pub fn new(value: V) -> Self {
        Value(value)
    }
}

pub fn value<V: value::Value>(value: V) -> Value<V> {
    Value::new(value)
}

impl<V: value::Value> Serialize for Value<V> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut stream = SerdeReceiver::begin(serializer);
        self.0
            .stream(&mut stream)
            .map_err(|_| S::Error::custom("failed to serialize value"))?;
        stream.end()
    }
}

struct Display<D>(D);

impl<D> Display<D> {
    fn new(value: D) -> Self {
        Display(value)
    }
}

impl<D: receiver::Display> Serialize for Display<D> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(&self.0)
    }
}

enum SerdeReceiver<S: Serializer> {
    Serializer(Option<StreamSerializer<S>>),
    SerializeMap(Option<StreamSerializeMap<S>>),
    SerializeStruct(Option<StreamSerializeStruct<S>>),
    SerializeStructVariant(Option<StreamSerializeStructVariant<S>>),
    SerializeSeq(Option<StreamSerializeSeq<S>>),
    SerializeTupleStruct(Option<StreamSerializeTupleStruct<S>>),
    SerializeTupleVariant(Option<StreamSerializeTupleVariant<S>>),
    Done(S::Ok),
}

struct StreamSerializer<S: Serializer> {
    serializer: S,
    type_tag: Option<&'static str>,
    variant_tag: Option<&'static str>,
    variant_index: Option<u32>,
}

struct StreamSerializeMap<S: Serializer> {
    serializer: S::SerializeMap,
    is_key: bool,
}

struct StreamSerializeStruct<S: Serializer> {
    serializer: S::SerializeStruct,
    field: Option<&'static str>,
}

struct StreamSerializeStructVariant<S: Serializer> {
    serializer: S::SerializeStructVariant,
    field: Option<&'static str>,
}

struct StreamSerializeSeq<S: Serializer> {
    serializer: S::SerializeSeq,
}

struct StreamSerializeTupleStruct<S: Serializer> {
    serializer: S::SerializeTupleStruct,
}

struct StreamSerializeTupleVariant<S: Serializer> {
    serializer: S::SerializeTupleVariant,
}

impl<S: Serializer> SerdeReceiver<S> {
    fn begin(serializer: S) -> Self {
        SerdeReceiver::Serializer(Some(StreamSerializer {
            serializer,
            type_tag: None,
            variant_tag: None,
            variant_index: None,
        }))
    }

    fn end(self) -> Result<S::Ok, S::Error> {
        if let SerdeReceiver::Done(ok) = self {
            Ok(ok)
        } else {
            Err(S::Error::custom("the stream is incomplete"))
        }
    }

    fn serializer(&mut self) -> Result<&mut StreamSerializer<S>> {
        if let SerdeReceiver::Serializer(Some(ref mut stream)) = self {
            Ok(stream)
        } else {
            Err(Error)
        }
    }

    fn serialize_source(&mut self, v: impl Serialize) -> Result {
        match self {
            // A standard serializer can appear at source level of serialization
            // The serializer is taken by value and returns the final result
            SerdeReceiver::Serializer(stream) => match stream.take() {
                Some(stream) => {
                    *self = SerdeReceiver::Done(v.serialize(stream.serializer).map_err(|_| Error)?);

                    Ok(())
                }
                None => Err(Error),
            },

            // If the serializer is inside a map then keys and values can be serialized independently
            // Serialize a map key
            SerdeReceiver::SerializeMap(Some(StreamSerializeMap {
                ref mut serializer,
                is_key: true,
            })) => serializer.serialize_key(&v).map_err(|_| Error),
            // Serialize a map value
            SerdeReceiver::SerializeMap(Some(StreamSerializeMap {
                ref mut serializer,
                is_key: false,
            })) => serializer.serialize_value(&v).map_err(|_| Error),
            SerdeReceiver::SerializeMap(_) => Err(Error),

            // If the serializer is inside a struct then expect the field to already be provided
            SerdeReceiver::SerializeStruct(Some(StreamSerializeStruct {
                ref mut serializer,
                field: Some(field),
            })) => serializer.serialize_field(field, &v).map_err(|_| Error),
            SerdeReceiver::SerializeStruct(_) => Err(Error),

            // If the serializer is inside a struct variant then expect the field to already be provided
            SerdeReceiver::SerializeStructVariant(Some(StreamSerializeStructVariant {
                ref mut serializer,
                field: Some(field),
            })) => serializer.serialize_field(field, &v).map_err(|_| Error),
            SerdeReceiver::SerializeStructVariant(_) => Err(Error),

            // If the serializer is inside a seq then serialize an element
            SerdeReceiver::SerializeSeq(Some(StreamSerializeSeq { ref mut serializer })) => {
                serializer.serialize_element(&v).map_err(|_| Error)
            }
            SerdeReceiver::SerializeSeq(_) => Err(Error),

            // If the serializer is inside a tuple struct then serialize a field
            // Fields in tuples are unnamed so they don't need to be provided
            SerdeReceiver::SerializeTupleStruct(Some(StreamSerializeTupleStruct {
                ref mut serializer,
            })) => serializer.serialize_field(&v).map_err(|_| Error),
            SerdeReceiver::SerializeTupleStruct(_) => Err(Error),

            // If the serializer is inside a tuple variant then serialize a field
            SerdeReceiver::SerializeTupleVariant(Some(StreamSerializeTupleVariant {
                ref mut serializer,
            })) => serializer.serialize_field(&v).map_err(|_| Error),
            SerdeReceiver::SerializeTupleVariant(_) => Err(Error),

            // If the serializer is already complete then we shouldn't still be sending it values
            SerdeReceiver::Done(_) => Err(Error),
        }
    }

    fn serialize_map_begin(&mut self, len: Option<usize>) -> Result {
        match self {
            SerdeReceiver::Serializer(stream) => match stream.take() {
                // Begin a serializer for a struct
                Some(StreamSerializer {
                    serializer,
                    type_tag: Some(ty),
                    variant_tag: None,
                    variant_index: None,
                }) => {
                    *self = SerdeReceiver::SerializeStruct(Some(StreamSerializeStruct {
                        serializer: serializer
                            .serialize_struct(ty, len.ok_or(Error)?)
                            .map_err(|_| Error)?,
                        field: None,
                    }));

                    Ok(())
                }
                // Begin a serializer for a plain anonymous map
                Some(StreamSerializer {
                    serializer,
                    type_tag: None,
                    variant_tag: None,
                    variant_index: None,
                }) => {
                    *self = SerdeReceiver::SerializeMap(Some(StreamSerializeMap {
                        serializer: serializer.serialize_map(len).map_err(|_| Error)?,
                        is_key: false,
                    }));

                    Ok(())
                }
                // Begin a serializer for a struct-like enum variant
                Some(StreamSerializer {
                    serializer,
                    type_tag: Some(ty),
                    variant_tag: Some(variant),
                    variant_index: Some(index),
                }) => {
                    *self =
                        SerdeReceiver::SerializeStructVariant(Some(StreamSerializeStructVariant {
                            serializer: serializer
                                .serialize_struct_variant(ty, index, variant, len.ok_or(Error)?)
                                .map_err(|_| Error)?,
                            field: None,
                        }));

                    Ok(())
                }
                // In source other case we can't begin a serializer
                _ => Err(Error),
            },
            _ => Err(Error),
        }
    }

    fn serialize_map_key_begin(&mut self) -> Result {
        match self {
            // An anonymous map needs to know whether to expect a key
            SerdeReceiver::SerializeMap(Some(ref mut stream)) => {
                stream.is_key = true;

                Ok(())
            }
            // Struct maps don't require key tracking
            SerdeReceiver::SerializeStruct(Some(_)) => Ok(()),
            // Struct variant maps don't require key tracking
            SerdeReceiver::SerializeStructVariant(Some(_)) => Ok(()),
            _ => Err(Error),
        }
    }

    fn serialize_map_key_end(&mut self) -> Result {
        match self {
            // An anonymous map needs to know whether to expect a key
            SerdeReceiver::SerializeMap(Some(ref mut stream)) => {
                stream.is_key = false;

                Ok(())
            }
            // Struct maps don't require key tracking
            SerdeReceiver::SerializeStruct(Some(_)) => Ok(()),
            // Struct variant maps don't require key tracking
            SerdeReceiver::SerializeStructVariant(Some(_)) => Ok(()),
            _ => Err(Error),
        }
    }

    fn serialize_map_field(&mut self, field: Result<&'static str, &str>) -> Result {
        match self {
            // An anonymous map can accept either a static or non-static field name
            SerdeReceiver::SerializeMap(Some(StreamSerializeMap {
                ref mut serializer,
                is_key: false,
            })) => match field {
                Ok(field) => serializer.serialize_key(field).map_err(|_| Error),
                Err(field) => serializer.serialize_key(field).map_err(|_| Error),
            },
            // Struct maps require a static field
            SerdeReceiver::SerializeStruct(Some(ref mut stream)) => {
                stream.field = field.ok();
                Ok(())
            }
            // Struct variant maps require a static field
            SerdeReceiver::SerializeStructVariant(Some(ref mut stream)) => {
                stream.field = field.ok();

                Ok(())
            }
            _ => Err(Error),
        }
    }

    fn serialize_map_end(&mut self) -> Result {
        match self {
            // Complete an anonymous map
            SerdeReceiver::SerializeMap(stream) => match stream.take() {
                Some(stream) => {
                    *self = SerdeReceiver::Done(stream.serializer.end().map_err(|_| Error)?);
                    Ok(())
                }
                None => Err(Error),
            },
            // Complete a struct
            SerdeReceiver::SerializeStruct(stream) => match stream.take() {
                Some(stream) => {
                    *self = SerdeReceiver::Done(stream.serializer.end().map_err(|_| Error)?);
                    Ok(())
                }
                None => Err(Error),
            },
            // Complete a struct variant
            SerdeReceiver::SerializeStructVariant(stream) => match stream.take() {
                Some(stream) => {
                    *self = SerdeReceiver::Done(stream.serializer.end().map_err(|_| Error)?);
                    Ok(())
                }
                None => Err(Error),
            },
            _ => Err(Error),
        }
    }

    fn serialize_seq_begin(&mut self, len: Option<usize>) -> Result {
        match self {
            SerdeReceiver::Serializer(stream) => match stream.take() {
                // Begin a serializer for a tuple struct
                Some(StreamSerializer {
                    serializer,
                    type_tag: Some(ty),
                    variant_tag: None,
                    variant_index: None,
                }) => {
                    *self = SerdeReceiver::SerializeTupleStruct(Some(StreamSerializeTupleStruct {
                        serializer: serializer
                            .serialize_tuple_struct(ty, len.ok_or(Error)?)
                            .map_err(|_| Error)?,
                    }));

                    Ok(())
                }
                // Begin a serializer for a plain anonymous seq
                Some(StreamSerializer {
                    serializer,
                    type_tag: None,
                    variant_tag: None,
                    variant_index: None,
                }) => {
                    *self = SerdeReceiver::SerializeSeq(Some(StreamSerializeSeq {
                        serializer: serializer.serialize_seq(len).map_err(|_| Error)?,
                    }));

                    Ok(())
                }
                // Begin a serializer for a tuple-like enum variant
                Some(StreamSerializer {
                    serializer,
                    type_tag: Some(ty),
                    variant_tag: Some(variant),
                    variant_index: Some(index),
                }) => {
                    *self =
                        SerdeReceiver::SerializeTupleVariant(Some(StreamSerializeTupleVariant {
                            serializer: serializer
                                .serialize_tuple_variant(ty, index, variant, len.ok_or(Error)?)
                                .map_err(|_| Error)?,
                        }));

                    Ok(())
                }
                // In source other case we can't begin a serializer
                _ => Err(Error),
            },
            _ => Err(Error),
        }
    }

    fn serialize_seq_end(&mut self) -> Result {
        match self {
            // Complete an anonymous seq
            SerdeReceiver::SerializeSeq(stream) => match stream.take() {
                Some(stream) => {
                    *self = SerdeReceiver::Done(stream.serializer.end().map_err(|_| Error)?);
                    Ok(())
                }
                None => Err(Error),
            },
            // Complete a tuple struct
            SerdeReceiver::SerializeTupleStruct(stream) => match stream.take() {
                Some(stream) => {
                    *self = SerdeReceiver::Done(stream.serializer.end().map_err(|_| Error)?);
                    Ok(())
                }
                None => Err(Error),
            },
            // Complete a tuple variant
            SerdeReceiver::SerializeTupleVariant(stream) => match stream.take() {
                Some(stream) => {
                    *self = SerdeReceiver::Done(stream.serializer.end().map_err(|_| Error)?);
                    Ok(())
                }
                None => Err(Error),
            },
            _ => Err(Error),
        }
    }
}

impl<'a, S: Serializer> Receiver<'a> for SerdeReceiver<S> {
    fn source<'b: 'a, V: source::Source<'b>>(&mut self, v: V) -> Result {
        buffer::stream(self, v)
    }

    fn display<D: receiver::Display>(&mut self, v: D) -> Result {
        self.serialize_source(Display::new(v))
    }

    fn u64(&mut self, v: u64) -> Result {
        self.serialize_source(v)
    }

    fn i64(&mut self, v: i64) -> Result {
        self.serialize_source(v)
    }

    fn u128(&mut self, v: u128) -> Result {
        self.serialize_source(v)
    }

    fn i128(&mut self, v: i128) -> Result {
        self.serialize_source(v)
    }

    fn f64(&mut self, v: f64) -> Result {
        self.serialize_source(v)
    }

    fn bool(&mut self, v: bool) -> Result {
        self.serialize_source(v)
    }

    fn none(&mut self) -> Result {
        self.serialize_source(None::<()>)
    }

    fn str<'s: 'a, T: source::ValueSource<'s, str>>(&mut self, mut v: T) -> Result {
        self.serialize_source(v.value()?)
    }

    fn type_tagged_begin<T: source::ValueSource<'static, str>>(
        &mut self,
        mut tag: tag::TypeTag<T>,
    ) -> Result {
        self.serializer()?.type_tag = tag.ty.value_ref().ok();

        Ok(())
    }

    fn type_tagged_end(&mut self) -> Result {
        Ok(())
    }

    fn variant_tagged_begin<
        T: source::ValueSource<'static, str>,
        K: source::ValueSource<'static, str>,
    >(
        &mut self,
        mut tag: tag::VariantTag<T, K>,
    ) -> Result {
        let serializer = self.serializer()?;

        serializer.type_tag = tag.ty.value_ref().ok();
        serializer.variant_tag = tag.variant_key.value_ref().ok();
        serializer.variant_index = tag.variant_index.and_then(|index| index.try_into().ok());

        Ok(())
    }

    fn variant_tagged_end(&mut self) -> Result {
        Ok(())
    }

    fn map_begin(&mut self, len: Option<usize>) -> Result {
        self.serialize_map_begin(len)
    }

    fn map_end(&mut self) -> Result {
        self.serialize_map_end()
    }

    fn map_key_begin(&mut self) -> Result {
        self.serialize_map_key_begin()
    }

    fn map_key_end(&mut self) -> Result {
        self.serialize_map_key_end()
    }

    fn map_value_begin(&mut self) -> Result {
        Ok(())
    }

    fn map_value_end(&mut self) -> Result {
        Ok(())
    }

    fn map_field<T: source::ValueSource<'static, str>>(&mut self, mut field: T) -> Result {
        match field.value_ref() {
            Ok(field) => self.serialize_map_field(Ok(field)),
            Err(field) => self.serialize_map_field(Err(field.into_result()?)),
        }
    }

    fn seq_begin(&mut self, len: Option<usize>) -> Result {
        self.serialize_seq_begin(len)
    }

    fn seq_end(&mut self) -> Result {
        self.serialize_seq_end()
    }

    fn seq_elem_begin(&mut self) -> Result {
        Ok(())
    }

    fn seq_elem_end(&mut self) -> Result {
        Ok(())
    }
}

impl<'a, S: Serializer> BufferReceiver<'a> for SerdeReceiver<S> {
    fn value_source<'v: 'a, T: value::Value + ?Sized + 'v, VS: source::ValueSource<'v, T>>(
        &mut self,
        mut v: VS,
    ) -> Result {
        self.serialize_source(Value::new(v.value()?))
    }
}