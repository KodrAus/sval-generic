#[macro_use]
extern crate async_trait;

pub mod buffer;
pub mod fmt;
pub mod receiver;
pub mod serde;
pub mod source;
pub mod tag;
pub mod valuable;
pub mod value;

pub mod erased;

mod for_all;
mod impls;

pub use sval_generic_api_derive::*;

#[doc(inline)]
pub use self::{for_all::ForAll, receiver::Receiver, source::Source, value::Value};

#[derive(Debug)]
pub struct Error;

impl From<std::fmt::Error> for Error {
    fn from(_: std::fmt::Error) -> Error {
        Error
    }
}

pub type Result<T = (), E = Error> = std::result::Result<T, E>;

pub fn stream<'a>(s: impl Receiver<'a>, mut v: impl Source<'a>) -> Result {
    v.stream(s)
}

/*
pub async fn stream_non_blocking<'a>(s: impl AsyncStream<'a>, v: impl AsyncSource<'a>) -> Result {
    v.stream(s).await
}
*/

pub fn for_all<T>(value: T) -> ForAll<T> {
    ForAll::new(value)
}

#[cfg(test)]
mod tests {
    use crate::{erased, receiver::Display, source::ValueSource, Receiver, Value};

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

        let erased_value = MyStruct {
            a: String::from("hello!"),
            b: 42,
        };
        let erased_value = erased::value(&erased_value);

        let mut erased_stream = MyReceiver(None);
        let mut erased_stream = erased::receiver(&mut erased_stream);

        erased_value.stream(MyReceiver(None)).unwrap();

        MyValue.stream(&mut erased_stream).unwrap();

        erased_value.stream(&mut erased_stream).unwrap();
    }
}
