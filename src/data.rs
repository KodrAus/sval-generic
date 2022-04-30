pub(crate) mod map;
pub(crate) mod number;
pub(crate) mod optional;
pub(crate) mod seq;
pub(crate) mod text;

use crate::{std::ops::Deref, Result, Stream, Value};

#[derive(Clone, Copy)]
pub struct Label<'a> {
    computed: &'a str,
    value: Option<&'static str>,
}

impl<'a> Label<'a> {
    pub fn new(label: &'static str) -> Self {
        Label {
            computed: label,
            value: Some(label),
        }
    }

    pub fn computed(label: &'a str) -> Self {
        Label {
            computed: label,
            value: None,
        }
    }

    pub fn get(&self) -> &'a str {
        self.computed
    }

    pub fn get_static(&self) -> Option<&'static str> {
        self.value
    }
}

impl<'a> Deref for Label<'a> {
    type Target = str;

    fn deref(&self) -> &str {
        self.get()
    }
}

impl Value for () {
    fn stream<'sval, S: Stream<'sval>>(&'sval self, mut stream: S) -> Result {
        stream.unit()
    }

    fn is_dynamic(&self) -> bool {
        false
    }
}

impl Value for bool {
    fn stream<'sval, S: Stream<'sval>>(&'sval self, mut stream: S) -> Result {
        stream.bool(*self)
    }

    fn is_dynamic(&self) -> bool {
        false
    }

    fn to_bool(&self) -> Option<bool> {
        Some(*self)
    }
}

pub(crate) fn bool_basic<'sval>(v: bool, stream: impl Stream<'sval>) -> Result {
    if stream.is_text_based() {
        if v { "true" } else { "false" }.stream(stream)
    } else {
        if v { &1u8 } else { &0u8 }.stream(stream)
    }
}
