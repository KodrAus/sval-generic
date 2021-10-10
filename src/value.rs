use crate::{
    receiver::{self, AsyncReceiver, Display, Receiver},
    source::{Source, ValueSource},
    tag::{TypeTag, TypeTagged, VariantTag, VariantTagged},
    Result,
};

#[async_trait]
pub trait Value
where
    for<'a> &'a Self: Source<'a>,
{
    fn stream<'a, R: Receiver<'a>>(&'a self, receiver: R) -> Result;

    async fn stream_non_blocking<'a, S: AsyncReceiver<'a>>(&'a self, mut receiver: S) -> Result
    where
        Self: Sync,
    {
        // TODO: Try stream to a primitive first
        receiver.blocking(self).await
    }

    fn to_str(&self) -> Option<&str> {
        struct Extract<'a>(Option<&'a str>);

        impl<'a> Receiver<'a> for Extract<'a> {
            fn display<D: Display>(&mut self, _: D) -> Result {
                receiver::unsupported()
            }

            fn none(&mut self) -> Result {
                receiver::unsupported()
            }

            fn str<'v: 'a, V: ValueSource<'v, str>>(&mut self, mut value: V) -> Result {
                match value.value_ref() {
                    Ok(v) => {
                        self.0 = Some(v);
                        Ok(())
                    }
                    _ => receiver::unsupported(),
                }
            }

            fn map_begin(&mut self, _: Option<usize>) -> Result {
                receiver::unsupported()
            }

            fn map_end(&mut self) -> Result {
                receiver::unsupported()
            }

            fn map_key_begin(&mut self) -> Result {
                receiver::unsupported()
            }

            fn map_key_end(&mut self) -> Result {
                receiver::unsupported()
            }

            fn map_value_begin(&mut self) -> Result {
                receiver::unsupported()
            }

            fn map_value_end(&mut self) -> Result {
                receiver::unsupported()
            }

            fn seq_begin(&mut self, _: Option<usize>) -> Result {
                receiver::unsupported()
            }

            fn seq_end(&mut self) -> Result {
                receiver::unsupported()
            }

            fn seq_elem_begin(&mut self) -> Result {
                receiver::unsupported()
            }

            fn seq_elem_end(&mut self) -> Result {
                receiver::unsupported()
            }
        }

        let mut stream = Extract(None);
        self.stream(&mut stream).ok()?;
        stream.0
    }

    fn type_tag<T: ValueSource<'static, str>>(&self, tag: TypeTag<T>) -> TypeTagged<T, &Self> {
        tag.tag(self)
    }

    fn variant_tag<T: ValueSource<'static, str>, K: ValueSource<'static, str>>(
        &self,
        tag: VariantTag<T, K>,
    ) -> VariantTagged<T, K, &Self> {
        tag.tag(self)
    }
}

impl<'a, T: Value + ?Sized> Value for &'a T {
    fn stream<'b, S: Receiver<'b>>(&'b self, stream: S) -> Result {
        (**self).stream(stream)
    }

    fn to_str(&self) -> Option<&str> {
        (**self).to_str()
    }
}
