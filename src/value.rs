use crate::{
    erased,
    value_ref::TypedValueRef,
};

#[doc(inline)]
pub use crate::{for_all::ForAll, stream::Stream, Error, Result};

pub trait Value {
    fn stream<'a, S>(&'a self, stream: S) -> Result
    where
        S: Stream<'a>;

    fn to_str(&self) -> Option<&str> {
        struct Extract<'a>(Option<&'a str>);

        impl<'a> Stream<'a> for Extract<'a> {
            fn u128(&mut self, _: u128) -> Result {
                Err(Error)
            }

            fn i128(&mut self, _: i128) -> Result {
                Err(Error)
            }

            fn map_begin(&mut self, _: Option<usize>) -> Result {
                Err(Error)
            }

            fn map_key_begin(&mut self) -> Result {
                Err(Error)
            }

            fn map_value_begin(&mut self) -> Result {
                Err(Error)
            }

            fn map_end(&mut self) -> Result {
                Err(Error)
            }

            fn str<'v, V: TypedValueRef<'v, str>>(&mut self, v: V) -> Result
            where
                'v: 'a,
            {
                match v.to_ref() {
                    Some(v) => {
                        self.0 = Some(v);
                        Ok(())
                    }
                    _ => Err(Error),
                }
            }
        }

        let mut stream = Extract(None);
        self.stream(&mut stream).ok()?;
        stream.0
    }

    fn for_all(&self) -> ForAll<&Self> {
        ForAll(self)
    }

    fn erase<'a>(&'a self) -> erased::Value<'a>
    where
        Self: Sized,
    {
        erased::Value::new(self)
    }
}

impl<'a, T: ?Sized> Value for &'a T
where
    T: Value,
{
    fn stream<'b, S>(&'b self, stream: S) -> Result
    where
        S: Stream<'b>,
    {
        (**self).stream(stream)
    }

    fn to_str(&self) -> Option<&str> {
        (**self).to_str()
    }
}
