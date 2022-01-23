use core::fmt::Display;
use crate::{
    source::{Stream, ValueSource},
    Receiver, Result, Source, Value,
};
use crate::data::{Bytes, Digits, Error};

pub fn tag<T: ValueSource<'static, str>>(label: T) -> Tag<T> {
    Tag::new(label)
}

pub fn tagged<T: ValueSource<'static, str>, V>(label: T, value: V) -> Tagged<T, V> {
    Tagged::new(Tag::new(label), value)
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub enum ContentHint {
    // No hint
    // Expect next: anything
    Unknown,
    // An optional value
    Optional,
    // A map that follows internally tagged enum rules: a single static string key
    // TODO: Semantics need work for this one
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

impl Default for ContentHint {
    fn default() -> Self {
        ContentHint::Unknown
    }
}

impl ContentHint {
    pub fn is_fixed_size(&self) -> bool {
        match self {
            ContentHint::Struct | ContentHint::Tuple | ContentHint::Array => true,
            _ => false,
        }
    }
}

// NOTE: Tags aren't zero-cost. They're a piece of data you have to inspect and interpret
// and possibly branch on. They may accompany some other data, or they may be out-of-band
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct Tag<T> {
    pub label: T,
    pub content_hint: ContentHint,
}

impl<T> Tag<T> {
    pub fn new(label: T) -> Self {
        Tag {
            label,
            content_hint: Default::default(),
        }
    }

    pub fn with_content_hint(self, content_hint: ContentHint) -> Self {
        Tag {
            label: self.label,
            content_hint,
        }
    }

    pub fn by_ref(&self) -> Tag<&T> {
        Tag {
            label: &self.label,
            content_hint: self.content_hint,
        }
    }

    pub fn by_mut(&mut self) -> Tag<&mut T> {
        Tag {
            label: &mut self.label,
            content_hint: self.content_hint,
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
    pub tag: Tag<T>,
    pub value: V,
}

impl<T, V> Tagged<T, V> {
    pub fn new(tag: Tag<T>, value: V) -> Self {
        Tagged { tag, value }
    }

    pub fn with_content_hint(self, content_hint: ContentHint) -> Self {
        Tagged {
            tag: self.tag.with_content_hint(content_hint),
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
            tag: &'a mut Tag<T>,
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
            fn source<'v: 'a, S: Source<'v>>(&mut self, source: S) -> Result {
                todo!()
            }

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

            fn digits<'d: 'a, D: ValueSource<'d, Digits>>(&mut self, value: D) -> Result {
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

            fn tag_variant<T: ValueSource<'static, str>, K: ValueSource<'static, str>>(&mut self, type_tag: Tag<T>, variant_tag: Tag<K>, variant_index: Option<u64>) -> Result {
                todo!()
            }

            fn tagged_begin<T: ValueSource<'static, str>>(&mut self, tag: Tag<T>) -> Result {
                todo!()
            }

            fn tagged_end(&mut self) -> Result {
                todo!()
            }

            fn tagged_variant_begin<T: ValueSource<'static, str>, K: ValueSource<'static, str>>(&mut self, type_tag: Tag<T>, variant_tag: Tag<K>, variant_index: Option<u64>) -> Result {
                todo!()
            }

            fn tagged_variant_end(&mut self) -> Result {
                todo!()
            }

            fn tagged<'v: 'a, T: ValueSource<'static, str>, V: Source<'v>>(&mut self, tag: Tag<T>, value: V) -> Result {
                todo!()
            }

            fn tagged_variant<'v: 'a, T: ValueSource<'static, str>, K: ValueSource<'static, str>, V: Source<'v>>(&mut self, type_tag: Tag<T>, variant_tag: Tag<K>, variant_index: Option<u64>, value: V) -> Result {
                todo!()
            }

            fn tagged_str<'s: 'a, T: ValueSource<'static, str>, S: ValueSource<'s, str>>(&mut self, tag: Tag<T>, value: S) -> Result {
                todo!()
            }

            fn tagged_bytes<'s: 'a, T: ValueSource<'static, str>, B: ValueSource<'s, Bytes>>(&mut self, tag: Tag<T>, value: B) -> Result {
                todo!()
            }

            fn map_begin(&mut self, size: Option<u64>) -> Result {
                todo!()
            }

            fn map_end(&mut self) -> Result {
                todo!()
            }

            fn tagged_map_begin<T: ValueSource<'static, str>>(&mut self, tag: Tag<T>, size: Option<u64>) -> Result {
                todo!()
            }

            fn tagged_map_end(&mut self) -> Result {
                todo!()
            }

            fn tagged_variant_map_begin<T: ValueSource<'static, str>, K: ValueSource<'static, str>>(&mut self, type_tag: Tag<T>, variant_tag: Tag<K>, variant_index: Option<u64>, size: Option<u64>) -> Result {
                todo!()
            }

            fn tagged_variant_map_end(&mut self) -> Result {
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

            fn map_entry<'k: 'a, 'v: 'a, K: Source<'k>, V: Source<'v>>(&mut self, key: K, value: V) -> Result {
                todo!()
            }

            fn map_field_entry<'v: 'a, F: ValueSource<'static, str>, V: Source<'v>>(&mut self, field: F, value: V) -> Result {
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

            fn tagged_seq_begin<T: ValueSource<'static, str>>(&mut self, tag: Tag<T>, size: Option<u64>) -> Result {
                todo!()
            }

            fn tagged_seq_end(&mut self) -> Result {
                todo!()
            }

            fn tagged_variant_seq_begin<T: ValueSource<'static, str>, K: ValueSource<'static, str>>(&mut self, type_tag: Tag<T>, variant_tag: Tag<K>, variant_index: Option<u64>, size: Option<u64>) -> Result {
                todo!()
            }

            fn tagged_variant_seq_end(&mut self) -> Result {
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
            tag: &mut self.tag,
            receiver,
        };

        // Dispatch through our special Receiver so that we get a chance to map strings
        // to the more specialized tagged_str versions instead of through the general ones
        self.value.stream_to_end(receiver)
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct VariantTagged<T, K, V> {
    pub type_tag: Tag<T>,
    pub variant_tag: Tag<K>,
    pub variant_index: Option<u64>,
    pub value: V,
}

impl<T, K, V> VariantTagged<T, K, V> {
    pub fn new(
        type_tag: Tag<T>,
        variant_tag: Tag<K>,
        variant_index: Option<u64>,
        value: V,
    ) -> Self {
        VariantTagged {
            type_tag,
            variant_tag,
            variant_index,
            value,
        }
    }
}

impl<V: Value> Value for VariantTagged<&'static str, &'static str, V> {
    fn stream<'a, S: Receiver<'a>>(&'a self, mut receiver: S) -> Result {
        receiver.tagged_variant(
            self.type_tag,
            self.variant_tag,
            self.variant_index,
            &self.value,
        )
    }
}

impl<'a, T: ValueSource<'static, str>, K: ValueSource<'static, str>, S: Source<'a>> Source<'a>
    for VariantTagged<T, K, S>
{
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
        receiver.tagged_variant(
            self.type_tag.by_mut(),
            self.variant_tag.by_mut(),
            self.variant_index,
            &mut self.value,
        )
    }
}
