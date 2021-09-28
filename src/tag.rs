use crate::{
    stream::{Stream, TypedRef, ValueRef},
    value::Value,
    Result,
};

#[derive(Clone, Copy)]
pub struct TypeTag<T> {
    ty: T,
}

pub fn type_tag<T: TypedRef<'static, str>>(ty: T) -> TypeTag<T> {
    TypeTag::new(ty)
}

impl<T> TypeTag<T> {
    pub fn new(ty: T) -> Self {
        TypeTag { ty }
    }

    pub fn ty(&self) -> T
    where
        T: TypedRef<'static, str>,
    {
        self.ty
    }

    pub fn value<V: Value>(&self, value: V) -> TypeTagged<T, V>
    where
        T: TypedRef<'static, str>,
    {
        TypeTagged::new(*self, value)
    }
}

impl<T: TypedRef<'static, str>> Value for TypeTag<T> {
    fn stream<'a, S>(&'a self, mut stream: S) -> Result
    where
        S: Stream<'a>,
    {
        stream.type_tag(*self)
    }
}

impl<'a, T: TypedRef<'static, str>> ValueRef<'a> for TypeTag<T> {
    fn stream<'b, S>(self, mut stream: S) -> Result
    where
        'a: 'b,
        S: Stream<'b>,
    {
        stream.type_tag(self)
    }

    fn to_str(self) -> Option<&'a str> {
        None
    }
}

#[derive(Clone, Copy)]
pub struct VariantTag<T, K> {
    ty: T,
    variant_key: K,
    variant_index: Option<u64>,
}

pub fn variant_tag<T: TypedRef<'static, str>, K: TypedRef<'static, str>>(
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

    pub fn ty(&self) -> T
    where
        T: TypedRef<'static, str>,
    {
        self.ty
    }

    pub fn variant_key(&self) -> K
    where
        K: TypedRef<'static, str>,
    {
        self.variant_key
    }

    pub fn variant_index(&self) -> Option<u64> {
        self.variant_index
    }

    pub fn value<V: Value>(&self, value: V) -> VariantTagged<T, K, V>
    where
        T: TypedRef<'static, str>,
        K: TypedRef<'static, str>,
    {
        VariantTagged::new(*self, value)
    }
}

impl<T: TypedRef<'static, str>, K: TypedRef<'static, str>> Value for VariantTag<T, K> {
    fn stream<'a, S>(&'a self, mut stream: S) -> Result
    where
        S: Stream<'a>,
    {
        stream.variant_tag(*self)
    }
}

impl<'a, T: TypedRef<'static, str>, K: TypedRef<'static, str>> ValueRef<'a> for VariantTag<T, K> {
    fn stream<'b, S>(self, mut stream: S) -> Result
    where
        'a: 'b,
        S: Stream<'b>,
    {
        stream.variant_tag(self)
    }

    fn to_str(self) -> Option<&'a str> {
        None
    }
}

pub struct TypeTagged<T, V> {
    tag: TypeTag<T>,
    value: V,
}

impl<T, V> TypeTagged<T, V> {
    pub fn new(tag: TypeTag<T>, value: V) -> Self {
        TypeTagged { tag, value }
    }
}

impl<T: TypedRef<'static, str>, V: Value> Value for TypeTagged<T, V> {
    fn stream<'a, S>(&'a self, mut stream: S) -> Result
    where
        S: Stream<'a>,
    {
        stream.type_tagged(self.tag, &self.value)
    }
}

pub struct VariantTagged<T, K, V> {
    tag: VariantTag<T, K>,
    value: V,
}

impl<T, K, V> VariantTagged<T, K, V> {
    pub fn new(tag: VariantTag<T, K>, value: V) -> Self {
        VariantTagged { tag, value }
    }
}

impl<T: TypedRef<'static, str>, K: TypedRef<'static, str>, V: Value> Value
    for VariantTagged<T, K, V>
{
    fn stream<'a, S>(&'a self, mut stream: S) -> Result
    where
        S: Stream<'a>,
    {
        stream.variant_tagged(self.tag, &self.value)
    }
}
