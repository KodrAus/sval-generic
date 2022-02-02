use crate::{receiver, source::SourceRef, std::fmt::Display, Receiver, Result};

pub trait SourceValue
where
    for<'a> &'a Self: SourceRef<'a, Self>,
{
    fn stream<'a, R: Receiver<'a>>(&'a self, receiver: R) -> Result;

    fn to_str(&self) -> Option<&str> {
        struct Extract<'a>(Option<&'a str>);

        impl<'a> Receiver<'a> for Extract<'a> {
            #[inline]
            fn unstructured<D: Display>(&mut self, _: D) -> Result {
                receiver::unsupported()
            }

            #[inline]
            fn null(&mut self) -> Result {
                receiver::unsupported()
            }

            #[inline]
            fn str<'v: 'a, V: SourceRef<'v, str>>(&mut self, mut value: V) -> Result {
                self.0 = value.try_take().ok();

                Ok(())
            }

            #[inline]
            fn map_begin(&mut self, _: Option<u64>) -> Result {
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
            fn seq_begin(&mut self, _: Option<u64>) -> Result {
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
}

impl<'a, T: SourceValue + ?Sized> SourceValue for &'a T {
    fn stream<'b, R: Receiver<'b>>(&'b self, receiver: R) -> Result {
        (**self).stream(receiver)
    }

    #[inline]
    fn to_str(&self) -> Option<&str> {
        (**self).to_str()
    }
}

#[cfg(feature = "alloc")]
mod alloc_support {
    use super::*;

    use crate::std::boxed::Box;

    impl<T: SourceValue + ?Sized> SourceValue for Box<T> {
        fn stream<'a, S: Receiver<'a>>(&'a self, receiver: S) -> crate::Result {
            (**self).stream(receiver)
        }
    }
}
