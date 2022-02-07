use crate::{error, std::fmt::Display, Receiver, Result, Source};

pub trait Value
where
    for<'a> &'a Self: Source<'a>,
{
    fn stream<'a, R: Receiver<'a>>(&'a self, receiver: R) -> Result;

    fn to_str(&self) -> Option<&str> {
        struct Extract<'a>(Option<&'a str>);

        impl<'a> Receiver<'a> for Extract<'a> {
            fn null(&mut self) -> Result {
                error::unsupported()
            }

            fn u128(&mut self, _: u128) -> Result {
                error::unsupported()
            }

            fn i128(&mut self, _: i128) -> Result {
                error::unsupported()
            }

            fn f64(&mut self, _: f64) -> Result {
                error::unsupported()
            }

            fn bool(&mut self, _: bool) -> Result {
                error::unsupported()
            }

            fn str(&mut self, value: &'a str) -> Result {
                self.0 = Some(value);
                Ok(())
            }

            fn text_begin(&mut self, _: Option<u64>) -> Result {
                error::unsupported()
            }

            fn text_fragment<D: Display>(&mut self, _: D) -> Result {
                error::unsupported()
            }

            fn text_end(&mut self) -> Result {
                error::unsupported()
            }

            fn binary_begin(&mut self, _: Option<u64>) -> Result {
                error::unsupported()
            }

            fn binary_fragment<B: AsRef<[u8]>>(&mut self, _: B) -> Result {
                error::unsupported()
            }

            fn binary_end(&mut self) -> Result {
                error::unsupported()
            }

            fn map_begin(&mut self, _: Option<u64>) -> Result {
                error::unsupported()
            }

            fn map_key_begin(&mut self) -> Result {
                error::unsupported()
            }

            fn map_key_end(&mut self) -> Result {
                error::unsupported()
            }

            fn map_value_begin(&mut self) -> Result {
                error::unsupported()
            }

            fn map_value_end(&mut self) -> Result {
                error::unsupported()
            }

            fn map_end(&mut self) -> Result {
                error::unsupported()
            }

            fn seq_begin(&mut self, _: Option<u64>) -> Result {
                error::unsupported()
            }

            fn seq_elem_begin(&mut self) -> Result {
                error::unsupported()
            }

            fn seq_elem_end(&mut self) -> Result {
                error::unsupported()
            }

            fn seq_end(&mut self) -> Result {
                error::unsupported()
            }
        }

        let mut receiver = Extract(None);
        self.stream(&mut receiver).ok()?;
        receiver.0
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

#[cfg(feature = "alloc")]
mod alloc_support {
    use super::*;

    use crate::std::boxed::Box;

    impl<T: Value + ?Sized> Value for Box<T> {
        fn stream<'a, S: Receiver<'a>>(&'a self, receiver: S) -> crate::Result {
            (**self).stream(receiver)
        }
    }
}
