#![cfg_attr(not(test), no_std)]

#[cfg(feature = "std")]
extern crate std;

#[cfg(all(feature = "alloc", not(feature = "std")))]
mod std {
    pub use alloc::*;
    pub use core::*;
}

#[cfg(all(not(feature = "alloc"), not(feature = "std")))]
extern crate core as std;

pub mod receiver;
pub mod source;
pub mod tag;
pub mod value;

mod for_all;
mod impls;
mod num;

#[cfg(feature = "derive")]
pub use derive::*;

#[doc(inline)]
pub use self::{
    for_all::{for_all, ForAll},
    receiver::Receiver,
    source::{stream_to_end, Source},
    value::{stream, Value},
};

#[derive(Debug)]
pub struct Error;

impl From<std::fmt::Error> for Error {
    #[inline]
    fn from(_: std::fmt::Error) -> Error {
        Error
    }
}

pub type Result<T = (), E = Error> = std::result::Result<T, E>;

#[cfg(test)]
mod tests {
    mod my_value {
        use crate::value::{self, Value};

        pub struct MyValue;

        impl Value for MyValue {
            fn stream<'a, S: value::Receiver<'a>>(&'a self, mut receiver: S) -> value::Result {
                let mut short = |s: &str| {
                    receiver.map_field("field")?;
                    receiver.map_value(value::for_all(s))
                };

                short("value")
            }
        }

        pub struct MyStruct {
            pub a: String,
            pub b: i64,
        }

        impl Value for MyStruct {
            fn stream<'a, S: value::Receiver<'a>>(&'a self, mut receiver: S) -> value::Result {
                receiver.type_tagged_map_begin(value::type_tag("MyStruct"), Some(1))?;
                receiver.map_field_entry("a", &self.a)?;
                receiver.map_field_entry("b", &self.b)?;
                receiver.type_tagged_map_end()
            }
        }

        pub struct MyInnerRef<'a> {
            pub a: &'a str,
            pub b: i64,
        }

        impl<'a> Value for MyInnerRef<'a> {
            fn stream<'b, S: value::Receiver<'b>>(&'b self, mut receiver: S) -> value::Result {
                receiver.map_begin(Some(1))?;
                receiver.map_field("a")?;
                receiver.map_value(self.a)?;
                receiver.map_field("b")?;
                receiver.map_value(self.b)?;
                receiver.map_end()
            }
        }
    }

    mod my_source {
        use crate::source::{self, Source};

        pub struct MySource<'a> {
            pub remaining: Vec<&'a str>,
            pub current: Option<&'a str>,
        }

        impl<'a> Source<'a> for MySource<'a> {
            fn stream<'b, R: source::Receiver<'b>>(
                &mut self,
                receiver: R,
            ) -> source::Result<source::Stream>
            where
                'a: 'b,
            {
                if let Some(ref mut current) = self.current {
                    match current.stream(receiver)? {
                        source::Stream::Yield => return Ok(source::Stream::Yield),
                        source::Stream::Done => self.current = None,
                    }
                }

                if let Some(next) = self.remaining.pop() {
                    self.current = Some(next);

                    return Ok(source::Stream::Yield);
                }

                Ok(source::Stream::Done)
            }
        }
    }

    mod my_receiver {
        use crate::receiver::{self, Receiver};

        pub struct MyReceiver<'a>(pub Option<&'a str>);

        impl<'a> Receiver<'a> for MyReceiver<'a> {
            fn display<V: receiver::Display>(&mut self, _: V) -> receiver::Result {
                Ok(())
            }

            fn none(&mut self) -> receiver::Result {
                Ok(())
            }

            fn str<'v, V: receiver::ValueSource<'v, str>>(
                &mut self,
                mut value: V,
            ) -> receiver::Result
            where
                'v: 'a,
            {
                match value.take_ref() {
                    Ok(v) => println!("borrowed: {}", v),
                    Err(v) => println!("short: {}", v.into_result().unwrap()),
                }

                Ok(())
            }

            fn map_begin(&mut self, _: Option<usize>) -> receiver::Result {
                Ok(())
            }

            fn map_end(&mut self) -> receiver::Result {
                Ok(())
            }

            fn map_key_begin(&mut self) -> receiver::Result {
                Ok(())
            }

            fn map_key_end(&mut self) -> receiver::Result {
                Ok(())
            }

            fn map_value_begin(&mut self) -> receiver::Result {
                Ok(())
            }

            fn map_value_end(&mut self) -> receiver::Result {
                Ok(())
            }

            fn seq_begin(&mut self, _: Option<usize>) -> receiver::Result {
                Ok(())
            }

            fn seq_end(&mut self) -> receiver::Result {
                Ok(())
            }

            fn seq_elem_begin(&mut self) -> receiver::Result {
                Ok(())
            }

            fn seq_elem_end(&mut self) -> receiver::Result {
                Ok(())
            }
        }
    }

    #[test]
    fn it_works() {
        use crate::{Source, Value};

        use self::{my_receiver::*, my_source::*, my_value::*};

        MyValue.stream(MyReceiver(None)).unwrap();

        let my_struct = MyStruct {
            a: String::from("hello!"),
            b: 42,
        };
        my_struct.stream(MyReceiver(None)).unwrap();

        let mut my_source = MySource {
            current: None,
            remaining: vec!["a", "b", "c"],
        };
        my_source.stream_to_end(MyReceiver(None)).unwrap();
    }
}
