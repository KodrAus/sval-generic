use crate::{
    source::{Stream, ValueSource},
    Receiver, Result, Source, Value,
};

pub fn tag<T: ValueSource<'static, str>>(label: T, kind: Option<TagKind>) -> Tag<T> {
    Tag::new(label, kind)
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub enum TagKind {
    // RFC3339
    Timestamp,
    // RFC3986
    Uri,
    // RFC791 (dot decimal)
    Ipv4,
    // RFC2460 (hextets)
    Ipv6,
    // RFC4122 (hyphenated)
    Uuid,
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct Tag<T> {
    pub label: T,
    pub kind: Option<TagKind>,
}

impl<T> Tag<T> {
    pub fn new(label: T, kind: Option<TagKind>) -> Self {
        Tag { label, kind }
    }

    pub fn by_ref(&self) -> Tag<&T> {
        Tag {
            label: &self.label,
            kind: self.kind,
        }
    }

    pub fn by_mut(&mut self) -> Tag<&mut T> {
        Tag {
            label: &mut self.label,
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
    fn stream<'b, S: Receiver<'b>>(&mut self, receiver: S) -> Result<Stream>
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
pub struct TypeTagged<T, V> {
    pub tag: Tag<T>,
    pub value: V,
}

impl<T, V> TypeTagged<T, V> {
    pub fn new(tag: Tag<T>, value: V) -> Self {
        TypeTagged { tag, value }
    }
}

impl<V: Value> Value for TypeTagged<&'static str, V> {
    fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> Result {
        receiver.tagged(self.tag, &self.value)
    }
}

impl<'a, T: ValueSource<'static, str>, S: Source<'a>> Source<'a> for TypeTagged<T, S> {
    fn stream<'b, R: Receiver<'b>>(&mut self, receiver: R) -> Result<Stream>
    where
        'a: 'b,
    {
        self.stream_to_end(receiver).map(|_| Stream::Done)
    }

    fn stream_to_end<'b, R: Receiver<'b>>(&mut self, mut receiver: R) -> Result
    where
        'a: 'b,
    {
        receiver.tagged(self.tag.by_mut(), &mut self.value)
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
    fn stream<'b, R: Receiver<'b>>(&mut self, receiver: R) -> Result<Stream>
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
