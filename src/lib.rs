#![feature(generic_associated_types)]

pub mod receiver;
pub mod source;
pub mod tag;
pub mod value;

pub mod generator;

mod for_all;
mod impls;

pub use sval_generic_api_derive::*;

#[doc(inline)]
pub use self::{for_all::ForAll, receiver::Receiver, source::Source, value::Value};

#[derive(Debug)]
pub struct Error;

impl From<std::fmt::Error> for Error {
    #[inline]
    fn from(_: std::fmt::Error) -> Error {
        Error
    }
}

pub type Result<T = (), E = Error> = std::result::Result<T, E>;

pub fn stream<'a>(s: impl Receiver<'a>, mut v: impl Source<'a>) -> Result {
    v.stream(s)
}

pub fn for_all<T>(value: T) -> ForAll<T> {
    ForAll::new(value)
}

#[cfg(test)]
mod tests {
    use crate::{receiver::Display, source::ValueSource, Receiver, Value};

    #[test]
    fn it_works() {
        struct MyValue;

        impl Value for MyValue {
            fn stream<'a, S: Receiver<'a>>(&'a self, mut stream: S) -> crate::Result {
                let mut short = |s: &str| {
                    stream.map_field("field")?;
                    stream.map_value(crate::for_all(s))
                };

                short("value")
            }
        }

        struct MyStruct {
            a: String,
            b: i64,
        }

        impl Value for MyStruct {
            fn stream<'a, S: Receiver<'a>>(&'a self, mut stream: S) -> crate::Result {
                stream.map_begin(Some(1))?;
                stream.map_field("a")?;
                stream.map_value(&self.a)?;
                stream.map_field("b")?;
                stream.map_value(self.b)?;
                stream.map_end()
            }
        }

        struct MyInnerRef<'a> {
            a: &'a str,
            b: i64,
        }

        impl<'a> Value for MyInnerRef<'a> {
            fn stream<'b, S: Receiver<'b>>(&'b self, mut stream: S) -> crate::Result {
                stream.map_begin(Some(1))?;
                stream.map_field("a")?;
                stream.map_value(self.a)?;
                stream.map_field("b")?;
                stream.map_value(self.b)?;
                stream.map_end()
            }
        }

        struct MyReceiver<'a>(Option<&'a str>);

        impl<'a> Receiver<'a> for MyReceiver<'a> {
            fn display<V: Display>(&mut self, _: V) -> crate::Result {
                Ok(())
            }

            fn none(&mut self) -> crate::Result {
                Ok(())
            }

            fn str<'v, V: ValueSource<'v, str>>(&mut self, mut value: V) -> crate::Result
            where
                'v: 'a,
            {
                match value.value_ref() {
                    Ok(v) => println!("borrowed: {}", v),
                    Err(v) => println!("short: {}", v.into_result().unwrap()),
                }

                Ok(())
            }

            fn map_begin(&mut self, _: Option<usize>) -> crate::Result {
                Ok(())
            }

            fn map_end(&mut self) -> crate::Result {
                Ok(())
            }

            fn map_key_begin(&mut self) -> crate::Result {
                Ok(())
            }

            fn map_key_end(&mut self) -> crate::Result {
                Ok(())
            }

            fn map_value_begin(&mut self) -> crate::Result {
                Ok(())
            }

            fn map_value_end(&mut self) -> crate::Result {
                Ok(())
            }

            fn seq_begin(&mut self, _: Option<usize>) -> crate::Result {
                Ok(())
            }

            fn seq_end(&mut self) -> crate::Result {
                Ok(())
            }

            fn seq_elem_begin(&mut self) -> crate::Result {
                Ok(())
            }

            fn seq_elem_end(&mut self) -> crate::Result {
                Ok(())
            }
        }

        MyValue.stream(MyReceiver(None)).unwrap();

        let my_struct = MyStruct {
            a: String::from("hello!"),
            b: 42,
        };
        my_struct.stream(MyReceiver(None)).unwrap();
    }
}
