pub(crate) mod map;
pub(crate) mod number;
pub(crate) mod optional;
pub(crate) mod seq;
pub(crate) mod text;

use crate::{
    std::{
        fmt,
        hash::{Hash, Hasher},
        ops::Deref,
    },
    Result, Stream, Value,
};

/**
A textual label for some value.
*/
#[derive(Clone, Copy)]
pub struct Label<'a> {
    computed: &'a str,
    value: Option<&'static str>,
}

impl<'a> Label<'a> {
    /**
    Create a new label from a static static value.

    For labels that can't satisfy the `'static` lifetime, use [`Label::computed`].
    */
    pub const fn new(label: &'static str) -> Self {
        Label {
            computed: label,
            value: Some(label),
        }
    }

    /**
    Create a new label from a string value.
    */
    pub const fn computed(label: &'a str) -> Self {
        Label {
            computed: label,
            value: None,
        }
    }

    /**
    Get the value of the label as a string.
    */
    pub const fn get(&self) -> &'a str {
        self.computed
    }

    /**
    Try get the value of the label as a static string.

    For labels that were created over computed data this method will return `None`.
    */
    pub const fn try_get_static(&self) -> Option<&'static str> {
        self.value
    }
}

impl<'a> Deref for Label<'a> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.computed
    }
}

impl<'a, 'b> PartialEq<Label<'b>> for Label<'a> {
    fn eq(&self, other: &Label<'b>) -> bool {
        self.computed == other.computed
    }
}

impl<'a> Eq for Label<'a> {}

impl<'a> Hash for Label<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.computed.hash(state)
    }
}

impl<'a> fmt::Debug for Label<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.computed.fmt(f)
    }
}

/**
A canonical identifier for some value.

Ids belong to some scope, which they must be unique within.
That scope may be either local (like the set of variants in an enum) or global (like the set of all values and variants).
*/
#[derive(Clone, Copy, Debug)]
pub struct Id {
    value: [u8; 16],
}

impl Id {
    /**
    Create an id for a local scope.
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
A tag annotates a data type with an informational label and id.

Data types with the same structure are not considered equal if they have different tag ids.
*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tag<'a> {
    /**
    The type of the tagged value depends on its structure.

    The tag carries an optional informational label.
    This label isn't considered canonical, different types may have the same label.
    */
    Structural(Option<Label<'a>>),
    /**
    The type of the tagged value depends on its structure and its id.

    The tag carries an optional informational label.
    The id carries a canonical identifier that separates the type of the tagged value from
    others that don't share the same id.
    */
    Identified(Id, Option<Label<'a>>),
}

impl<'a> Tag<'a> {
    pub fn id(&self) -> Option<Id> {
        match self {
            Tag::Structural(_) => None,
            Tag::Identified(id, _) => Some(*id),
        }
    }

    pub fn label(&self) -> Option<Label> {
        match self {
            Tag::Structural(label) => *label,
            Tag::Identified(_, label) => *label,
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
    if stream.is_text_based() {
        if v { "true" } else { "false" }.stream(stream)
    } else {
        if v { &1u8 } else { &0u8 }.stream(stream)
    }
}
