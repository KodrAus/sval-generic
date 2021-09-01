use std::fmt;

mod for_all;
mod impls;
pub mod stream;
pub mod value;
mod value_ref;

pub mod erased;
pub mod serde;

pub use sval_generic_api_derive::*;

#[doc(inline)]
pub use self::{stream::Stream, value::Value};

#[derive(Debug)]
pub struct Error;

impl From<fmt::Error> for Error {
    fn from(_: fmt::Error) -> Error {
        Error
    }
}

pub type Result<T = (), E = Error> = std::result::Result<T, E>;

pub fn stream<'a>(s: impl Stream<'a>, v: impl stream::UnknownValueRef<'a>) -> Result {
    v.stream(s)
}

#[cfg(test)]
mod tests {
    use crate::{
        stream::{self, Stream},
        value::{self, Value},
    };

    #[test]
    fn it_works() {
        struct MyValue;

        impl Value for MyValue {
            fn stream<'a, S>(&'a self, mut stream: S) -> value::Result
            where
                S: value::Stream<'a>,
            {
                let mut short = |s: &str| stream.map_field("field", s.for_all());

                short("value")
            }
        }

        struct MyStruct {
            a: String,
            b: i64,
        }

        impl Value for MyStruct {
            fn stream<'a, S>(&'a self, mut stream: S) -> value::Result
            where
                S: value::Stream<'a>,
            {
                stream.map_begin(Some(1))?;
                stream.map_field("a", &self.a)?;
                stream.map_field("b", self.b)?;
                stream.map_end()
            }
        }

        struct MyStream<'a>(Option<&'a str>);

        impl<'a> Stream<'a> for MyStream<'a> {
            fn u64(&mut self, _: u64) -> stream::Result {
                Ok(())
            }

            fn i64(&mut self, _: i64) -> stream::Result {
                Ok(())
            }

            fn u128(&mut self, _: u128) -> stream::Result {
                Ok(())
            }

            fn i128(&mut self, _: i128) -> stream::Result {
                Ok(())
            }

            fn f64(&mut self, _: f64) -> stream::Result {
                Ok(())
            }

            fn bool(&mut self, _: bool) -> stream::Result {
                Ok(())
            }

            fn none(&mut self) -> stream::Result {
                Ok(())
            }

            fn str<'v, V: stream::TypedValueRef<'v, str>>(&mut self, v: V) -> stream::Result
            where
                'v: 'a,
            {
                if let Some(v) = v.get_ref() {
                    println!("borrowed: {}", v);
                } else {
                    println!("short: {}", v.get());
                }

                Ok(())
            }

            fn map_begin(&mut self, _: Option<usize>) -> stream::Result {
                Ok(())
            }

            fn map_key_begin(&mut self) -> stream::Result {
                Ok(())
            }

            fn map_value_begin(&mut self) -> stream::Result {
                Ok(())
            }

            fn map_end(&mut self) -> stream::Result {
                Ok(())
            }

            fn seq_begin(&mut self, _: Option<usize>) -> stream::Result {
                Ok(())
            }

            fn seq_elem_begin(&mut self) -> stream::Result {
                Ok(())
            }

            fn seq_end(&mut self) -> stream::Result {
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
