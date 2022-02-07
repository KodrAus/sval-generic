use crate::{
    source,
    std::fmt::{self, Display},
    Receiver, Source, Value,
};

#[inline]
pub fn text<T: Display>(value: T) -> Text<T> {
    Text::new(value)
}

pub struct Text<T>(T);

impl<T: Display> Display for Text<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<T: Display> Value for Text<T> {
    fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> crate::Result {
        receiver.text_begin(None)?;
        receiver.text_fragment(&self.0)?;
        receiver.text_end()
    }
}

impl<'a, T: Display> Source<'a> for Text<T> {
    fn stream_resume<'b, R: Receiver<'b>>(&mut self, receiver: R) -> crate::Result<source::Resume>
    where
        'a: 'b,
    {
        self.stream_to_end(receiver).map(|_| source::Resume::Done)
    }

    fn stream_to_end<'b, R: Receiver<'b>>(&mut self, mut receiver: R) -> crate::Result
    where
        'a: 'b,
    {
        receiver.text_begin(None)?;
        receiver.text_fragment(&self.0)?;
        receiver.text_end()
    }
}

impl<T> Text<T> {
    pub fn new(value: T) -> Self {
        Text(value)
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

impl Value for char {
    fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> crate::Result {
        receiver.char(*self)
    }
}

impl<'a> Source<'a> for char {
    fn stream_resume<'b, R: Receiver<'b>>(&mut self, receiver: R) -> crate::Result<source::Resume>
    where
        'a: 'b,
    {
        self.stream_to_end(receiver).map(|_| source::Resume::Done)
    }

    fn stream_to_end<'b, R: Receiver<'b>>(&mut self, mut receiver: R) -> crate::Result
    where
        'a: 'b,
    {
        receiver.char(*self)
    }
}

impl Value for str {
    fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> crate::Result {
        receiver.str(self)
    }
}

#[cfg(feature = "alloc")]
mod alloc_support {
    use super::*;

    use crate::std::string::String;

    impl Value for String {
        fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> crate::Result {
            receiver.str(&**self)
        }

        #[inline]
        fn to_str(&self) -> Option<&str> {
            Some(self)
        }
    }
}
