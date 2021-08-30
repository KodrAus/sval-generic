use crate::value_ref::{AnyValueRef, TypedValueRef};

pub use crate::{stream::Stream, value_ref::ForAll, Error, Result};

pub trait Value {
    fn stream<'a, S>(&'a self, stream: S) -> Result
    where
        S: Stream<'a>;

    // TODO: Can we move this back to `ForAll`?
    fn stream_for_all<'a, S>(&self, stream: S) -> Result
    where
        S: Stream<'a>,
    {
        struct AnyStream<S>(S);

        impl<'a, 'b, S> Stream<'a> for AnyStream<S>
        where
            S: Stream<'b>,
        {
            fn u128(&mut self, v: u128) -> Result {
                self.0.u128(v)
            }

            fn i128(&mut self, v: i128) -> Result {
                self.0.i128(v)
            }

            fn str<'v, V: TypedValueRef<'v, str>>(&mut self, v: V) -> Result
            where
                'v: 'a,
            {
                self.0.str(ForAll(v))
            }

            fn map_begin(&mut self, len: Option<usize>) -> Result {
                self.0.map_begin(len)
            }

            fn map_key_begin(&mut self) -> Result {
                self.0.map_key_begin()
            }

            fn map_value_begin(&mut self) -> Result {
                self.0.map_value_begin()
            }

            fn map_end(&mut self) -> Result {
                self.0.map_end()
            }

            fn map_key<'k, K: AnyValueRef<'k>>(&mut self, k: K) -> Result
            where
                'k: 'a,
            {
                self.0.map_key(ForAll(k))
            }

            fn map_value<'v, V: AnyValueRef<'v>>(&mut self, v: V) -> Result
            where
                'v: 'a,
            {
                self.0.map_value(ForAll(v))
            }

            fn map_entry<'k, 'v, K: AnyValueRef<'k>, V: AnyValueRef<'v>>(&mut self, k: K, v: V) -> Result
            where
                'k: 'a,
                'v: 'a,
            {
                self.0.map_entry(ForAll(k), ForAll(v))
            }

            fn map_field<'v, F: TypedValueRef<'static, str>, V: AnyValueRef<'v>>(
                &mut self,
                f: F,
                v: V,
            ) -> Result
            where
                'v: 'a,
            {
                self.0.map_field(f, ForAll(v))
            }
        }

        self.stream(AnyStream(stream))
    }

    fn for_all(&self) -> ForAll<&Self> {
        ForAll(self)
    }

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

    fn stream_for_all<'b, S>(&self, stream: S) -> Result
    where
        S: Stream<'b>,
    {
        (**self).stream_for_all(stream)
    }
}
