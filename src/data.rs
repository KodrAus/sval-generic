pub(crate) mod map;
pub(crate) mod number;
pub(crate) mod optional;
pub(crate) mod seq;
pub(crate) mod text;

use crate::{
    std::{
        borrow::Borrow,
        fmt,
        hash::{Hash, Hasher},
        ops::Deref,
    },
    Result, Stream, Value,
};

/**
A textual label for some value.
*/
#[derive(Clone)]
pub struct Label<'computed> {
    value_computed: &'computed str,
    value_static: Option<&'static str>,
}

impl<'computed> Label<'computed> {
    /**
    Create a new label from a static static value.

    For labels that can't satisfy the `'static` lifetime, use [`Label::computed`].
    */
    pub const fn new(label: &'static str) -> Self {
        Label {
            value_computed: label,
            value_static: Some(label),
        }
    }

    /**
    Create a new label from a string value.
    */
    pub const fn computed(label: &'computed str) -> Self {
        Label {
            value_computed: label,
            value_static: None,
        }
    }

    /**
    Get the value of the label as a string.
    */
    pub const fn get(&self) -> &'computed str {
        self.value_computed
    }

    /**
    Try get the value of the label as a static string.

    For labels that were created over computed data this method will return `None`.
    */
    pub const fn try_get_static(&self) -> Option<&'static str> {
        self.value_static
    }
}

impl<'a> Deref for Label<'a> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.value_computed
    }
}

impl<'a, 'b> PartialEq<Label<'b>> for Label<'a> {
    fn eq(&self, other: &Label<'b>) -> bool {
        self.value_computed == other.value_computed
    }
}

impl<'a> Eq for Label<'a> {}

impl<'a> Hash for Label<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value_computed.hash(state)
    }
}

impl<'a> Borrow<str> for Label<'a> {
    fn borrow(&self) -> &str {
        self.value_computed
    }
}

impl<'a> fmt::Debug for Label<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.value_computed.fmt(f)
    }
}

/**
A canonical identifier for some value.

Ids are unique, so they can be used to tell whether a particular value has a particular type
regardless of the context it's seen in.
*/
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Id {
    value: [u8; 16],
}

impl Id {
    pub const OPTION: Self = Id::new(1u128.to_le_bytes());
    pub const OPTION_SOME: Self = Id::new(2u128.to_le_bytes());
    pub const OPTION_NONE: Self = Id::new(3u128.to_le_bytes());

    /**
    Create an id.
    */
    pub const fn new(id: [u8; 16]) -> Self {
        Id { value: id }
    }

    /**
    Get the id as a 16 byte value.
    */
    pub const fn get(&self) -> [u8; 16] {
        self.value
    }
}

/**
A binary index for some value in its parent context.
*/
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Index(u32);

impl Index {
    pub const fn new(index: u32) -> Self {
        Index(index)
    }

    pub const fn get(&self) -> u32 {
        self.0
    }
}

impl Value for () {
    fn stream<'sval, S: Stream<'sval> + ?Sized>(&'sval self, stream: &mut S) -> Result {
        stream.null()
    }

    fn is_dynamic(&self) -> bool {
        false
    }
}

impl Value for bool {
    fn stream<'sval, S: Stream<'sval> + ?Sized>(&'sval self, stream: &mut S) -> Result {
        stream.bool(*self)
    }

    fn is_dynamic(&self) -> bool {
        false
    }

    fn to_bool(&self) -> Option<bool> {
        Some(*self)
    }
}

pub(crate) fn bool_basic<'sval>(v: bool, stream: &mut (impl Stream<'sval> + ?Sized)) -> Result {
    if v { "true" } else { "false" }.stream(stream)
}
