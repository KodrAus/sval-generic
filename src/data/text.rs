use crate::{
    data, source,
    std::fmt::{self, Write},
    Receiver, Result, Source, Value,
};

impl Value for char {
    fn stream<'a, R: Receiver<'a>>(&'a self, receiver: R) -> Result {
        { *self }.stream_to_end(receiver)
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

    fn maybe_dynamic(&self) -> Option<bool> {
        Some(false)
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

pub fn display<T: fmt::Display>(text: T) -> Display<T> {
    Display::new(text)
}

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Display<T: ?Sized>(T);

impl<T> Display<T> {
    pub fn new(text: T) -> Self {
        Display(text)
    }

    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T: ?Sized> Display<T> {
    pub fn by_ref(&self) -> Display<&T> {
        Display(&self.0)
    }

    pub fn by_mut(&mut self) -> Display<&mut T> {
        Display(&mut self.0)
    }
}

impl<T: fmt::Display> Value for Display<T> {
    fn stream<'data, R: Receiver<'data>>(&'data self, mut receiver: R) -> Result {
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

impl<'a, T: fmt::Display> Source<'a> for Display<T> {
    fn stream_resume<'b, R: Receiver<'b>>(&mut self, receiver: R) -> Result<source::Resume>
    where
        'a: 'b,
    {
        self.stream_to_end(receiver).map(|_| source::Resume::Done)
    }

    fn stream_to_end<'b, R: Receiver<'b>>(&mut self, receiver: R) -> Result
    where
        'a: 'b,
    {
        self.stream(data::computed(receiver))
    }

    fn maybe_dynamic(&self) -> Option<bool> {
        Some(false)
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
