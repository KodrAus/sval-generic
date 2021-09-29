use crate::stream::{self, Stream};

use serde::ser::{self, Error as _, Serialize, SerializeMap, Serializer};

struct SerdeValueRef<V>(V);

impl<'a, V: stream::ValueRef<'a>> Serialize for SerdeValueRef<V> {
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
    Done(S::Ok),
}

#[derive(Default)]
struct Context {
    type_tag: Option<&'static str>,
    variant_tag: Option<&'static str>,
    variant_index: Option<u32>,
    field: Option<&'static str>,
}

struct StreamSerializer<S: Serializer> {
    serializer: S,
}

struct StreamSerializeMap<S: Serializer> {
    serializer: S::SerializeMap,
    is_key: bool,
}

impl<S: Serializer> SerdeStream<S> {
    fn begin(serializer: S) -> Self {
        SerdeStream::Serializer(Some(StreamSerializer { serializer }))
    }

    fn end(self) -> Result<S::Ok, S::Error> {
        if let SerdeStream::Done(ok) = self {
            Ok(ok)
        } else {
            Err(S::Error::custom("the stream is incomplete"))
        }
    }

    fn serialize_any(&mut self, v: impl Serialize) -> stream::Result {
        match self {
            SerdeStream::Serializer(stream) => match stream.take() {
                Some(stream) => {
                    *self = SerdeStream::Done(
                        v.serialize(stream.serializer).map_err(|_| stream::Error)?,
                    );
                    Ok(())
                }
                None => Err(stream::Error),
            },
            SerdeStream::SerializeMap(Some(ref mut stream)) => {
                if stream.is_key {
                    stream
                        .serializer
                        .serialize_key(&v)
                        .map_err(|_| stream::Error)?;
                    Ok(())
                } else {
                    stream
                        .serializer
                        .serialize_value(&v)
                        .map_err(|_| stream::Error)?;
                    Ok(())
                }
            }
            SerdeStream::SerializeMap(None) => Err(stream::Error),
            SerdeStream::Done(_) => Err(stream::Error),
        }
    }
}

impl<'a, S: Serializer> Stream<'a> for SerdeStream<S> {
    fn any<'b: 'a, V: stream::ValueRef<'b>>(&mut self, v: V) -> stream::Result {
        self.serialize_any(SerdeValueRef(v))
    }
}
