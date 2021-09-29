use std::convert::TryInto;

use crate::{
    stream::{self, Stream},
    value,
};

use serde::ser::{
    Error as _, Serialize, SerializeMap, SerializeStruct, SerializeStructVariant, Serializer,
};

pub struct Value<V>(V);

impl<V> Value<V> {
    pub fn new(value: V) -> Self {
        Value(value)
    }
}

impl<V: value::Value> Serialize for Value<V> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut stream = SerdeStream::begin(serializer);
        self.0
            .stream(&mut stream)
            .map_err(|_| S::Error::custom("failed to serialize value"))?;
        stream.end()
    }
}

enum SerdeStream<S: Serializer> {
    Serializer(Option<StreamSerializer<S>>),
    SerializeMap(Option<StreamSerializeMap<S>>),
    SerializeStruct(Option<StreamSerializeStruct<S>>),
    SerializeStructVariant(Option<StreamSerializeStructVariant<S>>),
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

impl<S: Serializer> SerdeStream<S> {
    fn begin(serializer: S) -> Self {
        SerdeStream::Serializer(Some(StreamSerializer {
            serializer,
            type_tag: None,
            variant_tag: None,
            variant_index: None,
        }))
    }

    fn end(self) -> Result<S::Ok, S::Error> {
        if let SerdeStream::Done(ok) = self {
            Ok(ok)
        } else {
            Err(S::Error::custom("the stream is incomplete"))
        }
    }

    fn serializer(&mut self) -> stream::Result<&mut StreamSerializer<S>> {
        if let SerdeStream::Serializer(Some(ref mut stream)) = self {
            Ok(stream)
        } else {
            Err(stream::Error)
        }
    }

    fn serialize_any(&mut self, v: impl Serialize) -> stream::Result {
        match self {
            // A standard serializer can appear at any level of serialization
            // The serializer is taken by value and returns the final result
            SerdeStream::Serializer(stream) => match stream.take() {
                Some(stream) => {
                    *self = SerdeStream::Done(
                        v.serialize(stream.serializer).map_err(|_| stream::Error)?,
                    );
                    Ok(())
                }
                None => Err(stream::Error),
            },
            // If the serializer is inside a map then keys and values can be serialized
            // independently
            SerdeStream::SerializeMap(Some(StreamSerializeMap {
                ref mut serializer,
                is_key: true,
            })) => serializer.serialize_key(&v).map_err(|_| stream::Error),
            SerdeStream::SerializeMap(Some(StreamSerializeMap {
                ref mut serializer,
                is_key: false,
            })) => serializer.serialize_value(&v).map_err(|_| stream::Error),
            SerdeStream::SerializeMap(_) => Err(stream::Error),
            // If the serializer is inside a struct then expect the field to already be provided
            SerdeStream::SerializeStruct(Some(StreamSerializeStruct {
                ref mut serializer,
                field: Some(field),
            })) => serializer
                .serialize_field(field, &v)
                .map_err(|_| stream::Error),
            SerdeStream::SerializeStruct(_) => Err(stream::Error),
            // If the serializer is inside a struct variant then expect the field to alreay be provided
            SerdeStream::SerializeStructVariant(Some(StreamSerializeStructVariant {
                ref mut serializer,
                field: Some(field),
            })) => serializer
                .serialize_field(field, &v)
                .map_err(|_| stream::Error),
            SerdeStream::SerializeStructVariant(_) => Err(stream::Error),
            // If the serializer is already complete then we shouldn't still be sending it values
            SerdeStream::Done(_) => Err(stream::Error),
        }
    }

    fn serialize_map(&mut self, len: Option<usize>) -> stream::Result {
        match self {
            SerdeStream::Serializer(stream) => match stream.take() {
                // Begin a serializer for a struct
                Some(StreamSerializer {
                    serializer,
                    type_tag: Some(ty),
                    variant_tag: None,
                    variant_index: None,
                }) => {
                    *self = SerdeStream::SerializeStruct(Some(StreamSerializeStruct {
                        serializer: serializer
                            .serialize_struct(ty, len.ok_or(stream::Error)?)
                            .map_err(|_| stream::Error)?,
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
                    *self = SerdeStream::SerializeMap(Some(StreamSerializeMap {
                        serializer: serializer.serialize_map(len).map_err(|_| stream::Error)?,
                        is_key: false,
                    }));

                    Ok(())
                }
                // Begin a serializer for a stuct-like enum variant
                Some(StreamSerializer {
                    serializer,
                    type_tag: Some(ty),
                    variant_tag: Some(variant),
                    variant_index: Some(index),
                }) => {
                    *self =
                        SerdeStream::SerializeStructVariant(Some(StreamSerializeStructVariant {
                            serializer: serializer
                                .serialize_struct_variant(
                                    ty,
                                    index,
                                    variant,
                                    len.ok_or(stream::Error)?,
                                )
                                .map_err(|_| stream::Error)?,
                            field: None,
                        }));

                    Ok(())
                }
                // In any other case we can't begin a serializer
                _ => Err(stream::Error),
            },
            _ => Err(stream::Error),
        }
    }

    fn serialize_map_field(&mut self, field: Result<&'static str, &str>) -> stream::Result {
        match self {
            // An anonymous map can accept either a static or non-static field name
            SerdeStream::SerializeMap(Some(StreamSerializeMap {
                ref mut serializer,
                is_key: true,
            })) => match field {
                Ok(field) => serializer.serialize_key(field).map_err(|_| stream::Error),
                Err(field) => serializer.serialize_key(field).map_err(|_| stream::Error),
            },
            // Struct maps require a static field
            SerdeStream::SerializeStruct(Some(ref mut stream)) => {
                stream.field = field.ok();
                Ok(())
            }
            // Struct variant maps require a static field
            SerdeStream::SerializeStructVariant(Some(ref mut stream)) => {
                stream.field = field.ok();
                Ok(())
            }
            _ => Err(stream::Error),
        }
    }

    fn serialize_map_end(&mut self) -> stream::Result {
        match self {
            // Complete an anonymous map
            SerdeStream::SerializeMap(stream) => match stream.take() {
                Some(stream) => {
                    *self = SerdeStream::Done(stream.serializer.end().map_err(|_| stream::Error)?);
                    Ok(())
                }
                None => Err(stream::Error),
            },
            // Complete a struct
            SerdeStream::SerializeStruct(stream) => match stream.take() {
                Some(stream) => {
                    *self = SerdeStream::Done(stream.serializer.end().map_err(|_| stream::Error)?);
                    Ok(())
                }
                None => Err(stream::Error),
            },
            // Complete a struct variant
            SerdeStream::SerializeStructVariant(stream) => match stream.take() {
                Some(stream) => {
                    *self = SerdeStream::Done(stream.serializer.end().map_err(|_| stream::Error)?);
                    Ok(())
                }
                None => Err(stream::Error),
            },
            _ => Err(stream::Error),
        }
    }
}

impl<'a, S: Serializer> Stream<'a> for SerdeStream<S> {
    fn any<'b: 'a, V: stream::ValueRef<'b>>(&mut self, v: V) -> stream::Result {
        self.serialize_any(Value::new(v))
    }

    fn i64(&mut self, v: i64) -> stream::Result {
        self.serialize_any(v)
    }

    fn u64(&mut self, v: u64) -> stream::Result {
        self.serialize_any(v)
    }

    fn i128(&mut self, v: i128) -> stream::Result {
        self.serialize_any(v)
    }

    fn u128(&mut self, v: u128) -> stream::Result {
        self.serialize_any(v)
    }

    fn f64(&mut self, v: f64) -> stream::Result {
        self.serialize_any(v)
    }

    fn bool(&mut self, v: bool) -> stream::Result {
        self.serialize_any(v)
    }

    fn none(&mut self) -> stream::Result {
        self.serialize_any(None::<()>)
    }

    fn str<'s: 'a, T: stream::TypedRef<'s, str>>(&mut self, v: T) -> stream::Result {
        self.serialize_any(v.get())
    }

    fn type_tagged_begin<T: stream::TypedRef<'static, str>>(
        &mut self,
        tag: stream::TypeTag<T>,
    ) -> stream::Result {
        self.serializer()?.type_tag = tag.ty().try_unwrap();

        Ok(())
    }

    fn type_tagged_end(&mut self) -> stream::Result {
        Ok(())
    }

    fn variant_tagged_begin<
        T: stream::TypedRef<'static, str>,
        K: stream::TypedRef<'static, str>,
    >(
        &mut self,
        tag: stream::VariantTag<T, K>,
    ) -> stream::Result {
        let serializer = self.serializer()?;

        serializer.type_tag = tag.ty().try_unwrap();
        serializer.variant_tag = tag.variant_key().try_unwrap();
        serializer.variant_index = tag.variant_index().and_then(|index| index.try_into().ok());

        Ok(())
    }

    fn variant_tagged_end(&mut self) -> stream::Result {
        Ok(())
    }

    fn map_begin(&mut self, len: Option<usize>) -> stream::Result {
        self.serialize_map(len)
    }

    fn map_end(&mut self) -> stream::Result {
        self.serialize_map_end()
    }

    fn map_field<T: stream::TypedRef<'static, str>>(&mut self, field: T) -> stream::Result {
        self.serialize_map_field(field.try_unwrap().ok_or(field.get()))
    }
}
