pub(crate) mod map;
pub(crate) mod number;
pub(crate) mod optional;
pub(crate) mod seq;
pub(crate) mod text;

use crate::{Stream, Value};

impl Value for () {
    fn stream<'sval, S: Stream<'sval>>(&'sval self, mut stream: S) -> crate::Result {
        stream.unit()
    }

    fn is_dynamic(&self) -> bool {
        false
    }
}

impl Value for bool {
    fn stream<'sval, S: Stream<'sval>>(&'sval self, mut stream: S) -> crate::Result {
        stream.bool(*self)
    }

    fn is_dynamic(&self) -> bool {
        false
    }

    fn to_bool(&self) -> Option<bool> {
        Some(*self)
    }
}

#[cfg(not(test))]
pub(crate) fn bool_basic<'sval>(v: bool, stream: impl Stream<'sval>) -> crate::Result {
    if stream.is_text_based() {
        if v { "true" } else { "false" }.stream(stream)
    } else {
        if v { &1u8 } else { &0u8 }.stream(stream)
    }
}
