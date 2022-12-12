pub fn serialize<V: sval::Value>(value: V) -> Serialize<V> {
    Serialize(value)
}

pub struct Serialize<V>(V);

impl<V: sval::Value> serde::Serialize for Serialize<V> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut stream = Serializer::Value(serializer);

        let _ = self.0.stream(&mut stream);

        stream.finish()
    }
}

enum Serializer<S: serde::Serializer> {
    Value(S),
    Done(Result<S::Ok, S::Error>),
}

impl<S: serde::Serializer> Serializer<S> {
    fn finish(self) -> Result<S::Ok, S::Error> {
        todo!()
    }
}

impl<'sval, S: serde::Serializer> sval::Stream<'sval> for Serializer<S> {
    fn value<V: sval::Value + ?Sized>(&mut self, v: &'sval V) -> sval::Result {
        todo!()
    }

    fn value_computed<V: sval::Value + ?Sized>(&mut self, v: &V) -> sval::Result {
        todo!()
    }

    fn null(&mut self) -> sval::Result {
        todo!()
    }

    fn bool(&mut self, value: bool) -> sval::Result {
        todo!()
    }

    fn text_begin(&mut self, num_bytes_hint: Option<usize>) -> sval::Result {
        todo!()
    }

    fn text_fragment(&mut self, fragment: &'sval str) -> sval::Result {
        todo!()
    }

    fn text_fragment_computed(&mut self, fragment: &str) -> sval::Result {
        todo!()
    }

    fn text_end(&mut self) -> sval::Result {
        todo!()
    }

    fn binary_begin(&mut self, num_bytes_hint: Option<usize>) -> sval::Result {
        todo!()
    }

    fn binary_fragment(&mut self, fragment: &'sval [u8]) -> sval::Result {
        todo!()
    }

    fn binary_fragment_computed(&mut self, fragment: &[u8]) -> sval::Result {
        todo!()
    }

    fn binary_end(&mut self) -> sval::Result {
        todo!()
    }

    fn u8(&mut self, value: u8) -> sval::Result {
        todo!()
    }

    fn u16(&mut self, value: u16) -> sval::Result {
        todo!()
    }

    fn u32(&mut self, value: u32) -> sval::Result {
        todo!()
    }

    fn u64(&mut self, value: u64) -> sval::Result {
        todo!()
    }

    fn u128(&mut self, value: u128) -> sval::Result {
        todo!()
    }

    fn i8(&mut self, value: i8) -> sval::Result {
        todo!()
    }

    fn i16(&mut self, value: i16) -> sval::Result {
        todo!()
    }

    fn i32(&mut self, value: i32) -> sval::Result {
        todo!()
    }

    fn i64(&mut self, value: i64) -> sval::Result {
        todo!()
    }

    fn i128(&mut self, value: i128) -> sval::Result {
        todo!()
    }

    fn f32(&mut self, value: f32) -> sval::Result {
        todo!()
    }

    fn f64(&mut self, value: f64) -> sval::Result {
        todo!()
    }

    fn map_begin(&mut self, num_entries_hint: Option<usize>) -> sval::Result {
        todo!()
    }

    fn map_key_begin(&mut self) -> sval::Result {
        todo!()
    }

    fn map_key_end(&mut self) -> sval::Result {
        todo!()
    }

    fn map_value_begin(&mut self) -> sval::Result {
        todo!()
    }

    fn map_value_end(&mut self) -> sval::Result {
        todo!()
    }

    fn map_end(&mut self) -> sval::Result {
        todo!()
    }

    fn seq_begin(&mut self, num_entries_hint: Option<usize>) -> sval::Result {
        todo!()
    }

    fn seq_value_begin(&mut self) -> sval::Result {
        todo!()
    }

    fn seq_value_end(&mut self) -> sval::Result {
        todo!()
    }

    fn seq_end(&mut self) -> sval::Result {
        todo!()
    }

    fn dynamic_begin(&mut self) -> sval::Result {
        todo!()
    }

    fn dynamic_end(&mut self) -> sval::Result {
        todo!()
    }

    fn enum_begin(
        &mut self,
        tag: Option<sval::Tag>,
        label: Option<sval::Label>,
        index: Option<sval::Index>,
    ) -> sval::Result {
        todo!()
    }

    fn enum_end(
        &mut self,
        tag: Option<sval::Tag>,
        label: Option<sval::Label>,
        index: Option<sval::Index>,
    ) -> sval::Result {
        todo!()
    }

    fn tagged_begin(
        &mut self,
        tag: Option<sval::Tag>,
        label: Option<sval::Label>,
        index: Option<sval::Index>,
    ) -> sval::Result {
        todo!()
    }

    fn tagged_end(
        &mut self,
        tag: Option<sval::Tag>,
        label: Option<sval::Label>,
        index: Option<sval::Index>,
    ) -> sval::Result {
        todo!()
    }

    fn tag(
        &mut self,
        tag: Option<sval::Tag>,
        label: Option<sval::Label>,
        index: Option<sval::Index>,
    ) -> sval::Result {
        todo!()
    }

    fn record_begin(
        &mut self,
        tag: Option<sval::Tag>,
        label: Option<sval::Label>,
        index: Option<sval::Index>,
        num_entries: Option<usize>,
    ) -> sval::Result {
        todo!()
    }

    fn record_value_begin(&mut self, label: sval::Label) -> sval::Result {
        todo!()
    }

    fn record_value_end(&mut self, label: sval::Label) -> sval::Result {
        todo!()
    }

    fn record_end(
        &mut self,
        tag: Option<sval::Tag>,
        label: Option<sval::Label>,
        index: Option<sval::Index>,
    ) -> sval::Result {
        todo!()
    }

    fn tuple_begin(
        &mut self,
        tag: Option<sval::Tag>,
        label: Option<sval::Label>,
        index: Option<sval::Index>,
        num_entries: Option<usize>,
    ) -> sval::Result {
        todo!()
    }

    fn tuple_value_begin(&mut self, index: sval::Index) -> sval::Result {
        todo!()
    }

    fn tuple_value_end(&mut self, index: sval::Index) -> sval::Result {
        todo!()
    }

    fn tuple_end(
        &mut self,
        tag: Option<sval::Tag>,
        label: Option<sval::Label>,
        index: Option<sval::Index>,
    ) -> sval::Result {
        todo!()
    }
}
