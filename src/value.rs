use crate::{
    receiver::{self, Display},
    source::{Source, ValueSource},
    tag::{TypeTag, TypeTagged, VariantTag, VariantTagged},
};

pub use crate::{
    tag::{type_tag, variant_tag},
    Receiver, Result,
};

pub fn stream<'a>(s: impl Receiver<'a>, v: &'a impl Value) -> Result {
    v.stream(s)
}

pub trait Value
where
    for<'a> &'a Self: Source<'a>,
{
    fn stream<'a, R: Receiver<'a>>(&'a self, receiver: R) -> Result;

    // TODO: is_unbuffered()

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
                match value.take_ref() {
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
    fn stream<'b, S: Receiver<'b>>(&'b self, stream: S) -> Result {
        (**self).stream(stream)
    }

    fn to_str(&self) -> Option<&str> {
        (**self).to_str()
    }
}
