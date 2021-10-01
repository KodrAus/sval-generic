pub mod fmt;
pub mod serde;
pub mod value;
pub mod stream;

pub mod erased;

mod for_all;
mod impls;
mod reference;
mod tag;

pub use sval_generic_api_derive::*;

#[doc(inline)]
pub use self::{value::Value, stream::Stream};

#[derive(Debug)]
pub struct Error;

impl From<std::fmt::Error> for Error {
    fn from(_: std::fmt::Error) -> Error {
        Error
    }
}

pub type Result<T = (), E = Error> = std::result::Result<T, E>;

pub fn stream_value<'a>(s: impl Stream<'a>, v: impl reference::ValueRef<'a>) -> Result {
    v.stream(s)
}

#[cfg(test)]
mod tests {
    use crate::{
        value::{self, Value},
        stream::{self, Stream},
    };

    #[test]
    fn it_works() {
        struct MyValue;

        impl Value for MyValue {
            fn stream<'a, S: value::Stream<'a>>(&'a self, mut stream: S) -> value::Result {
                let mut short = |s: &str| {
                    stream.map_field("field")?;
                    stream.map_value(s.for_all())
                };

                short("value")
            }
        }

        struct MyStruct {
            a: String,
            b: i64,
        }

        impl Value for MyStruct {
            fn stream<'a, S: value::Stream<'a>>(&'a self, mut stream: S) -> value::Result {
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
            fn stream<'b, S: value::Stream<'b>>(&'b self, mut stream: S) -> value::Result {
                stream.map_begin(Some(1))?;
                stream.map_field("a")?;
                stream.map_value(self.a)?;
                stream.map_field("b")?;
                stream.map_value(self.b)?;
                stream.map_end()
            }
        }

        struct MyStream<'a>(Option<&'a str>);

        impl<'a> Stream<'a> for MyStream<'a> {
            fn display<V: stream::Display>(&mut self, _: V) -> stream::Result {
                Ok(())
            }

            fn none(&mut self) -> stream::Result {
                Ok(())
            }

            fn str<'v, V: stream::TypedRef<'v, str>>(&mut self, v: V) -> stream::Result
            where
                'v: 'a,
            {
                if let Some(v) = v.try_unwrap() {
                    println!("borrowed: {}", v);
                } else {
                    println!("short: {}", v.get());
                }

                Ok(())
            }

            fn map_begin(&mut self, _: Option<usize>) -> stream::Result {
                Ok(())
            }

            fn map_end(&mut self) -> stream::Result {
                Ok(())
            }

            fn map_key_begin(&mut self) -> stream::Result {
                Ok(())
            }

            fn map_key_end(&mut self) -> stream::Result {
                Ok(())
            }

            fn map_value_begin(&mut self) -> stream::Result {
                Ok(())
            }

            fn map_value_end(&mut self) -> stream::Result {
                Ok(())
            }

            fn seq_begin(&mut self, _: Option<usize>) -> stream::Result {
                Ok(())
            }

            fn seq_end(&mut self) -> stream::Result {
                Ok(())
            }

            fn seq_elem_begin(&mut self) -> stream::Result {
                Ok(())
            }

            fn seq_elem_end(&mut self) -> stream::Result {
                Ok(())
            }
        }

        MyValue.stream(MyStream(None)).unwrap();

        let my_struct = MyStruct {
            a: String::from("hello!"),
            b: 42,
        };
        my_struct.stream(MyStream(None)).unwrap();

        let erased_value = MyStruct {
            a: String::from("hello!"),
            b: 42,
        };
        let erased_value = erased_value.erase();

        let mut erased_stream = MyStream(None);
        let mut erased_stream = erased_stream.erase();

        erased_value.stream(MyStream(None)).unwrap();

        MyValue.stream(&mut erased_stream).unwrap();

        erased_value.stream(&mut erased_stream).unwrap();
    }
}
