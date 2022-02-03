use crate::{source, Receiver, Result, Source, SourceRef, SourceValue};

pub fn tag() -> Tag<&'static str> {
    Tag::new()
}

pub fn tagged<V>(value: V) -> Tagged<&'static str, V> {
    Tagged::new(Tag::new(), Tag::new(), value)
}

pub fn tag_nullable() -> Tag<&'static str> {
    Tag::new().with_kind(TagKind::Nullable)
}

pub fn tagged_nullable<V>(value: V) -> Tagged<&'static str, V> {
    Tagged::new(tag(), tag(), value)
}

pub fn tag_enum() -> Tag<&'static str> {
    Tag::new().with_kind(TagKind::Enum)
}

pub fn tagged_enum<V>(value: V) -> Tagged<&'static str, V> {
    Tagged::new(tag_enum(), tag_enum(), value)
}

pub fn tag_struct() -> Tag<&'static str> {
    Tag::new().with_kind(TagKind::Struct)
}

pub fn tagged_struct<V>(value: V) -> Tagged<&'static str, V> {
    Tagged::new(tag_struct(), tag_struct(), value)
}

pub fn tag_tuple() -> Tag<&'static str> {
    Tag::new().with_kind(TagKind::Tuple)
}

pub fn tagged_tuple<V>(value: V) -> Tagged<&'static str, V> {
    Tagged::new(tag_tuple(), tag_tuple(), value)
}

pub fn tag_slice() -> Tag<&'static str> {
    Tag::new().with_kind(TagKind::Slice)
}

pub fn tagged_slice<V>(value: V) -> Tagged<&'static str, V> {
    Tagged::new(tag_slice(), tag_slice(), value)
}

pub fn tag_array() -> Tag<&'static str> {
    Tag::new().with_kind(TagKind::Array)
}

pub fn tagged_array<V>(value: V) -> Tagged<&'static str, V> {
    Tagged::new(tag_array(), tag_array(), value)
}

pub fn tag_number() -> Tag<&'static str> {
    Tag::new().with_kind(TagKind::Number)
}

pub fn tagged_number<V>(value: V) -> Tagged<&'static str, V> {
    Tagged::new(tag_number(), tag_number(), value)
}

pub fn tag_big_integer() -> Tag<&'static str> {
    Tag::new().with_kind(TagKind::BigInteger)
}

pub fn tagged_big_integer<V>(value: V) -> Tagged<&'static str, V> {
    Tagged::new(tag_big_integer(), tag_big_integer(), value)
}

pub fn tag_date_time() -> Tag<&'static str> {
    Tag::new().with_kind(TagKind::DateTime)
}

pub fn tagged_date_time<V>(value: V) -> Tagged<&'static str, V> {
    Tagged::new(tag_date_time(), tag_date_time(), value)
}

pub fn tag_uri() -> Tag<&'static str> {
    Tag::new().with_kind(TagKind::Uri)
}

pub fn tagged_uri<V>(value: V) -> Tagged<&'static str, V> {
    Tagged::new(tag_uri(), tag_uri(), value)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum TagKind {
    // No hint
    // Expect next: anything
    Unspecified,
    // An optional value
    // 1 should be used for Some
    // 0 should be used for None
    Nullable,
    // An enum
    // Followed by a second tagged item for the variant
    Enum,
    // A map that follows struct rules: static string keys
    // Expect next: a map
    Struct,
    // A sequence that follows tuple rules: fixed size, multi-type
    // Expect next: a sequence
    Tuple,
    // A sequence that follows slice rules: variable size, single-type
    // Expect next: a sequence
    Slice,
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

impl Default for TagKind {
    fn default() -> Self {
        TagKind::Unspecified
    }
}

impl TagKind {
    pub fn is_unspecified(&self) -> bool {
        matches!(self, TagKind::Unspecified)
    }

    pub fn is_nullable(&self) -> bool {
        matches!(self, TagKind::Nullable)
    }

    pub fn is_enum(&self) -> bool {
        matches!(self, TagKind::Enum)
    }

    pub fn is_struct(&self) -> bool {
        matches!(self, TagKind::Struct)
    }

    pub fn is_tuple(&self) -> bool {
        matches!(self, TagKind::Tuple)
    }

    pub fn is_slice(&self) -> bool {
        matches!(self, TagKind::Slice)
    }

    pub fn is_array(&self) -> bool {
        matches!(self, TagKind::Array)
    }

    pub fn is_number(&self) -> bool {
        matches!(self, TagKind::Number)
    }

    pub fn is_big_integer(&self) -> bool {
        matches!(self, TagKind::BigInteger)
    }

    pub fn is_date_time(&self) -> bool {
        matches!(self, TagKind::DateTime)
    }

    pub fn is_uri(&self) -> bool {
        matches!(self, TagKind::Uri)
    }

    // TODO: What does it mean to be a type? We should map out `sval`'s type system
    pub fn is_fixed_type(&self) -> bool {
        match self {
            TagKind::Enum | TagKind::Array | TagKind::Slice => true,
            _ => false,
        }
    }

    pub fn is_fixed_size(&self) -> bool {
        match self {
            TagKind::Enum | TagKind::Struct | TagKind::Tuple | TagKind::Array => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct Tag<T> {
    label: Option<T>,
    id: Option<u64>,
    kind: TagKind,
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

    pub fn with_kind(self, kind: TagKind) -> Self {
        Tag {
            label: self.label,
            id: self.id,
            kind,
        }
    }

    pub fn as_ref(&self) -> Tag<&T> {
        Tag {
            label: self.label.as_ref(),
            id: self.id,
            kind: self.kind,
        }
    }

    pub fn as_mut(&mut self) -> Tag<&mut T> {
        Tag {
            label: self.label.as_mut(),
            id: self.id,
            kind: self.kind,
        }
    }
}

impl SourceValue for Tag<&'static str> {
    fn stream<'a, S: Receiver<'a>>(&'a self, mut receiver: S) -> Result {
        receiver.tag(*self)
    }
}

impl<'a, T: SourceRef<'static, str>> Source<'a> for Tag<T> {
    fn stream_resume<'b, S: Receiver<'b>>(&mut self, receiver: S) -> Result<source::Resume>
    where
        'a: 'b,
    {
        self.stream_to_end(receiver).map(|_| source::Resume::Done)
    }

    fn stream_to_end<'b, S: Receiver<'b>>(&mut self, mut receiver: S) -> Result
    where
        'a: 'b,
    {
        receiver.tag(self.as_mut())
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct Tagged<T, V> {
    begin_label: Option<T>,
    end_label: Option<T>,
    id: Option<u64>,
    kind: TagKind,
    value: V,
}

impl<T, V> Tagged<T, V> {
    pub fn new(begin_tag: Tag<T>, end_tag: Tag<T>, value: V) -> Self {
        Tagged {
            begin_label: begin_tag.label,
            end_label: end_tag.label,
            id: begin_tag.id,
            kind: begin_tag.kind,
            value,
        }
    }

    pub fn with_label<U: Clone>(self, label: U) -> Tagged<U, V> {
        Tagged {
            begin_label: Some(label.clone()),
            end_label: Some(label),
            id: self.id,
            kind: self.kind,
            value: self.value,
        }
    }

    pub fn map_label<U>(self, f: impl Fn(T) -> U) -> Tagged<U, V> {
        Tagged {
            begin_label: self.begin_label.map(&f),
            end_label: self.end_label.map(f),
            id: self.id,
            kind: self.kind,
            value: self.value,
        }
    }

    pub fn try_map_label<U, E>(self, f: impl Fn(T) -> Result<U, E>) -> Result<Tagged<U, V>, E> {
        Ok(Tagged {
            begin_label: match self.begin_label {
                Some(label) => Some(f(label)?),
                None => None,
            },
            end_label: match self.end_label {
                Some(label) => Some(f(label)?),
                None => None,
            },
            id: self.id,
            kind: self.kind,
            value: self.value,
        })
    }

    pub fn with_id(self, id: u64) -> Self {
        Tagged {
            begin_label: self.begin_label,
            end_label: self.end_label,
            id: Some(id),
            kind: self.kind,
            value: self.value,
        }
    }

    pub fn with_kind(self, kind: TagKind) -> Self {
        Tagged {
            begin_label: self.begin_label,
            end_label: self.end_label,
            id: self.id,
            kind,
            value: self.value,
        }
    }

    pub fn with_value<U>(self, value: U) -> Tagged<T, U> {
        Tagged {
            begin_label: self.begin_label,
            end_label: self.end_label,
            id: self.id,
            kind: self.kind,
            value,
        }
    }

    pub fn map_value<U>(self, f: impl FnOnce(V) -> U) -> Tagged<T, U> {
        Tagged {
            begin_label: self.begin_label,
            end_label: self.end_label,
            id: self.id,
            kind: self.kind,
            value: f(self.value),
        }
    }

    pub fn try_map_value<U, E>(self, f: impl FnOnce(V) -> Result<U, E>) -> Result<Tagged<T, U>, E> {
        Ok(Tagged {
            begin_label: self.begin_label,
            end_label: self.end_label,
            id: self.id,
            kind: self.kind,
            value: f(self.value)?,
        })
    }

    pub fn as_ref(&self) -> Tagged<&T, &V> {
        Tagged {
            begin_label: self.begin_label.as_ref(),
            end_label: self.end_label.as_ref(),
            id: self.id,
            kind: self.kind,
            value: &self.value,
        }
    }

    pub fn as_mut(&mut self) -> Tagged<&mut T, &mut V> {
        Tagged {
            begin_label: self.begin_label.as_mut(),
            end_label: self.end_label.as_mut(),
            id: self.id,
            kind: self.kind,
            value: &mut self.value,
        }
    }
}

impl<V: SourceValue> SourceValue for Tagged<&'static str, V> {
    fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> Result {
        receiver.tagged(self.as_ref().map_label(|l| *l))
    }
}

impl<'a, T: SourceRef<'static, str>, S: Source<'a>> Source<'a> for Tagged<T, S> {
    fn stream_resume<'b, R: Receiver<'b>>(&mut self, receiver: R) -> Result<source::Resume>
    where
        'a: 'b,
    {
        self.stream_to_end(receiver).map(|_| source::Resume::Done)
    }

    fn stream_to_end<'b, R: Receiver<'b>>(&mut self, mut receiver: R) -> Result
    where
        'a: 'b,
    {
        receiver.tagged(self.as_mut())
    }
}

#[cfg(feature = "alloc")]
mod alloc_support {
    use super::*;

    use crate::{for_all, source, std::borrow::Cow};

    struct Adapter<'a>(&'a Cow<'static, str>);

    impl<'a> Source<'static> for Adapter<'a> {
        fn stream_resume<'b, R: Receiver<'b>>(&mut self, receiver: R) -> Result<source::Resume> {
            self.stream_to_end(receiver).map(|_| source::Resume::Done)
        }

        fn stream_to_end<'b, R: Receiver<'b>>(&mut self, receiver: R) -> Result {
            match self.0 {
                Cow::Borrowed(v) => (*v).stream(receiver),
                Cow::Owned(v) => (*v).stream(for_all(receiver)),
            }
        }
    }

    impl<'a> SourceRef<'static, str> for Adapter<'a> {
        type Error = source::Impossible;

        #[inline]
        fn take(&mut self) -> Result<&str, source::TakeError<Self::Error>> {
            Ok(&**self.0)
        }

        #[inline]
        fn try_take(&mut self) -> Result<&'static str, source::TryTakeError<&str, Self::Error>> {
            match self.0 {
                Cow::Borrowed(v) => Ok(*v),
                Cow::Owned(v) => Err(source::TryTakeError::Fallback(&*v)),
            }
        }
    }

    impl SourceValue for Tag<Cow<'static, str>> {
        fn stream<'a, S: Receiver<'a>>(&'a self, mut receiver: S) -> Result {
            receiver.tag(self.as_ref().map_label(Adapter))
        }
    }

    impl<T: SourceValue> SourceValue for Tagged<Cow<'static, str>, T> {
        fn stream<'a, S: Receiver<'a>>(&'a self, mut receiver: S) -> Result {
            receiver.tagged(self.as_ref().map_label(Adapter))
        }
    }
}
