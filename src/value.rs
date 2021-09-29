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
    fn stream<'a, S: Stream<'a>>(&'a self, stream: S) -> Result;

    fn to_str(&self) -> Option<&str> {
        struct Extract<'a>(Option<&'a str>);

        impl<'a> Stream<'a> for Extract<'a> {
            fn str<'v: 'a, V: TypedRef<'v, str>>(&mut self, v: V) -> Result {
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

    fn erase(&self) -> erased::Value
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
    fn stream<'b, S: Stream<'b>>(&'b self, stream: S) -> Result {
        (**self).stream(stream)
    }

    fn to_str(&self) -> Option<&str> {
        (**self).to_str()
    }
}
