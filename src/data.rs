pub(crate) mod map;
pub(crate) mod number;
pub(crate) mod optional;
pub(crate) mod seq;
pub(crate) mod text;

use crate::{Result, Stream, Value};

/**
An informational label for some value.
*/
#[derive(Clone, Copy)]
pub struct Label<'a> {
    computed: &'a str,
    value: Option<&'static str>,
}

impl<'a> Label<'a> {
    pub const fn new(label: &'static str) -> Self {
        Label {
            computed: label,
            value: Some(label),
        }
    }

    pub const fn computed(label: &'a str) -> Self {
        Label {
            computed: label,
            value: None,
        }
    }

    pub const fn get(&self) -> &'a str {
        self.computed
    }

    pub const fn try_get_static(&self) -> Option<&'static str> {
        self.value
    }
}

/**
A canonical identifier for some value.
*/
#[derive(Clone, Copy)]
pub struct Id(u128);

impl Id {
    pub const fn new(id: u128) -> Self {
        Id(id)
    }

    pub const fn get(&self) -> u128 {
        self.0
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
