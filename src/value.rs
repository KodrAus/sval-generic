use crate::{
    erased, fmt, serde,
    source::{Source, TypedSource},
    stream::{self, Display},
    tag::{TypeTag, VariantTag},
};

#[doc(inline)]
pub use crate::{
    for_all::ForAll,
    stream::Stream,
    tag::{type_tag, variant_tag, TypeTagged, VariantTagged},
    Error, Result,
};

#[async_trait]
pub trait Value
where
    for<'a> &'a Self: Source<'a>,
{
    fn stream<'a, S: Stream<'a>>(&'a self, stream: S) -> Result;

    /*
    async fn stream_non_blocking<'a, S: AsyncStream<'a>>(&'a self, stream: S) -> Result {
        stream.blocking(self)
    }
    */

    fn to_str(&self) -> Option<&str> {
        struct Extract<'a>(Option<&'a str>);

        impl<'a> Stream<'a> for Extract<'a> {
            fn str<'v: 'a, V: TypedSource<'v, str>>(&mut self, mut value: V) -> Result {
                match value.stream_to_ref() {
                    Ok(v) => {
                        self.0 = Some(v);
                        Ok(())
                    }
                    _ => stream::unsupported(),
                }
            }

            fn display<D: Display>(&mut self, _: D) -> Result {
                stream::unsupported()
            }

            fn none(&mut self) -> Result {
                stream::unsupported()
            }

            fn map_begin(&mut self, _: Option<usize>) -> Result {
                stream::unsupported()
            }

            fn map_key_begin(&mut self) -> Result {
                stream::unsupported()
            }

            fn map_key_end(&mut self) -> Result {
                stream::unsupported()
            }

            fn map_value_begin(&mut self) -> Result {
                stream::unsupported()
            }

            fn map_value_end(&mut self) -> Result {
                stream::unsupported()
            }

            fn map_end(&mut self) -> Result {
                stream::unsupported()
            }

            fn seq_begin(&mut self, _: Option<usize>) -> Result {
                stream::unsupported()
            }

            fn seq_elem_begin(&mut self) -> Result {
                stream::unsupported()
            }

            fn seq_elem_end(&mut self) -> Result {
                stream::unsupported()
            }

            fn seq_end(&mut self) -> Result {
                stream::unsupported()
            }
        }

        let mut stream = Extract(None);
        self.stream(&mut stream).ok()?;
        stream.0
    }

    fn type_tag<T: TypedSource<'static, str>>(&self, tag: TypeTag<T>) -> TypeTagged<T, &Self> {
        TypeTagged::new(tag, self)
    }

    fn variant_tag<T: TypedSource<'static, str>, K: TypedSource<'static, str>>(
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

    fn to_serialize(&self) -> serde::Value<&Self> {
        serde::Value::new(self)
    }

    fn to_debug(&self) -> fmt::Value<&Self> {
        fmt::Value::new(self)
    }
}

impl<'a, T: Value + ?Sized> Value for &'a T {
    fn stream<'b, S: Stream<'b>>(&'b self, stream: S) -> Result {
        (**self).stream(stream)
    }

    fn to_str(&self) -> Option<&str> {
        (**self).to_str()
    }
}
