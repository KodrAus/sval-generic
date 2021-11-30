use crate::{
    receiver::{self, Display},
    source::ValueSource,
    tag::{TypeTag, TypeTagged, VariantTag, VariantTagged},
    Receiver, Result,
};

pub fn stream<'a>(s: impl Receiver<'a>, v: &'a impl Value) -> Result {
    v.stream(s)
}

pub trait Value
where
    for<'a> &'a Self: ValueSource<'a, Self>,
{
    fn stream<'a, R: Receiver<'a>>(&'a self, receiver: R) -> Result;

    fn to_str(&self) -> Option<&str> {
        struct Extract<'a>(Option<&'a str>);

        impl<'a> Receiver<'a> for Extract<'a> {
            #[inline]
            fn display<D: Display>(&mut self, _: D) -> Result {
                receiver::unsupported()
            }

            #[inline]
            fn none(&mut self) -> Result {
                receiver::unsupported()
            }

            #[inline]
            fn str<'v: 'a, V: ValueSource<'v, str>>(&mut self, mut value: V) -> Result {
                match value.take_ref() {
                    Ok(v) => {
                        self.0 = Some(v);
                        Ok(())
                    }
                    _ => receiver::unsupported(),
                }
            }

            #[inline]
            fn map_begin(&mut self, _: receiver::Size) -> Result {
                receiver::unsupported()
            }

            #[inline]
            fn map_end(&mut self) -> Result {
                receiver::unsupported()
            }

            #[inline]
            fn map_key_begin(&mut self) -> Result {
                receiver::unsupported()
            }

            #[inline]
            fn map_key_end(&mut self) -> Result {
                receiver::unsupported()
            }

            #[inline]
            fn map_value_begin(&mut self) -> Result {
                receiver::unsupported()
            }

            #[inline]
            fn map_value_end(&mut self) -> Result {
                receiver::unsupported()
            }

            #[inline]
            fn seq_begin(&mut self, _: receiver::Size) -> Result {
                receiver::unsupported()
            }

            #[inline]
            fn seq_end(&mut self) -> Result {
                receiver::unsupported()
            }

            #[inline]
            fn seq_elem_begin(&mut self) -> Result {
                receiver::unsupported()
            }

            #[inline]
            fn seq_elem_end(&mut self) -> Result {
                receiver::unsupported()
            }
        }

        let mut receiver = Extract(None);
        self.stream(&mut receiver).ok()?;
        receiver.0
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
    fn stream<'b, R: Receiver<'b>>(&'b self, receiver: R) -> Result {
        (**self).stream(receiver)
    }

    #[inline]
    fn to_str(&self) -> Option<&str> {
        (**self).to_str()
    }
}
