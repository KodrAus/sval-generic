use serde::ser::{Serialize, SerializeMap, SerializeSeq, Serializer};

use crate::stream::{self, Stream};

struct SerdeStream<S: Serializer>(Option<Serde<S>>);

enum Serde<S: Serializer> {
    Serializer(S),
    SerializeMap(S::SerializeMap),
    SerializeSeq(S::SerializeSeq),
    Ok(S::Ok),
}

impl<S> SerdeStream<S>
where
    S: Serializer,
{
    fn new(serializer: S) -> Self {
        SerdeStream(Some(Serde::Serializer(serializer)))
    }

    fn value(&mut self, v: impl Serialize) {
        let r = v.serialize(self.serializer()).unwrap();
        self.0 = Some(Serde::Ok(r));
    }

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

    fn serialize_seq(&mut self) -> S::SerializeSeq {
        if let Some(Serde::SerializeSeq(s)) = self.0.take() {
            return s;
        }

        panic!("invalid serializer")
    }

    fn ok(self) -> S::Ok {
        if let Some(Serde::Ok(r)) = self.0 {
            return r;
        }

        panic!("invalid serializer")
    }
}

struct SerdeValue<V>(V);

impl<'a, V> Serialize for SerdeValue<V>
where
    V: stream::UnknownValueRef<'a>,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut stream = SerdeStream::new(serializer);

        self.0.stream(&mut stream).unwrap();
        Ok(stream.ok())
    }
}

impl<'a, S> Stream<'a> for SerdeStream<S>
where
    S: Serializer,
{
    fn u64(&mut self, v: u64) -> stream::Result {
        self.value(v);
        Ok(())
    }

    fn i64(&mut self, v: i64) -> stream::Result {
        self.value(v);
        Ok(())
    }

    fn u128(&mut self, v: u128) -> stream::Result {
        self.value(v);
        Ok(())
    }

    fn i128(&mut self, v: i128) -> stream::Result {
        self.value(v);
        Ok(())
    }

    fn f64(&mut self, v: f64) -> stream::Result {
        self.value(v);
        Ok(())
    }

    fn bool(&mut self, v: bool) -> stream::Result {
        self.value(v);
        Ok(())
    }

    fn none(&mut self) -> stream::Result {
        self.value(Option::<()>::None);
        Ok(())
    }

    fn str<'v, V: stream::TypedValueRef<'v, str>>(&mut self, v: V) -> stream::Result
    where
        'v: 'a,
    {
        self.value(v.get());
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

    fn map_key<'k, K: stream::UnknownValueRef<'k>>(&mut self, k: K) -> stream::Result
    where
        'k: 'a,
    {
        self.serialize_map().serialize_key(&SerdeValue(k)).unwrap();
        Ok(())
    }

    fn map_value<'v, V: stream::UnknownValueRef<'v>>(&mut self, v: V) -> stream::Result
    where
        'v: 'a,
    {
        self.serialize_map()
            .serialize_value(&SerdeValue(v))
            .unwrap();
        Ok(())
    }

    fn map_entry<'k, 'v, K: stream::UnknownValueRef<'k>, V: stream::UnknownValueRef<'v>>(
        &mut self,
        k: K,
        v: V,
    ) -> stream::Result
    where
        'k: 'a,
        'v: 'a,
    {
        self.serialize_map()
            .serialize_entry(&SerdeValue(k), &SerdeValue(v))
            .unwrap();
        Ok(())
    }

    fn map_field<'v, F: stream::TypedValueRef<'static, str>, V: stream::UnknownValueRef<'v>>(
        &mut self,
        f: F,
        v: V,
    ) -> stream::Result
    where
        'v: 'a,
    {
        self.serialize_map()
            .serialize_entry(&SerdeValue(f), &SerdeValue(v))
            .unwrap();
        Ok(())
    }

    fn seq_begin(&mut self, len: Option<usize>) -> stream::Result {
        self.0 = Some(Serde::SerializeSeq(
            self.serializer().serialize_seq(len).unwrap(),
        ));
        Ok(())
    }

    fn seq_elem_begin(&mut self) -> stream::Result {
        Ok(())
    }

    fn seq_end(&mut self) -> stream::Result {
        self.serialize_seq().end().unwrap();
        Ok(())
    }

    fn seq_elem<'e, E: stream::UnknownValueRef<'e>>(&mut self, e: E) -> stream::Result
    where
        'e: 'a,
    {
        self.serialize_seq()
            .serialize_element(&SerdeValue(e))
            .unwrap();
        Ok(())
    }
}
