use std::{error, fmt};

use crate::{
    erased,
    reference::{TypedRef, ValueRef},
};

#[doc(inline)]
pub use crate::{
    for_all::ForAll,
    stream::Stream,
    tag::{type_tag, variant_tag, TypeTag, TypeTagged, VariantTag, VariantTagged},
    Error, Result,
};

pub trait Value
where
    for<'a> &'a Self: ValueRef<'a>,
{
    fn stream<'a, S>(&'a self, stream: S) -> Result
    where
        S: Stream<'a>;

    fn to_str(&self) -> Option<&str> {
        struct Extract<'a>(Option<&'a str>);

        impl<'a> Stream<'a> for Extract<'a> {
            fn str<'v, V: TypedRef<'v, str>>(&mut self, v: V) -> Result
            where
                'v: 'a,
            {
                match v.try_unwrap() {
                    Some(v) => {
                        self.0 = Some(v);
                        Ok(())
                    }
                    _ => Err(Error),
                }
            }
        }

        let mut stream = Extract(None);
        self.stream(&mut stream).ok()?;
        stream.0
    }

    fn type_tag<T: TypedRef<'static, str>>(&self, tag: TypeTag<T>) -> TypeTagged<T, &Self> {
        TypeTagged::new(tag, self)
    }

    fn variant_tag<T: TypedRef<'static, str>, K: TypedRef<'static, str>>(
        &self,
        tag: VariantTag<T, K>,
    ) -> VariantTagged<T, K, &Self> {
        VariantTagged::new(tag, self)
    }

    fn for_all(&self) -> ForAll<&Self> {
        ForAll(self)
    }

    fn erase<'a>(&'a self) -> erased::Value<'a>
    where
        Self: Sized,
    {
        erased::Value::new(self)
    }
}

impl<'a, T: ?Sized> Value for &'a T
where
    T: Value,
{
    fn stream<'b, S>(&'b self, stream: S) -> Result
    where
        S: Stream<'b>,
    {
        (**self).stream(stream)
    }

    fn to_str(&self) -> Option<&str> {
        (**self).to_str()
    }
}
