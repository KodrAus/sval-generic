use crate::{
    source::{Stream, ValueSource},
    Receiver, Result, Source, Value,
};

pub fn tag() -> Tag<&'static str> {
    Tag::new()
}

pub fn tagged<V>(value: V) -> Tagged<&'static str, V> {
    Tagged::new(Tag::new(), Tag::new(), value)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Kind {
    // No hint
    // Expect next: anything
    Unspecified,
    // An optional value
    // 1 should be used for Some
    // 0 should be used for None
    Nullable,
    // A fallible value
    // 1 should be used for Err
    // 0 should be used for Ok
    Fallible,
    // An enum
    // Followed by a second tagged item for the variant
    Enum,
    // A map that follows struct rules: static string keys
    // Expect next: a map
    Struct,
    // A sequence that follows tuple rules: fixed size, multi-type
    // Expect next: a sequence
    Tuple,
    // A sequence that follows array rules: fixed size, single-type
    // Expect next: a sequence / string / bytes
    Array,
    // Text: A string formatted as a RFC8259 number
    // Binary: A tuple of two integers: mantissa and base-2 scaling factor
    Number,
    // Text: A string containing a sequence of digits from 0-9 with optional leading `-`
    // Binary: A two's compliment integer in LE format
    BigInteger,
    // Text: A string formatted as a RFC3339 timestamp
    // Binary: An integer or number with seconds offset since Unix Epoch
    DateTime,
    // All: A string formatted as a RFC3986 URI
    Uri,
}

impl Default for Kind {
    fn default() -> Self {
        Kind::Unspecified
    }
}

impl Kind {
    pub fn is_enum(&self) -> bool {
        matches!(self, Kind::Enum)
    }

    pub fn is_fixed_size(&self) -> bool {
        match self {
            Kind::Struct | Kind::Tuple | Kind::Array => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct Tag<T> {
    label: Option<T>,
    id: Option<u64>,
    kind: Kind,
}

impl<T> Default for Tag<T> {
    fn default() -> Self {
        Tag {
            label: Default::default(),
            id: Default::default(),
            kind: Default::default(),
        }
    }
}

impl<T> Tag<T> {
    pub fn new() -> Self {
        Tag::default()
    }

    pub fn label(&self) -> Option<&T> {
        self.label.as_ref()
    }

    pub fn label_mut(&mut self) -> Option<&mut T> {
        self.label.as_mut()
    }

    pub fn id(&self) -> Option<u64> {
        self.id
    }

    pub fn kind(&self) -> Kind {
        self.kind
    }

    pub fn with_kind(self, kind: Kind) -> Self {
        Tag {
            label: self.label,
            id: self.id,
            kind,
        }
    }

    pub fn with_label<U>(self, label: U) -> Tag<U> {
        Tag {
            label: Some(label),
            id: self.id,
            kind: self.kind,
        }
    }

    pub fn map_label<U>(self, f: impl FnOnce(T) -> U) -> Tag<U> {
        Tag {
            label: self.label.map(f),
            id: self.id,
            kind: self.kind,
        }
    }

    pub fn try_map_label<U, E>(self, f: impl FnOnce(T) -> Result<U, E>) -> Result<Tag<U>, E> {
        Ok(Tag {
            label: match self.label {
                Some(label) => Some(f(label)?),
                None => None,
            },
            id: self.id,
            kind: self.kind,
        })
    }

    pub fn with_id(self, id: u64) -> Self {
        Tag {
            label: self.label,
            id: Some(id),
            kind: self.kind,
        }
    }

    pub fn by_ref(&self) -> Tag<&T> {
        Tag {
            label: self.label.as_ref(),
            id: self.id,
            kind: self.kind,
        }
    }

    pub fn by_mut(&mut self) -> Tag<&mut T> {
        Tag {
            label: self.label.as_mut(),
            id: self.id,
            kind: self.kind,
        }
    }
}

impl<'a, T: ValueSource<'static, str>> Source<'a> for Tag<T> {
    fn stream_resume<'b, S: Receiver<'b>>(&mut self, receiver: S) -> Result<Stream>
    where
        'a: 'b,
    {
        self.stream_to_end(receiver).map(|_| Stream::Done)
    }

    fn stream_to_end<'b, S: Receiver<'b>>(&mut self, mut receiver: S) -> Result
    where
        'a: 'b,
    {
        receiver.tag(self.by_mut())
    }
}

impl Value for Tag<&'static str> {
    fn stream<'a, S: Receiver<'a>>(&'a self, mut receiver: S) -> Result {
        receiver.tag(*self)
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct Tagged<T, V> {
    begin_tag: Tag<T>,
    end_tag: Tag<T>,
    value: V,
}

impl<T, V> Tagged<T, V> {
    pub fn new(begin_tag: Tag<T>, end_tag: Tag<T>, value: V) -> Self {
        Tagged {
            begin_tag,
            end_tag,
            value,
        }
    }

    pub fn begin_tag(&self) -> Tag<&T> {
        self.begin_tag.by_ref()
    }

    pub fn end_tag(&self) -> Tag<&T> {
        self.end_tag.by_ref()
    }

    pub fn begin_tag_mut(&mut self) -> Tag<&mut T> {
        self.begin_tag.by_mut()
    }

    pub fn end_tag_mut(&mut self) -> Tag<&mut T> {
        self.end_tag.by_mut()
    }

    pub fn value(&self) -> &V {
        &self.value
    }

    pub fn value_mut(&mut self) -> &mut V {
        &mut self.value
    }

    pub fn by_mut(&mut self) -> Tagged<&mut T, &mut V> {
        Tagged {
            begin_tag: self.begin_tag.by_mut(),
            end_tag: self.end_tag.by_mut(),
            value: &mut self.value,
        }
    }

    pub fn map_value<U>(self, f: impl FnOnce(V) -> U) -> Tagged<T, U> {
        Tagged {
            begin_tag: self.begin_tag,
            end_tag: self.end_tag,
            value: f(self.value),
        }
    }

    pub fn with_label<U: Clone>(self, label: U) -> Tagged<U, V> {
        Tagged {
            begin_tag: self.begin_tag.with_label(label.clone()),
            end_tag: self.end_tag.with_label(label),
            value: self.value,
        }
    }

    pub fn map_label<U>(self, f: impl Fn(T) -> U) -> Tagged<U, V> {
        Tagged {
            begin_tag: self.begin_tag.map_label(&f),
            end_tag: self.end_tag.map_label(f),
            value: self.value,
        }
    }

    pub fn with_id(self, id: u64) -> Self {
        Tagged {
            begin_tag: self.begin_tag.with_id(id),
            end_tag: self.end_tag.with_id(id),
            value: self.value,
        }
    }

    pub fn with_kind(self, kind: Kind) -> Self {
        Tagged {
            begin_tag: self.begin_tag.with_kind(kind),
            end_tag: self.end_tag.with_kind(kind),
            value: self.value,
        }
    }

    pub fn with_begin_label(self, label: T) -> Self {
        Tagged {
            begin_tag: self.begin_tag.with_label(label),
            end_tag: self.end_tag,
            value: self.value,
        }
    }

    pub fn with_end_label(self, label: T) -> Self {
        Tagged {
            begin_tag: self.begin_tag,
            end_tag: self.end_tag.with_label(label),
            value: self.value,
        }
    }
}

impl<V: Value> Value for Tagged<&'static str, V> {
    fn stream<'a, R: Receiver<'a>>(&'a self, receiver: R) -> Result {
        (&*self).stream_to_end(receiver)
    }
}

impl<'a, T: ValueSource<'static, str>, S: Source<'a>> Source<'a> for Tagged<T, S> {
    fn stream_resume<'b, R: Receiver<'b>>(&mut self, receiver: R) -> Result<Stream>
    where
        'a: 'b,
    {
        self.stream_to_end(receiver).map(|_| Stream::Done)
    }

    fn stream_to_end<'b, R: Receiver<'b>>(&mut self, mut receiver: R) -> Result
    where
        'a: 'b,
    {
        receiver.tagged(self.by_mut())
    }
}

#[cfg(feature = "alloc")]
mod alloc_support {
    use super::*;

    use crate::std::borrow::Cow;

    impl Value for Tag<Cow<'static, str>> {
        fn stream<'a, S: Receiver<'a>>(&'a self, mut receiver: S) -> Result {
            receiver.tag(self.by_ref())
        }
    }
}
