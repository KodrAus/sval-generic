use crate::{
    source::{Stream, ValueSource},
    Receiver, Result, Source, Value,
};

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct TypeTag<T> {
    pub ty: T,
}

pub fn type_tag<T: ValueSource<'static, str>>(ty: T) -> TypeTag<T> {
    TypeTag::new(ty)
}

impl<T> TypeTag<T> {
    pub fn new(ty: T) -> Self {
        TypeTag { ty }
    }

    pub fn by_ref(&self) -> TypeTag<&T> {
        TypeTag { ty: &self.ty }
    }

    pub fn by_mut(&mut self) -> TypeTag<&mut T> {
        TypeTag { ty: &mut self.ty }
    }

    pub fn tag<V: Value>(self, value: V) -> TypeTagged<T, V> {
        TypeTagged::new(self, value)
    }
}

impl Value for TypeTag<&'static str> {
    fn stream<'a, S: Receiver<'a>>(&'a self, mut receiver: S) -> Result {
        receiver.type_tag(*self)
    }
}

impl<'a, T: ValueSource<'static, str>> Source<'a> for TypeTag<T> {
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
        receiver.type_tag(self.by_mut())
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct VariantTag<T, K> {
    pub ty: T,
    pub variant_key: K,
    pub variant_index: Option<u64>,
}

pub fn variant_tag<T: ValueSource<'static, str>, K: ValueSource<'static, str>>(
    ty: T,
    variant_key: K,
    variant_index: Option<u64>,
) -> VariantTag<T, K> {
    VariantTag::new(ty, variant_key, variant_index)
}

impl<T, K> VariantTag<T, K> {
    pub fn new(ty: T, variant_key: K, variant_index: Option<u64>) -> Self {
        VariantTag {
            ty,
            variant_key,
            variant_index,
        }
    }

    pub fn by_ref(&self) -> VariantTag<&T, &K> {
        VariantTag {
            ty: &self.ty,
            variant_key: &self.variant_key,
            variant_index: self.variant_index,
        }
    }

    pub fn by_mut(&mut self) -> VariantTag<&mut T, &mut K> {
        VariantTag {
            ty: &mut self.ty,
            variant_key: &mut self.variant_key,
            variant_index: self.variant_index,
        }
    }

    pub fn tag<V: Value>(self, value: V) -> VariantTagged<T, K, V> {
        VariantTagged::new(self, value)
    }
}

impl Value for VariantTag<&'static str, &'static str> {
    fn stream<'a, S: Receiver<'a>>(&'a self, mut receiver: S) -> Result {
        receiver.variant_tag(*self)
    }
}

impl<'a, T: ValueSource<'static, str>, K: ValueSource<'static, str>> Source<'a>
    for VariantTag<T, K>
{
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
        receiver.variant_tag(self.by_mut())
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct TypeTagged<T, V> {
    pub tag: TypeTag<T>,
    pub value: V,
}

impl<T, V> TypeTagged<T, V> {
    pub fn new(tag: TypeTag<T>, value: V) -> Self {
        TypeTagged { tag, value }
    }
}

impl<V: Value> Value for TypeTagged<&'static str, V> {
    fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> Result {
        receiver.type_tagged(self.tag, &self.value)
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
        receiver.type_tagged(self.tag.by_mut(), &mut self.value)
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct VariantTagged<T, K, V> {
    pub tag: VariantTag<T, K>,
    pub value: V,
}

impl<T, K, V> VariantTagged<T, K, V> {
    pub fn new(tag: VariantTag<T, K>, value: V) -> Self {
        VariantTagged { tag, value }
    }
}

impl<V: Value> Value for VariantTagged<&'static str, &'static str, V> {
    fn stream<'a, S: Receiver<'a>>(&'a self, mut receiver: S) -> Result {
        receiver.variant_tagged(self.tag, &self.value)
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
        receiver.variant_tagged(self.tag.by_mut(), &mut self.value)
    }
}
