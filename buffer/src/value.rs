use crate::{
    std::{mem, ops::Range, vec::Vec},
    BinaryBuf, TextBuf,
};

#[derive(Debug, PartialEq)]
pub struct ValueBuf<'sval> {
    parts: Vec<ValuePart<'sval>>,
    stack: Vec<usize>,
}

#[repr(transparent)]
struct ValueSlice<'sval>([ValuePart<'sval>]);

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
    Map {
        len: usize,
        num_entries_hint: Option<usize>,
    },
    MapKey {
        len: usize,
    },
    MapValue {
        len: usize,
    },
    Seq {
        len: usize,
        num_entries_hint: Option<usize>,
    },
    SeqValue {
        len: usize,
    },
}

impl<'sval> ValueBuf<'sval> {
    pub fn new() -> Self {
        ValueBuf {
            parts: Vec::new(),
            stack: Vec::new(),
        }
    }

    fn slice<'a>(&'a self) -> &'a ValueSlice<'sval> {
        unsafe { mem::transmute::<&'a [ValuePart<'sval>], &'a ValueSlice<'sval>>(&self.parts) }
    }

    fn push_kind(&mut self, kind: ValueKind<'sval>) {
        self.parts.push(ValuePart { kind });
    }

    fn push_begin(&mut self, kind: ValueKind<'sval>) {
        self.stack.push(self.parts.len());
        self.parts.push(ValuePart { kind });
    }

    fn push_end(&mut self) {
        let index = self.stack.pop().expect("missing stack frame");

        let len = self.parts.len() - index - 1;

        *match &mut self.parts[index].kind {
            ValueKind::Map { len, .. } => len,
            ValueKind::MapKey { len } => len,
            ValueKind::MapValue { len } => len,
            ValueKind::Seq { len, .. } => len,
            ValueKind::SeqValue { len } => len,
            ValueKind::Null
            | ValueKind::Bool(_)
            | ValueKind::U8(_)
            | ValueKind::U16(_)
            | ValueKind::U32(_)
            | ValueKind::U64(_)
            | ValueKind::U128(_)
            | ValueKind::I8(_)
            | ValueKind::I16(_)
            | ValueKind::I32(_)
            | ValueKind::I64(_)
            | ValueKind::I128(_)
            | ValueKind::F32(_)
            | ValueKind::F64(_)
            | ValueKind::Text(_)
            | ValueKind::Binary(_) => panic!("can't end at this index"),
        } = len;
    }

    fn current_mut(&mut self) -> &mut ValuePart<'sval> {
        self.parts.last_mut().expect("missing current")
    }
}

impl<'sval> ValueSlice<'sval> {
    fn slice<'a>(&'a self, range: Range<usize>) -> &'a ValueSlice<'sval> {
        match self.0.get(range.clone()) {
            Some(_) => (),
            None => {
                panic!("{:?} is out of range for {:?}", range, &self.0);
            }
        }

        unsafe { mem::transmute::<&'a [ValuePart<'sval>], &'a ValueSlice<'sval>>(&self.0[range]) }
    }
}

impl<'a> sval::Value for ValueBuf<'a> {
    fn stream<'sval, S: sval::Stream<'sval> + ?Sized>(&'sval self, stream: &mut S) -> sval::Result {
        self.slice().stream(stream)
    }
}

impl<'a> sval::Value for ValueSlice<'a> {
    fn stream<'sval, S: sval::Stream<'sval> + ?Sized>(&'sval self, stream: &mut S) -> sval::Result {
        let mut i = 0;

        while let Some(part) = self.0.get(i) {
            match &part.kind {
                ValueKind::Null => stream.null()?,
                ValueKind::Bool(v) => v.stream(stream)?,
                ValueKind::U8(v) => v.stream(stream)?,
                ValueKind::U16(v) => v.stream(stream)?,
                ValueKind::U32(v) => v.stream(stream)?,
                ValueKind::U64(v) => v.stream(stream)?,
                ValueKind::U128(v) => v.stream(stream)?,
                ValueKind::I8(v) => v.stream(stream)?,
                ValueKind::I16(v) => v.stream(stream)?,
                ValueKind::I32(v) => v.stream(stream)?,
                ValueKind::I64(v) => v.stream(stream)?,
                ValueKind::I128(v) => v.stream(stream)?,
                ValueKind::F32(v) => v.stream(stream)?,
                ValueKind::F64(v) => v.stream(stream)?,
                ValueKind::Text(v) => v.stream(stream)?,
                ValueKind::Binary(v) => v.stream(stream)?,
                ValueKind::Map {
                    len,
                    num_entries_hint,
                } => {
                    stream.map_begin(*num_entries_hint)?;

                    let start = i + 1;
                    let end = start + *len;

                    self.slice(start..end).stream(stream)?;

                    stream.map_end()?;

                    i += *len;
                }
                ValueKind::MapKey { len } => {
                    stream.map_key_begin()?;

                    let start = i + 1;
                    let end = start + *len;

                    stream.value(self.slice(start..end))?;

                    stream.map_key_end()?;

                    i += *len;
                }
                ValueKind::MapValue { len } => {
                    stream.map_value_begin()?;

                    let start = i + 1;
                    let end = start + *len;

                    stream.value(self.slice(start..end))?;

                    stream.map_value_end()?;

                    i += *len;
                }
                ValueKind::Seq {
                    len,
                    num_entries_hint,
                } => {
                    stream.seq_begin(*num_entries_hint)?;

                    let start = i + 1;
                    let end = start + *len;

                    self.slice(start..end).stream(stream)?;

                    stream.seq_end()?;

                    i += *len;
                }
                ValueKind::SeqValue { len } => {
                    stream.seq_value_begin()?;

                    let start = i + 1;
                    let end = start + *len;

                    stream.value(self.slice(start..end))?;

                    stream.seq_value_end()?;

                    i += *len;
                }
            }

            i += 1;
        }

        Ok(())
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
        self.push_begin(ValueKind::Map {
            len: 0,
            num_entries_hint,
        });

        Ok(())
    }

    fn map_key_begin(&mut self) -> sval::Result {
        self.push_begin(ValueKind::MapKey { len: 0 });

        Ok(())
    }

    fn map_key_end(&mut self) -> sval::Result {
        self.push_end();

        Ok(())
    }

    fn map_value_begin(&mut self) -> sval::Result {
        self.push_begin(ValueKind::MapValue { len: 0 });

        Ok(())
    }

    fn map_value_end(&mut self) -> sval::Result {
        self.push_end();

        Ok(())
    }

    fn map_end(&mut self) -> sval::Result {
        self.push_end();

        Ok(())
    }

    fn seq_begin(&mut self, num_entries_hint: Option<usize>) -> sval::Result {
        self.push_begin(ValueKind::Seq {
            len: 0,
            num_entries_hint,
        });

        Ok(())
    }

    fn seq_value_begin(&mut self) -> sval::Result {
        self.push_begin(ValueKind::SeqValue { len: 0 });

        Ok(())
    }

    fn seq_value_end(&mut self) -> sval::Result {
        self.push_end();

        Ok(())
    }

    fn seq_end(&mut self) -> sval::Result {
        self.push_end();

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
    use super::*;
    use crate::std::vec;

    use sval::Stream as _;

    #[test]
    fn buffer_primitive() {
        let mut value = ValueBuf::new();

        value.i32(42).unwrap();

        let expected = vec![ValuePart {
            kind: ValueKind::I32(42),
        }];

        assert_eq!(expected, value.parts);
    }

    #[test]
    fn buffer_roundtrip() {
        let value = vec![
            vec![],
            vec![vec![1, 2, 3], vec![4]],
            vec![vec![5, 6], vec![7, 8, 9]],
        ];

        let mut value_1 = ValueBuf::new();

        value_1.value(&value).unwrap();

        let mut value_2 = ValueBuf::new();

        value_2.value(&value_1).unwrap();

        assert_eq!(value_1, value_2);
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
            ValuePart {
                kind: ValueKind::Map {
                    len: 8,
                    num_entries_hint: Some(2),
                },
            },
            ValuePart {
                kind: ValueKind::MapKey { len: 1 },
            },
            ValuePart {
                kind: ValueKind::I32(0),
            },
            ValuePart {
                kind: ValueKind::MapValue { len: 1 },
            },
            ValuePart {
                kind: ValueKind::Bool(false),
            },
            ValuePart {
                kind: ValueKind::MapKey { len: 1 },
            },
            ValuePart {
                kind: ValueKind::I32(1),
            },
            ValuePart {
                kind: ValueKind::MapValue { len: 1 },
            },
            ValuePart {
                kind: ValueKind::Bool(true),
            },
        ];

        assert_eq!(expected, value.parts);
    }
}
