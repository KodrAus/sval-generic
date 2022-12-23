use crate::{std::vec::Vec, BinaryBuf, TextBuf};

pub struct ValueBuf<'sval> {
    parts: Vec<ValuePart<'sval>>,
    stack: Vec<StackEntry>,
}

struct ValuePart<'sval> {
    kind: ValueKind<'sval>,
}

struct StackEntry {
    start_idx: usize,
}

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
    Map { end_idx: Option<usize>, num_entries_hint: Option<usize> },
    MapKey { end_idx: Option<usize> },
    MapValue { end_idx: Option<usize> },
    Seq { end_idx: Option<usize>, num_entries_hint: Option<usize> },
    SeqValue { end_idx: Option<usize> },
    Dynamic,
}

impl<'sval> ValueBuf<'sval> {
    fn push_kind(&mut self, kind: ValueKind<'sval>) {
        self.parts.push(ValuePart { kind });
    }

    fn current_mut(&mut self) -> &mut ValuePart<'sval> {
        self.parts.last_mut().expect("missing current")
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
        self.stack.push(StackEntry { start_idx: self.parts.len() });
        self.push_kind(ValueKind::Map { end_idx: None, num_entries_hint });

        Ok(())
    }

    fn map_key_begin(&mut self) -> sval::Result {
        self.stack.push(StackEntry { start_idx: self.parts.len() });
        self.push_kind(ValueKind::MapKey { end_idx: None });

        Ok(())
    }

    fn map_key_end(&mut self) -> sval::Result {
        if let Some(StackEntry { start_idx }) = self.stack.pop() {
            if let ValueKind::MapKey { ref mut end_idx } = &mut self.parts[start_idx].kind {
                *end_idx = Some(self.parts.len());
            }
        }

        Ok(())
    }

    fn map_value_begin(&mut self) -> sval::Result {
        self.stack.push(StackEntry { start_idx: self.parts.len() });
        self.push_kind(ValueKind::MapValue { end_idx: None });

        Ok(())
    }

    fn map_value_end(&mut self) -> sval::Result {
        if let Some(StackEntry { start_idx }) = self.stack.pop() {
            if let ValueKind::MapValue { ref mut end_idx } = &mut self.parts[start_idx].kind {
                *end_idx = Some(self.parts.len());
            }
        }

        Ok(())
    }

    fn map_end(&mut self) -> sval::Result {
        Ok(())
    }

    fn seq_begin(&mut self, num_entries_hint: Option<usize>) -> sval::Result {
        self.push_kind(ValueKind::Seq { num_entries_hint });

        Ok(())
    }

    fn seq_value_begin(&mut self) -> sval::Result {
        self.push_kind(ValueKind::SeqValue);

        Ok(())
    }

    fn seq_value_end(&mut self) -> sval::Result {
        Ok(())
    }

    fn seq_end(&mut self) -> sval::Result {
        Ok(())
    }

    fn dynamic_begin(&mut self) -> sval::Result {
        self.push_kind(ValueKind::Dynamic);

        Ok(())
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
