use serde::ser::{Serialize, SerializeMap, Serializer};

use crate::stream::{self, Stream};

struct SerdeStream<S: Serializer>(Option<Serde<S>>);

enum Serde<S: Serializer> {
    Serializer(S),
    SerializeMap(S::SerializeMap),
}

impl<S> SerdeStream<S>
where
    S: Serializer,
{
    fn serializer(&mut self) -> S {
        if let Some(Serde::Serializer(s)) = self.0.take() {
            return s;
        }

        panic!("invalid serializer")
    }

    fn serialize_map(&mut self) -> S::SerializeMap {
        if let Some(Serde::SerializeMap(s)) = self.0.take() {
            return s;
        }

        panic!("invalid serializer")
    }
}

impl<'a, S> Stream<'a> for SerdeStream<S>
where
    S: Serializer,
{
    fn u128(&mut self, v: u128) -> stream::Result {
        self.serializer().serialize_u128(v).unwrap();
        Ok(())
    }

    fn i128(&mut self, v: i128) -> stream::Result {
        self.serializer().serialize_i128(v).unwrap();
        Ok(())
    }

    fn str<V: stream::TypedValue<'a, str>>(&mut self, v: V) -> stream::Result {
        self.serializer().serialize_str(&*v).unwrap();
        Ok(())
    }

    fn map_begin(&mut self, len: Option<usize>) -> stream::Result {
        self.0 = Some(Serde::SerializeMap(
            self.serializer().serialize_map(len).unwrap(),
        ));
        Ok(())
    }

    fn map_key_begin(&mut self) -> stream::Result {
        Ok(())
    }

    fn map_value_begin(&mut self) -> stream::Result {
        Ok(())
    }

    fn map_end(&mut self) -> stream::Result {
        self.serialize_map().end().unwrap();
        Ok(())
    }

    fn map_key<K: stream::UntypedValue<'a>>(&mut self, k: K) -> stream::Result {
        Ok(())
    }

    fn map_value<V: stream::UntypedValue<'a>>(&mut self, v: V) -> stream::Result {
        Ok(())
    }

    fn map_entry<K: stream::UntypedValue<'a>, V: stream::UntypedValue<'a>>(
        &mut self,
        k: K,
        v: V,
    ) -> stream::Result {
        Ok(())
    }

    fn map_field<F: stream::TypedValue<'static, str>, V: stream::UntypedValue<'a>>(
        &mut self,
        f: F,
        v: V,
    ) -> stream::Result {
        Ok(())
    }
}
