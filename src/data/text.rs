use crate::{
    std::fmt::{self, Write},
    Result, Stream, Value,
};

impl Value for char {
    fn stream<'sval, S: Stream<'sval>>(&'sval self, mut stream: S) -> Result {
        let mut buf = [0; 4];
        let value = &*self.encode_utf8(&mut buf);

        stream.text_begin(Some(value.len()))?;
        stream.text_fragment_computed(value)?;
        stream.text_end()
    }
}

impl Value for str {
    fn stream<'sval, S: Stream<'sval>>(&'sval self, mut stream: S) -> Result {
        stream.text_begin(Some(self.len()))?;
        stream.text_fragment(self)?;
        stream.text_end()
    }

    fn to_text(&self) -> Option<&str> {
        Some(self)
    }
}

pub(crate) fn display<'sval, T: fmt::Display>(text: T, mut stream: impl Stream<'sval>) -> Result {
    struct Writer<S>(S);

    impl<'a, S: Stream<'a>> Write for Writer<S> {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.0.text_fragment_computed(s)?;

            Ok(())
        }
    }

    stream.text_begin(None)?;
    write!(Writer(&mut stream), "{}", text)?;
    stream.text_end()
}

#[cfg(feature = "alloc")]
mod alloc_support {
    use super::*;

    use crate::std::string::String;

    impl Value for String {
        fn stream<'a, S: Stream<'a>>(&'a self, stream: S) -> Result {
            (&**self).stream(stream)
        }

        #[inline]
        fn to_text(&self) -> Option<&str> {
            Some(self)
        }
    }
}
