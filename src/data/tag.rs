use crate::{source, Receiver, Result, Source, Value};

#[inline]
pub fn tag() -> Tag {
    Tag::new()
}

#[inline]
pub fn tagged<V>(tag: Tag, value: V) -> Tagged<V> {
    Tagged::new(tag, value)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum TagShape {
    /**
    The tagged value inherits its shape from its contents.

    Values tagged with this shape have the same shape so long as their contents
    also have the same shape.
    */
    Contents,
    /**
    The tagged value may change dynamically.

    Values tagged with this shape have the same shape as any other value also
    tagged with this shape.
    */
    Dynamic,
    /**
    The tagged value is always the same.

    Values tagged with this shape have the same shape so long as their contents
    match exactly.
     */
    Constant,
    /**
    The tagged value may be null.

    Values tagged with this shape have the same shape so long as their contents
    also have the same shape, or their contents are null.
    */
    Nullable,
    /**
    The tagged value is a variable-length array.

    All elements of the array must have the same shape.

    Values tagged with this shape have the same shape so long as the shape of
    their elements is the same.
    */
    Slice,
    /**
    The tagged value is a fixed-length array.

    All elements of the array must have the same shape.

    Values tagged with this shape have the same shape so long as the shape of
    their elements is the same.
    */
    Array,
    /**
    The tagged value is an arbitrarily sized integer.

    The shape of an arbitrarily sized integer depends on whether the receiver is human
    readable or not.

    For human readable receivers, an arbitrarily sized integer is a text value with an
    optional leading `-` sign, followed by a sequence of one or more digits `0`-`9`.

    For binary receivers, an arbitrarily sized integer is a binary value consisting of the
    signed little-endian bytes of the integer.
    */
    BigInt,
    /**
    The tagged value is an enum variant.

    Values tagged with this shape have the same shape so long as the variant
    tag is different, or the contents have the same shape.
    */
    Enum,
    /**
    The tagged value is a struct.

    Values tagged with this shape have the same shape so long as any overlapping
    fields also have the same shape.
    */
    Struct,
    /**
    The tagged value is the key in a struct with named fields.

    Values tagged with this shape have the same shape so long as their contents
    also have the same shape.
    */
    StructKey,
    /**
    The tagged value is a struct with named or unnamed fields.

    Values tagged with this shape have the same shape so long as their contents
    also have the same shape.
     */
    StructValue,
    /**
    The tagged value is application-specific.

    This shape is not recommended for public types. It should only be used
    between sources and receivers in the same application.

    Whether or not values tagged with this shape have the same shape depends
    on how the application interprets the identifier.
    */
    Custom(u64),
}

impl Default for TagShape {
    fn default() -> Self {
        TagShape::Contents
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
    pub fn for_dynamic(self) -> Self {
        self.with_shape(TagShape::Dynamic)
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
    pub fn for_constant(self) -> Self {
        self.with_shape(TagShape::Constant)
    }

    #[inline]
    pub fn for_struct(self) -> Self {
        self.with_shape(TagShape::Struct)
    }

    #[inline]
    pub fn for_struct_key(self) -> Self {
        self.with_shape(TagShape::StructKey)
    }

    #[inline]
    pub fn for_struct_value(self) -> Self {
        self.with_shape(TagShape::StructValue)
    }

    #[inline]
    pub fn for_slice(self) -> Self {
        self.with_shape(TagShape::Slice)
    }

    #[inline]
    pub fn for_array(self) -> Self {
        self.with_shape(TagShape::Array)
    }

    #[inline]
    pub fn for_bigint(self) -> Self {
        self.with_shape(TagShape::BigInt)
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
