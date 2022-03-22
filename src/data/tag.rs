use crate::{source, Receiver, Result, Source, Value};

#[inline]
pub fn tag() -> Tag {
    Tag::new()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Tag {
    // NOTE: These fields are public and `Tag` is constructable (not non-exhaustive)
    // *specifically* to limit any possible changes to its shape.
    pub label: Option<&'static str>,
    pub id: Option<u64>,
}

impl Default for Tag {
    #[inline]
    fn default() -> Self {
        Tag {
            label: Default::default(),
            id: Default::default(),
        }
    }
}

impl Tag {
    #[inline]
    pub fn new() -> Self {
        Tag::default()
    }

    #[inline]
    pub fn with_label(self, label: impl Into<Option<&'static str>>) -> Tag {
        Tag {
            label: label.into(),
            id: self.id,
        }
    }

    #[inline]
    pub fn with_id(self, id: impl Into<Option<u64>>) -> Self {
        Tag {
            label: self.label,
            id: id.into(),
        }
    }
}

#[inline]
pub fn struct_key<T>(tag: Tag, key: T) -> StructKey<T> {
    StructKey::new(tag, key)
}

#[inline]
pub fn struct_value<T>(tag: Tag, value: T) -> StructValue<T> {
    StructValue::new(tag, value)
}

#[derive(Clone, Copy)]
pub struct StructKey<T> {
    tag: Tag,
    value: T,
}

#[derive(Clone, Copy)]
pub struct StructValue<T> {
    tag: Tag,
    value: T,
}

macro_rules! impl_wrapped {
    ($ty:ident, $begin:ident, $end:ident) => {
        impl<T> $ty<T> {
            pub fn new(tag: Tag, value: T) -> Self {
                $ty { tag, value }
            }

            pub fn tag(&self) -> Tag {
                self.tag
            }

            pub fn by_ref(&self) -> $ty<&T> {
                $ty {
                    tag: self.tag,
                    value: &self.value,
                }
            }

            pub fn by_mut(&mut self) -> $ty<&mut T> {
                $ty {
                    tag: self.tag,
                    value: &mut self.value,
                }
            }

            pub fn into_inner(self) -> T {
                self.value
            }
        }

        impl<T: Value> Value for $ty<T> {
            fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> Result {
                receiver.$begin(self.tag)?;
                self.value.stream(&mut receiver)?;
                receiver.$end()
            }
        }

        impl<'a, T: Source<'a>> Source<'a> for $ty<T> {
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
                receiver.$begin(self.tag)?;
                self.value.stream_to_end(&mut receiver)?;
                receiver.$end()?;

                Ok(())
            }
        }
    };
}

impl_wrapped!(StructKey, struct_key_begin, struct_key_end);
impl_wrapped!(StructValue, struct_value_begin, struct_value_end);
