use crate::{source, Receiver, Result, Source, Value};

#[inline]
pub fn tag() -> Tag {
    Tag::new()
}

#[inline]
pub fn tagged<V>(tag: Tag, value: V) -> Tagged<V> {
    Tagged::new(tag, value)
}

// Shape is purely structural. It's based on the flattened calls a `Receiver` may get.
// Tags are optional, but if they're used they can change shape. A tag and its associated
// data are considered to have the same shape if their `TagShape` is the same.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum TagShape {
    // No hint
    // Expect next: anything
    Unspecified,
    // An optional value
    // tagged(Nullable, null) and tagged(Nullable, T) have the same shape
    Nullable,
    // An enum
    // Followed by a second tagged item with a shape of EnumVariant
    Enum,
    EnumConstant,
    // A map that follows struct rules: static string keys
    // Expect next: a map
    Struct,
    // The name of a field in a struct
    StructField,
    // A seq that follows tuple rules: fixed size
    // Expect next: a seq
    Tuple,
    // A sequence that follows array rules: fixed size, single-type
    // NOTE: Rust slices _aren't_ necessarily arrays in sval. They
    // can contain the same Rust type but produce different shapes
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
    // A custom shape that's only understood by a specific combination of source and receiver
    Custom(u64),
}

impl Default for TagShape {
    fn default() -> Self {
        TagShape::Unspecified
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Tag {
    // NOTE: These fields are public and `Tag` is constructable (not non-exhaustive)
    // *specifically* to limit any possible changes to its shape.
    pub label: Option<&'static str>,
    pub id: Option<u64>,
    pub shape: TagShape,
}

impl Default for Tag {
    #[inline]
    fn default() -> Self {
        Tag {
            label: Default::default(),
            id: Default::default(),
            shape: Default::default(),
        }
    }
}

impl Tag {
    #[inline]
    pub fn new() -> Self {
        Tag::default()
    }

    #[inline]
    pub fn for_nullable(self) -> Self {
        self.with_shape(TagShape::Nullable)
    }

    #[inline]
    pub fn for_enum(self) -> Self {
        self.with_shape(TagShape::Enum)
    }

    #[inline]
    pub fn for_enum_constant(self) -> Self {
        self.with_shape(TagShape::EnumConstant)
    }

    #[inline]
    pub fn for_struct(self) -> Self {
        self.with_shape(TagShape::Struct)
    }

    #[inline]
    pub fn for_struct_field(self) -> Self {
        self.with_shape(TagShape::StructField)
    }

    #[inline]
    pub fn for_tuple(self) -> Self {
        self.with_shape(TagShape::Tuple)
    }

    #[inline]
    pub fn for_array(self) -> Self {
        self.with_shape(TagShape::Array)
    }

    #[inline]
    pub fn for_number(self) -> Self {
        self.with_shape(TagShape::Number)
    }

    #[inline]
    pub fn for_big_integer(self) -> Self {
        self.with_shape(TagShape::BigInteger)
    }

    #[inline]
    pub fn for_date_time(self) -> Self {
        self.with_shape(TagShape::DateTime)
    }

    #[inline]
    pub fn for_uri(self) -> Self {
        self.with_shape(TagShape::Uri)
    }

    #[inline]
    pub fn with_label(self, label: impl Into<Option<&'static str>>) -> Tag {
        Tag {
            label: label.into(),
            id: self.id,
            shape: self.shape,
        }
    }

    #[inline]
    pub fn with_id(self, id: impl Into<Option<u64>>) -> Self {
        Tag {
            label: self.label,
            id: id.into(),
            shape: self.shape,
        }
    }

    #[inline]
    pub fn with_shape(self, shape: TagShape) -> Self {
        Tag {
            label: self.label,
            id: self.id,
            shape,
        }
    }

    #[inline]
    pub fn with_value<V>(self, value: V) -> Tagged<V> {
        Tagged::new(self, value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Tagged<V> {
    // NOTE: These fields are public and `Tag` is constructable (not non-exhaustive)
    // *specifically* to limit any possible changes to its shape.
    pub tag: Tag,
    pub value: V,
}

impl<V> Tagged<V> {
    #[inline]
    pub fn new(tag: Tag, value: V) -> Self {
        Tagged { tag, value }
    }

    #[inline]
    pub fn with_value<U>(self, value: U) -> Tagged<U> {
        Tagged {
            tag: self.tag,
            value,
        }
    }

    #[inline]
    pub fn map_value<U>(self, f: impl FnOnce(V) -> U) -> Tagged<U> {
        Tagged {
            tag: self.tag,
            value: f(self.value),
        }
    }

    #[inline]
    pub fn try_map_value<U, E>(self, f: impl FnOnce(V) -> Result<U, E>) -> Result<Tagged<U>, E> {
        Ok(Tagged {
            tag: self.tag,
            value: f(self.value)?,
        })
    }

    #[inline]
    pub fn as_ref(&self) -> Tagged<&V> {
        Tagged {
            tag: self.tag,
            value: &self.value,
        }
    }

    #[inline]
    pub fn as_mut(&mut self) -> Tagged<&mut V> {
        Tagged {
            tag: self.tag,
            value: &mut self.value,
        }
    }
}

impl<V: Value> Value for Tagged<V> {
    fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> Result {
        receiver.tagged(self.as_ref())
    }
}

impl<'a, S: Source<'a>> Source<'a> for Tagged<S> {
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
