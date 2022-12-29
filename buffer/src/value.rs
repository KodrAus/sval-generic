use crate::{
    std::{ops::Range, vec::Vec},
    BinaryBuf, TextBuf,
};

pub struct ValueBuf<'sval> {
    parts: Vec<ValuePart<'sval>>,
    stack: Vec<usize>,
}

#[derive(Debug, PartialEq)]
struct ValuePart<'sval> {
    kind: ValueKind<'sval>,
}

#[derive(Debug, PartialEq)]
enum ValueKind<'sval> {
    Null,
    Bool(bool),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    F32(f32),
    F64(f64),
    Text(TextBuf<'sval>),
    Binary(BinaryBuf<'sval>),
    MapBegin { range: Range<usize>, num_entries_hint: Option<usize> },
    MapEnd,
    MapKeyBegin { range: Range<usize> },
    MapKeyEnd,
    MapValueBegin { range: Range<usize> },
    MapValueEnd,
    SeqBegin { range: Range<usize>, num_entries_hint: Option<usize> },
    SeqEnd,
    SeqValueBegin { range: Range<usize> },
    SeqValueEnd,
    DynamicBegin,
    DynamicEnd,
}

impl<'sval> ValueBuf<'sval> {
    pub fn new() -> Self {
        ValueBuf {
            parts: Vec::new(),
            start: 0,
        }
    }

    fn push_kind(&mut self, kind: ValueKind<'sval>) {
        let range = self.parts.len()..self.parts.len() + 1;

        self.parts.push(ValuePart { kind, range });
    }

    fn push_begin(&mut self, kind: ValueKind<'sval>) {
        let start = self.parts.len();
        let end = start + 1;

        let range = start..end;

        self.parts.push(ValuePart { kind, range });
        self.start = start;
    }

    fn push_end(&mut self, kind: ValueKind<'sval>) {
        let start = self.start;
        let end = self.parts.len() + 1;

        let range = start..end;

        self.parts.push(ValuePart { kind, range });

        let begin = &mut self.parts[start];
        begin.range.end = end;

        self.start = self.parts[start.saturating_sub(1)].range.start;
    }

    fn current_mut(&mut self) -> &mut ValuePart<'sval> {
        self.parts.last_mut().expect("missing current")
    }
}

impl<'a> sval::Value for ValueBuf<'a> {
    fn stream<'sval, S: sval::Stream<'sval> + ?Sized>(&'sval self, stream: &mut S) -> sval::Result {
        todo!()
    }
}

impl<'sval> sval::Stream<'sval> for ValueBuf<'sval> {
    fn null(&mut self) -> sval::Result {
        self.push_kind(ValueKind::Null);

        Ok(())
    }

    fn bool(&mut self, value: bool) -> sval::Result {
        self.push_kind(ValueKind::Bool(value));

        Ok(())
    }

    fn text_begin(&mut self, _: Option<usize>) -> sval::Result {
        self.push_kind(ValueKind::Text(TextBuf::new()));

        Ok(())
    }

    fn text_fragment(&mut self, fragment: &'sval str) -> sval::Result {
        match self.current_mut().kind {
            ValueKind::Text(ref mut text) => text.push_fragment(fragment),
            _ => Err(sval::Error::unsupported()),
        }
    }

    fn text_fragment_computed(&mut self, fragment: &str) -> sval::Result {
        match self.current_mut().kind {
            ValueKind::Text(ref mut text) => text.push_fragment_computed(fragment),
            _ => Err(sval::Error::unsupported()),
        }
    }

    fn text_end(&mut self) -> sval::Result {
        Ok(())
    }

    fn binary_begin(&mut self, _: Option<usize>) -> sval::Result {
        self.push_kind(ValueKind::Binary(BinaryBuf::new()));

        Ok(())
    }

    fn binary_fragment(&mut self, fragment: &'sval [u8]) -> sval::Result {
        match self.current_mut().kind {
            ValueKind::Binary(ref mut binary) => binary.push_fragment(fragment),
            _ => Err(sval::Error::unsupported()),
        }
    }

    fn binary_fragment_computed(&mut self, fragment: &[u8]) -> sval::Result {
        match self.current_mut().kind {
            ValueKind::Binary(ref mut binary) => binary.push_fragment_computed(fragment),
            _ => Err(sval::Error::unsupported()),
        }
    }

    fn binary_end(&mut self) -> sval::Result {
        Ok(())
    }

    fn u8(&mut self, value: u8) -> sval::Result {
        self.push_kind(ValueKind::U8(value));

        Ok(())
    }

    fn u16(&mut self, value: u16) -> sval::Result {
        self.push_kind(ValueKind::U16(value));

        Ok(())
    }

    fn u32(&mut self, value: u32) -> sval::Result {
        self.push_kind(ValueKind::U32(value));

        Ok(())
    }

    fn u64(&mut self, value: u64) -> sval::Result {
        self.push_kind(ValueKind::U64(value));

        Ok(())
    }

    fn u128(&mut self, value: u128) -> sval::Result {
        self.push_kind(ValueKind::U128(value));

        Ok(())
    }

    fn i8(&mut self, value: i8) -> sval::Result {
        self.push_kind(ValueKind::I8(value));

        Ok(())
    }

    fn i16(&mut self, value: i16) -> sval::Result {
        self.push_kind(ValueKind::I16(value));

        Ok(())
    }

    fn i32(&mut self, value: i32) -> sval::Result {
        self.push_kind(ValueKind::I32(value));

        Ok(())
    }

    fn i64(&mut self, value: i64) -> sval::Result {
        self.push_kind(ValueKind::I64(value));

        Ok(())
    }

    fn i128(&mut self, value: i128) -> sval::Result {
        self.push_kind(ValueKind::I128(value));

        Ok(())
    }

    fn f32(&mut self, value: f32) -> sval::Result {
        self.push_kind(ValueKind::F32(value));

        Ok(())
    }

    fn f64(&mut self, value: f64) -> sval::Result {
        self.push_kind(ValueKind::F64(value));

        Ok(())
    }

    fn map_begin(&mut self, num_entries_hint: Option<usize>) -> sval::Result {
        self.push_begin(ValueKind::MapBegin { num_entries_hint });

        Ok(())
    }

    fn map_key_begin(&mut self) -> sval::Result {
        self.push_begin(ValueKind::MapKeyBegin);

        Ok(())
    }

    fn map_key_end(&mut self) -> sval::Result {
        self.push_end(ValueKind::MapKeyEnd);

        Ok(())
    }

    fn map_value_begin(&mut self) -> sval::Result {
        self.push_begin(ValueKind::MapValueBegin);

        Ok(())
    }

    fn map_value_end(&mut self) -> sval::Result {
        self.push_end(ValueKind::MapValueEnd);

        Ok(())
    }

    fn map_end(&mut self) -> sval::Result {
        self.push_end(ValueKind::MapEnd);

        Ok(())
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
        Ok(())
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

#[cfg(test)]
mod tests {
    use crate::std::vec;
    use super::*;

    use sval::Stream as _;

    #[test]
    fn buffer_primitive() {
        let mut value = ValueBuf::new();

        value.i32(42).unwrap();

        let expected = vec![
            ValuePart { kind: ValueKind::I32(42), range: 0..1 },
        ];

        assert_eq!(expected, value.parts);
    }

    #[test]
    fn buffer_map() {
        let mut value = ValueBuf::new();

        value.map_begin(Some(2)).unwrap();

        value.map_key_begin().unwrap();
        value.i32(0).unwrap();
        value.map_key_end().unwrap();

        value.map_value_begin().unwrap();
        value.bool(false).unwrap();
        value.map_value_end().unwrap();

        value.map_key_begin().unwrap();
        value.i32(1).unwrap();
        value.map_key_end().unwrap();

        value.map_value_begin().unwrap();
        value.bool(true).unwrap();
        value.map_value_end().unwrap();

        value.map_end().unwrap();

        let expected = vec![
            ValuePart { kind: ValueKind::I32(42), range: 0..1 },
        ];

        assert_eq!(expected, value.parts);
    }
}
