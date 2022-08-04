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

Ids used on enum variants must be unique within that enum.
Ids used on values outside of enums must be unique among all values.
*/
#[derive(Clone, Copy, Debug)]
pub struct Id {
    value: [u8; 16],
}

impl Id {
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

impl PartialEq for Id {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for Id {}

impl Hash for Id {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state)
    }
}

/**
A tag annotates a data type with an informational label and canonical id.

Data types with the same structure are not considered equal if they have different tag ids.
*/
#[derive(Clone, Debug)]
pub enum Tag<'computed> {
    /**
    The type of the tagged value depends on its structure.

    The tag carries an optional informational label.
    This label isn't considered canonical, different types may have the same label.
    */
    Structural(Option<Label<'computed>>),
    /**
    The type of the tagged value depends on its structure and its id.

    The tag carries an optional informational label.
    The id carries a canonical identifier that separates the type of the tagged value from
    others that don't share the same id.
    */
    Identified(Id, Option<Label<'computed>>),
}

/**
Equality for tags is based purely on their ids. Labels are informational.
*/
impl<'computed> PartialEq for Tag<'computed> {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

impl<'computed> Eq for Tag<'computed> {}

/**
Tags are hashed based purely on their ids. Labels are informational.
*/
impl<'computed> Hash for Tag<'computed> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id().hash(state)
    }
}

impl<'computed> Tag<'computed> {
    pub fn id(&self) -> Option<&Id> {
        match self {
            Tag::Structural(_) => None,
            Tag::Identified(id, _) => Some(id),
        }
    }

    pub fn label(&self) -> Option<&Label<'computed>> {
        match self {
            Tag::Structural(label) => label.as_ref(),
            Tag::Identified(_, label) => label.as_ref(),
        }
    }

    pub fn split(self) -> (Option<Id>, Option<Label<'computed>>) {
        match self {
            Tag::Structural(label) => (None, label),
            Tag::Identified(id, label) => (Some(id), label),
        }
    }
}

impl Value for () {
    fn stream<'sval, S: Stream<'sval> + ?Sized>(&'sval self, stream: &mut S) -> Result {
        stream.unit()
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
