use std::vec::Vec;

use crate::{BinaryBuf, TextBuf};

pub struct ValueBuf<'sval> {
    parts: Vec<ValuePart<'sval>>,
}

struct ValuePart<'sval> {
    next_idx: Option<usize>,
    kind: ValueKind<'sval>,
}

enum ValueKind<'sval> {
    Map { num_entries_hint: Option<usize> },
    MapKey,
    MapValue,
    Text(TextBuf<'sval>),
    Binary(BinaryBuf<'sval>),
}
