#[macro_use]
extern crate async_trait;

pub mod fmt;
pub mod serde;
pub mod source;
pub mod stream;
pub mod tag;
pub mod value;

pub mod erased;

mod for_all;
mod impls;

pub use sval_generic_api_derive::*;

#[doc(inline)]
pub use self::{for_all::ForAll, source::Source, stream::Stream, value::Value};

#[derive(Debug)]
pub struct Error;

impl From<std::fmt::Error> for Error {
    fn from(_: std::fmt::Error) -> Error {
        Error
    }
}

pub type Result<T = (), E = Error> = std::result::Result<T, E>;

pub fn stream<'a>(s: impl Stream<'a>, mut v: impl Source<'a>) -> Result {
    v.stream(s)
}

/*
pub async fn stream_non_blocking<'a>(s: impl AsyncStream<'a>, v: impl AsyncSource<'a>) -> Result {
    v.stream(s).await
}
*/

#[cfg(test)]
mod tests {
    use crate::{source::TypedSource, stream::Display, Stream, Value};

    #[test]
    fn it_works() {
        struct MyValue;

        impl Value for MyValue {
            fn stream<'a, S: Stream<'a>>(&'a self, mut stream: S) -> crate::Result {
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
            fn stream<'a, S: Stream<'a>>(&'a self, mut stream: S) -> crate::Result {
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
            fn stream<'b, S: Stream<'b>>(&'b self, mut stream: S) -> crate::Result {
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
            fn display<V: Display>(&mut self, _: V) -> crate::Result {
                Ok(())
            }

            fn none(&mut self) -> crate::Result {
                Ok(())
            }

            fn str<'v, V: TypedSource<'v, str>>(&mut self, mut value: V) -> crate::Result
            where
                'v: 'a,
            {
                match value.stream_to_ref() {
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
