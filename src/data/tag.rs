use crate::{source, Receiver, Result, Source, Value};

pub fn tag() -> Tag {
    Tag::new()
}

pub fn tagged<V>(value: V, tag: Tag) -> Tagged<V> {
    Tagged::new(value, tag)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Shape {
    // No hint
    // Expect next: anything
    Unspecified,
    // An optional value
    // 1 should be used for Some
    // 0 should be used for None
    Nullable,
    // An enum
    // Followed by a second tagged item with a shape of EnumVariant
    Enum,
    // An enum variant
    // This prevents variants from holding other shapes
    EnumVariant,
    // A map that follows struct rules: static string keys
    // Expect next: a map
    Struct,
    // The name of a field in a struct
    StructField,
    // A sequence that follows tuple rules: fixed size, multi-type
    // Expect next: a sequence / string / bytes
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
}

impl Default for Shape {
    fn default() -> Self {
        Shape::Unspecified
    }
}

impl Shape {
    pub fn is_nullable(&self) -> bool {
        matches!(self, Shape::Nullable)
    }

    pub fn is_enum(&self) -> bool {
        matches!(self, Shape::Enum)
    }

    pub fn is_struct(&self) -> bool {
        matches!(self, Shape::Struct)
    }

    pub fn is_tuple(&self) -> bool {
        matches!(self, Shape::Tuple)
    }

    pub fn is_array(&self) -> bool {
        matches!(self, Shape::Array)
    }

    pub fn is_number(&self) -> bool {
        matches!(self, Shape::Number)
    }

    pub fn is_big_integer(&self) -> bool {
        matches!(self, Shape::BigInteger)
    }

    pub fn is_date_time(&self) -> bool {
        matches!(self, Shape::DateTime)
    }

    pub fn is_uri(&self) -> bool {
        matches!(self, Shape::Uri)
    }

    pub fn is_fixed_shape(&self) -> bool {
        match self {
            Shape::Enum | Shape::Array => true,
            _ => false,
        }
    }

    pub fn is_fixed_size(&self) -> bool {
        match self {
            Shape::Enum | Shape::Struct | Shape::Tuple | Shape::Array => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct Tag {
    label: Option<&'static str>,
    id: Option<u64>,
    shape: Shape,
}

impl Default for Tag {
    fn default() -> Self {
        Tag {
            label: Default::default(),
            id: Default::default(),
            shape: Default::default(),
        }
    }
}

impl Tag {
    pub fn new() -> Self {
        Tag::default()
    }

    pub fn label(&self) -> Option<&'static str> {
        self.label
    }

    pub fn id(&self) -> Option<u64> {
        self.id
    }

    pub fn shape(&self) -> Shape {
        self.shape
    }

    pub fn with_label(self, label: &'static str) -> Tag {
        Tag {
            label: Some(label),
            id: self.id,
            shape: self.shape,
        }
    }

    pub fn with_id(self, id: u64) -> Self {
        Tag {
            label: self.label,
            id: Some(id),
            shape: self.shape,
        }
    }

    pub fn for_nullable(self) -> Self {
        self.with_shape(Shape::Nullable)
    }

    pub fn for_enum(self) -> Self {
        self.with_shape(Shape::Enum)
    }

    pub fn for_enum_variant(self) -> Self {
        self.with_shape(Shape::EnumVariant)
    }

    pub fn for_struct(self) -> Self {
        self.with_shape(Shape::Struct)
    }

    pub fn for_struct_field(self) -> Self {
        self.with_shape(Shape::StructField)
    }

    pub fn for_tuple(self) -> Self {
        self.with_shape(Shape::Tuple)
    }

    pub fn for_array(self) -> Self {
        self.with_shape(Shape::Array)
    }

    pub fn for_number(self) -> Self {
        self.with_shape(Shape::Number)
    }

    pub fn for_big_integer(self) -> Self {
        self.with_shape(Shape::BigInteger)
    }

    pub fn for_date_time(self) -> Self {
        self.with_shape(Shape::DateTime)
    }

    pub fn for_uri(self) -> Self {
        self.with_shape(Shape::Uri)
    }

    pub fn with_shape(self, shape: Shape) -> Self {
        Tag {
            label: self.label,
            id: self.id,
            shape,
        }
    }
}

impl Value for Tag {
    fn stream<'a, S: Receiver<'a>>(&'a self, mut receiver: S) -> Result {
        receiver.tag(*self)
    }
}

impl<'a> Source<'a> for Tag {
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
        receiver.tag(*self)
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct Tagged<V> {
    tag: Tag,
    value: V,
}

impl<V> Tagged<V> {
    pub fn new(value: V, tag: Tag) -> Self {
        Tagged { tag, value }
    }

    pub fn tag(&self) -> Tag {
        self.tag
    }

    pub fn value(&self) -> &V {
        &self.value
    }

    pub fn value_mut(&mut self) -> &mut V {
        &mut self.value
    }

    pub fn with_value<U>(self, value: U) -> Tagged<U> {
        Tagged {
            tag: self.tag,
            value,
        }
    }

    pub fn map_value<U>(self, f: impl FnOnce(V) -> U) -> Tagged<U> {
        Tagged {
            tag: self.tag,
            value: f(self.value),
        }
    }

    pub fn try_map_value<U, E>(self, f: impl FnOnce(V) -> Result<U, E>) -> Result<Tagged<U>, E> {
        Ok(Tagged {
            tag: self.tag,
            value: f(self.value)?,
        })
    }

    pub fn as_ref(&self) -> Tagged<&V> {
        Tagged {
            tag: self.tag,
            value: &self.value,
        }
    }

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
