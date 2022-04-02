use crate::{
    source,
    std::fmt::{self, Write},
    Receiver, Result, Source, Value,
};

pub fn text<T: fmt::Display>(text: T) -> Text<T> {
    Text::new(text)
}

#[derive(Clone, Copy)]
pub struct Text<T>(T);

impl<T> Text<T> {
    pub fn new(text: T) -> Self {
        Text(text)
    }

    pub fn by_ref(&self) -> Text<&T> {
        Text(&self.0)
    }

    pub fn by_mut(&mut self) -> Text<&mut T> {
        Text(&mut self.0)
    }

    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T: fmt::Display> Value for Text<T> {
    fn stream<'b, R: Receiver<'b>>(&'b self, receiver: R) -> Result {
        Text::new(&self.0).stream_to_end(receiver)
    }
}

impl<'a, T: fmt::Display> Source<'a> for Text<T> {
    fn stream_resume<'b, R: Receiver<'b>>(&mut self, receiver: R) -> Result<source::Resume>
    where
        'a: 'b,
    {
        self.stream_to_end(receiver).map(|_| source::Resume::Done)
    }

    fn stream_to_end<'b, R: Receiver<'b>>(&mut self, mut receiver: R) -> Result
    where
        'a: 'b,
    {
        struct Writer<R>(R);

        impl<'a, R: Receiver<'a>> Write for Writer<R> {
            fn write_str(&mut self, s: &str) -> fmt::Result {
                self.0.text_fragment_computed(s)?;

                Ok(())
            }
        }

        receiver.text_begin(None)?;
        write!(Writer(&mut receiver), "{}", self.0)?;
        receiver.text_end()
    }
}

impl Value for char {
    fn stream<'a, R: Receiver<'a>>(&'a self, receiver: R) -> Result {
        let mut value = *self;
        value.stream_to_end(receiver)
    }
}

impl<'a> Source<'a> for char {
    fn stream_resume<'b, R: Receiver<'b>>(&mut self, receiver: R) -> Result<source::Resume>
    where
        'a: 'b,
    {
        self.stream_to_end(receiver).map(|_| source::Resume::Done)
    }

    fn stream_to_end<'b, R: Receiver<'b>>(&mut self, mut receiver: R) -> Result
    where
        'a: 'b,
    {
        let mut buf = [0; 4];
        let value = &*self.encode_utf8(&mut buf);

        receiver.text_begin(Some(value.len()))?;
        receiver.text_fragment_computed(value)?;
        receiver.text_end()
    }
}

impl Value for str {
    fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> Result {
        receiver.text(self)
    }

    fn to_text(&self) -> Option<&str> {
        Some(self)
    }
}

#[cfg(feature = "alloc")]
mod alloc_support {
    use super::*;

    use crate::std::string::String;

    impl Value for String {
        fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> Result {
            receiver.text(&**self)
        }

        #[inline]
        fn to_text(&self) -> Option<&str> {
            Some(self)
        }
    }
}
