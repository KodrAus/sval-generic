use crate::{error, receiver::DefaultUnsupported, std::convert::TryInto, Receiver, Result, Source};

/**
An immutable and repeatable source of structured data.

# Implementation notes

Valid implementations of `Value` must adhere to the following requirements:

1. All instances of this type must always stream with the same shape.
2. If the type also implements [`Source`] then [`Value::stream`] must be the same
as [`Source::stream_to_end`].
*/
pub trait Value
where
    for<'a> &'a Self: Source<'a>,
{
    fn stream<'a, R: Receiver<'a>>(&'a self, receiver: R) -> Result;

    #[inline]
    fn is_dynamic(&self) -> bool {
        struct Check(bool);

        impl<'a> DefaultUnsupported<'a> for Check {
            fn dynamic_begin(&mut self) -> Result {
                self.0 = true;
                Ok(())
            }

            fn dynamic_end(&mut self) -> Result {
                Ok(())
            }
        }

        let mut check = Check(false);
        if let Ok(()) = self.stream(check.as_receiver()) {
            check.0
        } else {
            false
        }
    }

    #[inline]
    fn to_bool(&self) -> Option<bool> {
        struct Extract(Option<bool>);

        impl<'a> DefaultUnsupported<'a> for Extract {
            fn bool(&mut self, value: bool) -> Result {
                self.0 = Some(value);
                Ok(())
            }
        }

        let mut extract = Extract(None);
        self.stream(extract.as_receiver()).ok()?;
        extract.0
    }

    #[inline]
    fn to_f32(&self) -> Option<f32> {
        struct Extract(Option<f32>);

        impl<'a> DefaultUnsupported<'a> for Extract {
            fn f32(&mut self, value: f32) -> Result {
                self.0 = Some(value);
                Ok(())
            }
        }

        let mut extract = Extract(None);
        self.stream(extract.as_receiver()).ok()?;
        extract.0
    }

    #[inline]
    fn to_f64(&self) -> Option<f64> {
        struct Extract(Option<f64>);

        impl<'a> DefaultUnsupported<'a> for Extract {
            fn f64(&mut self, value: f64) -> Result {
                self.0 = Some(value);
                Ok(())
            }
        }

        let mut extract = Extract(None);
        self.stream(extract.as_receiver()).ok()?;
        extract.0
    }

    #[inline]
    fn to_i8(&self) -> Option<i8> {
        self.to_i128().and_then(|value| value.try_into().ok())
    }

    #[inline]
    fn to_i16(&self) -> Option<i16> {
        self.to_i128().and_then(|value| value.try_into().ok())
    }

    #[inline]
    fn to_i32(&self) -> Option<i32> {
        self.to_i128().and_then(|value| value.try_into().ok())
    }

    #[inline]
    fn to_i64(&self) -> Option<i64> {
        self.to_i128().and_then(|value| value.try_into().ok())
    }

    #[inline]
    fn to_i128(&self) -> Option<i128> {
        struct Extract(Option<i128>);

        impl<'a> DefaultUnsupported<'a> for Extract {
            fn i128(&mut self, value: i128) -> Result {
                self.0 = Some(value);
                Ok(())
            }
        }

        let mut extract = Extract(None);
        self.stream(extract.as_receiver()).ok()?;
        extract.0
    }

    #[inline]
    fn to_u8(&self) -> Option<u8> {
        self.to_u128().and_then(|value| value.try_into().ok())
    }

    #[inline]
    fn to_u16(&self) -> Option<u16> {
        self.to_u128().and_then(|value| value.try_into().ok())
    }

    #[inline]
    fn to_u32(&self) -> Option<u32> {
        self.to_u128().and_then(|value| value.try_into().ok())
    }

    #[inline]
    fn to_u64(&self) -> Option<u64> {
        self.to_u128().and_then(|value| value.try_into().ok())
    }

    #[inline]
    fn to_u128(&self) -> Option<u128> {
        struct Extract(Option<u128>);

        impl<'a> DefaultUnsupported<'a> for Extract {
            fn u128(&mut self, value: u128) -> Result {
                self.0 = Some(value);
                Ok(())
            }
        }

        let mut extract = Extract(None);
        self.stream(extract.as_receiver()).ok()?;
        extract.0
    }

    #[inline]
    fn to_char(&self) -> Option<char> {
        struct Extract(Option<char>);

        impl<'a> DefaultUnsupported<'a> for Extract {
            fn char(&mut self, value: char) -> Result {
                self.0 = Some(value);
                Ok(())
            }
        }

        let mut extract = Extract(None);
        self.stream(extract.as_receiver()).ok()?;
        extract.0
    }

    #[inline]
    fn to_str(&self) -> Option<&str> {
        struct Extract<'a> {
            extracted: Option<&'a str>,
            seen_fragment: bool,
        }

        impl<'a> DefaultUnsupported<'a> for Extract<'a> {
            fn str(&mut self, value: &'a str) -> Result {
                // Allow either independent strings, or fragments of a single borrowed string
                if !self.seen_fragment {
                    self.extracted = Some(value);
                    self.seen_fragment = true;
                } else {
                    self.extracted = None;
                }

                Ok(())
            }

            fn text_begin(&mut self, _: Option<usize>) -> Result {
                Ok(())
            }

            fn text_fragment(&mut self, fragment: &'a str) -> Result {
                self.str(fragment)
            }

            fn text_fragment_computed(&mut self, _: &str) -> Result {
                self.extracted = None;
                self.seen_fragment = true;

                error::unsupported()
            }

            fn text_end(&mut self) -> Result {
                Ok(())
            }
        }

        let mut extract = Extract {
            extracted: None,
            seen_fragment: false,
        };

        self.stream(extract.as_receiver()).ok()?;
        extract.extracted
    }

    #[inline]
    fn to_bytes(&self) -> Option<&[u8]> {
        struct Extract<'a> {
            extracted: Option<&'a [u8]>,
            seen_fragment: bool,
        }

        impl<'a> DefaultUnsupported<'a> for Extract<'a> {
            fn bytes(&mut self, value: &'a [u8]) -> Result {
                // Allow either independent bytes, or fragments of a single borrowed byte stream
                if !self.seen_fragment {
                    self.extracted = Some(value);
                    self.seen_fragment = true;
                } else {
                    self.extracted = None;
                }

                Ok(())
            }

            fn binary_begin(&mut self, _: Option<usize>) -> Result {
                Ok(())
            }

            fn binary_fragment(&mut self, fragment: &'a [u8]) -> Result {
                self.bytes(fragment)
            }

            fn binary_fragment_computed(&mut self, _: &[u8]) -> Result {
                self.extracted = None;
                self.seen_fragment = true;

                error::unsupported()
            }

            fn binary_end(&mut self) -> Result {
                Ok(())
            }
        }

        let mut extract = Extract {
            extracted: None,
            seen_fragment: false,
        };

        self.stream(extract.as_receiver()).ok()?;
        extract.extracted
    }
}

macro_rules! impl_value_forward {
    ({ $($r:tt)* } => $bind:ident => { $($forward:tt)* }) => {
        $($r)* {
            fn stream<'b, R: Receiver<'b>>(&'b self, receiver: R) -> Result {
                let $bind = self;
                ($($forward)*).stream(receiver)
            }

            #[inline]
            fn is_dynamic(&self) -> bool {
                let $bind = self;
                ($($forward)*).is_dynamic()
            }

            #[inline]
            fn to_bool(&self) -> Option<bool> {
                let $bind = self;
                ($($forward)*).to_bool()
            }

            #[inline]
            fn to_f32(&self) -> Option<f32> {
                let $bind = self;
                ($($forward)*).to_f32()
            }

            #[inline]
            fn to_f64(&self) -> Option<f64> {
                let $bind = self;
                ($($forward)*).to_f64()
            }

            #[inline]
            fn to_i8(&self) -> Option<i8> {
                let $bind = self;
                ($($forward)*).to_i8()
            }

            #[inline]
            fn to_i16(&self) -> Option<i16> {
                let $bind = self;
                ($($forward)*).to_i16()
            }

            #[inline]
            fn to_i32(&self) -> Option<i32> {
                let $bind = self;
                ($($forward)*).to_i32()
            }

            #[inline]
            fn to_i64(&self) -> Option<i64> {
                let $bind = self;
                ($($forward)*).to_i64()
            }

            #[inline]
            fn to_i128(&self) -> Option<i128> {
                let $bind = self;
                ($($forward)*).to_i128()
            }

            #[inline]
            fn to_u8(&self) -> Option<u8> {
                let $bind = self;
                ($($forward)*).to_u8()
            }

            #[inline]
            fn to_u16(&self) -> Option<u16> {
                let $bind = self;
                ($($forward)*).to_u16()
            }

            #[inline]
            fn to_u32(&self) -> Option<u32> {
                let $bind = self;
                ($($forward)*).to_u32()
            }

            #[inline]
            fn to_u64(&self) -> Option<u64> {
                let $bind = self;
                ($($forward)*).to_u64()
            }

            #[inline]
            fn to_u128(&self) -> Option<u128> {
                let $bind = self;
                ($($forward)*).to_u128()
            }

            #[inline]
            fn to_char(&self) -> Option<char> {
                let $bind = self;
                ($($forward)*).to_char()
            }

            #[inline]
            fn to_str(&self) -> Option<&str> {
                let $bind = self;
                ($($forward)*).to_str()
            }

            #[inline]
            fn to_bytes(&self) -> Option<&[u8]> {
                let $bind = self;
                ($($forward)*).to_bytes()
            }
        }
    };
}

impl_value_forward!({impl<'a, T: Value + ?Sized> Value for &'a T} => x => { **x });

#[cfg(feature = "alloc")]
mod alloc_support {
    use super::*;

    use crate::std::boxed::Box;

    impl_value_forward!({impl<T: Value + ?Sized> Value for Box<T>} => x => { **x });
}
