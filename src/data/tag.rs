use crate::data::{Bytes, Error};
use crate::{
    source::{Stream, ValueSource},
    Receiver, Result, Source, Value,
};
use core::fmt::Display;

pub fn tag() -> Tag<&'static str> {
    Tag::new()
}

pub fn tagged<V>(value: V) -> Tagged<&'static str, V> {
    Tagged::new(Tag::new(), Tag::new(), value)
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub enum Kind {
    // No hint
    // Expect next: anything
    Unspecified,
    // An optional value
    Nullable,
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
    // A string formatted as a RFC3339 timestamp
    // Expect next: a string
    RFC3339DateTime,
    // A string formatted as a RFC3986 URI
    // Expect next: a string
    RFC3986Uri,
}

impl Default for Kind {
    fn default() -> Self {
        Kind::Unspecified
    }
}

impl Kind {
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

impl Value for Tag<&'static str> {
    fn stream<'a, S: Receiver<'a>>(&'a self, mut receiver: S) -> Result {
        receiver.tag(*self)
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

    pub fn begin_tag(&self) -> &Tag<T> {
        &self.begin_tag
    }

    pub fn end_tag(&self) -> &Tag<T> {
        &self.end_tag
    }

    pub fn begin_tag_mut(&mut self) -> &mut Tag<T> {
        &mut self.begin_tag
    }

    pub fn end_tag_mut(&mut self) -> &mut Tag<T> {
        &mut self.end_tag
    }

    pub fn value(&self) -> &V {
        &self.value
    }

    pub fn value_mut(&mut self) -> &mut V {
        &mut self.value
    }

    pub fn with_label<U: Clone>(self, label: U) -> Tagged<U, V> {
        Tagged {
            begin_tag: self.begin_tag.with_label(label.clone()),
            end_tag: self.end_tag.with_label(label),
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
    fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> Result {
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

    fn stream_to_end<'b, R: Receiver<'b>>(&mut self, receiver: R) -> Result
    where
        'a: 'b,
    {
        struct TaggedReceiver<'a, T, R> {
            // NOTE: It's expected that we'll only access the tag once on this receiver
            begin_tag: &'a mut Tag<T>,
            end_tag: &'a mut Tag<T>,
            receiver: R,
        }

        /*
        fn source<'v: 'a, S: Source<'v>>(&mut self, source: S) -> Result {
            self.receiver.tagged(self.tag.by_mut(), source)
        }

        fn value<'v: 'a, V: Value + ?Sized + 'v>(&mut self, value: &'v V) -> Result {
            self.receiver.tagged(self.tag.by_mut(), value)
        }

        fn str<'s: 'a, S: ValueSource<'s, str>>(&mut self, value: S) -> Result {
            self.receiver.tagged_str(self.tag.by_mut(), value)
        }
        */

        impl<'a, 'b, U: ValueSource<'static, str>, R: Receiver<'a>> Receiver<'a>
            for TaggedReceiver<'b, U, R>
        {
            fn value<'v: 'a, V: Value + ?Sized + 'v>(&mut self, value: &'v V) -> Result {
                todo!()
            }

            fn unstructured<D: Display>(&mut self, fmt: D) -> Result {
                todo!()
            }

            fn null(&mut self) -> Result {
                todo!()
            }

            fn u8(&mut self, value: u8) -> Result {
                todo!()
            }

            fn u16(&mut self, value: u16) -> Result {
                todo!()
            }

            fn u32(&mut self, value: u32) -> Result {
                todo!()
            }

            fn u64(&mut self, value: u64) -> Result {
                todo!()
            }

            fn i8(&mut self, value: i8) -> Result {
                todo!()
            }

            fn i16(&mut self, value: i16) -> Result {
                todo!()
            }

            fn i32(&mut self, value: i32) -> Result {
                todo!()
            }

            fn i64(&mut self, value: i64) -> Result {
                todo!()
            }

            fn u128(&mut self, value: u128) -> Result {
                todo!()
            }

            fn i128(&mut self, value: i128) -> Result {
                todo!()
            }

            fn f32(&mut self, value: f32) -> Result {
                todo!()
            }

            fn f64(&mut self, value: f64) -> Result {
                todo!()
            }

            fn bool(&mut self, value: bool) -> Result {
                todo!()
            }

            fn char(&mut self, value: char) -> Result {
                todo!()
            }

            fn str<'s: 'a, S: ValueSource<'s, str>>(&mut self, value: S) -> Result {
                todo!()
            }

            fn error<'e: 'a, E: ValueSource<'e, Error>>(&mut self, error: E) -> Result {
                todo!()
            }

            fn bytes<'s: 'a, B: ValueSource<'s, Bytes>>(&mut self, bytes: B) -> Result {
                todo!()
            }

            fn tag<T: ValueSource<'static, str>>(&mut self, tag: Tag<T>) -> Result {
                todo!()
            }

            fn tagged_begin<T: ValueSource<'static, str>>(&mut self, tag: Tag<T>) -> Result {
                todo!()
            }

            fn tagged_end<T: ValueSource<'static, str>>(&mut self, tag: Tag<T>) -> Result {
                todo!()
            }

            fn tagged<'v: 'a, T: ValueSource<'static, str>, V: Source<'v>>(
                &mut self,
                tag: Tag<T>,
                value: V,
            ) -> Result {
                todo!()
            }

            fn tagged_str<'s: 'a, T: ValueSource<'static, str>, S: ValueSource<'s, str>>(
                &mut self,
                tag: Tag<T>,
                value: S,
            ) -> Result {
                todo!()
            }

            fn tagged_bytes<'s: 'a, T: ValueSource<'static, str>, B: ValueSource<'s, Bytes>>(
                &mut self,
                tag: Tag<T>,
                value: B,
            ) -> Result {
                todo!()
            }

            fn map_begin(&mut self, size: Option<u64>) -> Result {
                todo!()
            }

            fn map_end(&mut self) -> Result {
                todo!()
            }

            fn map_key_begin(&mut self) -> Result {
                todo!()
            }

            fn map_key_end(&mut self) -> Result {
                todo!()
            }

            fn map_value_begin(&mut self) -> Result {
                todo!()
            }

            fn map_value_end(&mut self) -> Result {
                todo!()
            }

            fn map_entry<'k: 'a, 'v: 'a, K: Source<'k>, V: Source<'v>>(
                &mut self,
                key: K,
                value: V,
            ) -> Result {
                todo!()
            }

            fn map_field_entry<'v: 'a, F: ValueSource<'static, str>, V: Source<'v>>(
                &mut self,
                field: F,
                value: V,
            ) -> Result {
                todo!()
            }

            fn map_field<F: ValueSource<'static, str>>(&mut self, field: F) -> Result {
                todo!()
            }

            fn map_key<'k: 'a, K: Source<'k>>(&mut self, key: K) -> Result {
                todo!()
            }

            fn map_value<'v: 'a, V: Source<'v>>(&mut self, value: V) -> Result {
                todo!()
            }

            fn seq_begin(&mut self, size: Option<u64>) -> Result {
                todo!()
            }

            fn seq_end(&mut self) -> Result {
                todo!()
            }

            fn seq_elem_begin(&mut self) -> Result {
                todo!()
            }

            fn seq_elem_end(&mut self) -> Result {
                todo!()
            }

            fn seq_elem<'e: 'a, E: Source<'e>>(&mut self, elem: E) -> Result {
                todo!()
            }
        }

        let mut receiver = TaggedReceiver {
            begin_tag: &mut self.begin_tag,
            end_tag: &mut self.end_tag,
            receiver,
        };

        // Dispatch through our special Receiver so that we get a chance to map strings
        // to the more specialized tagged_str versions instead of through the general ones
        self.value.stream_to_end(receiver)
    }
}
