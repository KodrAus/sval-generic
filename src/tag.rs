use crate::{source::TypedSource, Result, Source, Stream, Value};

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct TypeTag<T> {
    pub ty: T,
}

pub fn type_tag<T: TypedSource<'static, str>>(ty: T) -> TypeTag<T> {
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

    pub fn value<V: Value>(self, value: V) -> TypeTagged<T, V> {
        TypeTagged::new(self, value)
    }
}

impl Value for TypeTag<&'static str> {
    fn stream<'a, S: Stream<'a>>(&'a self, mut stream: S) -> Result {
        stream.type_tag(*self)
    }
}

impl<'a, T: TypedSource<'static, str>> Source<'a> for TypeTag<T> {
    fn stream<'b, S: Stream<'b>>(&mut self, mut stream: S) -> Result
    where
        'a: 'b,
    {
        stream.type_tag(self.by_mut())
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct VariantTag<T, K> {
    pub ty: T,
    pub variant_key: K,
    pub variant_index: Option<u64>,
}

pub fn variant_tag<T: TypedSource<'static, str>, K: TypedSource<'static, str>>(
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

    pub fn value<V: Value>(self, value: V) -> VariantTagged<T, K, V> {
        VariantTagged::new(self, value)
    }
}

impl Value for VariantTag<&'static str, &'static str> {
    fn stream<'a, S: Stream<'a>>(&'a self, mut stream: S) -> Result {
        stream.variant_tag(*self)
    }
}

impl<'a, T: TypedSource<'static, str>, K: TypedSource<'static, str>> Source<'a>
    for VariantTag<T, K>
{
    fn stream<'b, S: Stream<'b>>(&mut self, mut stream: S) -> Result
    where
        'a: 'b,
    {
        stream.variant_tag(self.by_mut())
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
    fn stream<'a, S: Stream<'a>>(&'a self, mut stream: S) -> Result {
        stream.type_tagged(self.tag, &self.value)
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
    fn stream<'a, S: Stream<'a>>(&'a self, mut stream: S) -> Result {
        stream.variant_tagged(self.tag, &self.value)
    }
}
