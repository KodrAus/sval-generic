use crate::{
    source::{self, ValueSource},
    Receiver, Result, Source, Value,
};

impl Value for char {
    fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> crate::Result {
        receiver.char(*self)
    }
}

impl<'a> Source<'a> for char {
    fn stream<'b, R: Receiver<'b>>(&mut self, receiver: R) -> crate::Result<source::Stream>
    where
        'a: 'b,
    {
        self.stream_to_end(receiver).map(|_| source::Stream::Done)
    }

    fn stream_to_end<'b, R: Receiver<'b>>(&mut self, mut receiver: R) -> crate::Result
    where
        'a: 'b,
    {
        receiver.char(*self)
    }
}

impl<'a> ValueSource<'a, char> for char {
    type Error = source::Impossible;

    #[inline]
    fn take(&mut self) -> Result<&char, source::TakeError<Self::Error>> {
        Ok(self)
    }
}

impl Value for str {
    fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> crate::Result {
        receiver.str(self)
    }
}

impl<'a> Source<'a> for str {
    fn stream<'b, R: Receiver<'b>>(&mut self, receiver: R) -> crate::Result<source::Stream>
    where
        'a: 'b,
    {
        self.stream_to_end(receiver).map(|_| source::Stream::Done)
    }

    fn stream_to_end<'b, R: Receiver<'b>>(&mut self, mut receiver: R) -> crate::Result
    where
        'a: 'b,
    {
        receiver.str(self)
    }
}

impl<'a> ValueSource<'a, str> for str {
    type Error = source::Impossible;

    #[inline]
    fn take(&mut self) -> Result<&str, source::TakeError<Self::Error>> {
        Ok(self)
    }
}

#[cfg(feature = "alloc")]
mod alloc_support {
    use super::*;

    use crate::{
        for_all,
        std::{borrow::Cow, mem, string::String},
    };

    impl Value for String {
        fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> crate::Result {
            receiver.str(&**self)
        }

        #[inline]
        fn to_str(&self) -> Option<&str> {
            Some(self)
        }
    }

    impl<'a> Source<'a> for String {
        fn stream<'b, R: Receiver<'b>>(&mut self, receiver: R) -> crate::Result<source::Stream>
        where
            'a: 'b,
        {
            self.stream_to_end(receiver).map(|_| source::Stream::Done)
        }

        fn stream_to_end<'b, R: Receiver<'b>>(&mut self, mut receiver: R) -> crate::Result
        where
            'a: 'b,
        {
            receiver.str(for_all(self))
        }
    }

    impl<'a> ValueSource<'a, str> for String {
        type Error = source::Impossible;

        #[inline]
        fn take(&mut self) -> Result<&str, source::TakeError<Self::Error>> {
            Ok(&**self)
        }

        #[inline]
        fn take_owned(&mut self) -> Result<String, source::TakeError<Self::Error>> {
            Ok(mem::take(self))
        }
    }

    impl<'a> ValueSource<'a, str> for &'a String {
        type Error = source::Impossible;

        #[inline]
        fn take(&mut self) -> Result<&str, source::TakeError<Self::Error>> {
            Ok(&**self)
        }
    }

    impl<'a> Value for Cow<'a, str> {
        fn stream<'b, R: Receiver<'b>>(&'b self, mut receiver: R) -> crate::Result {
            receiver.str(&**self)
        }

        #[inline]
        fn to_str(&self) -> Option<&str> {
            if let Cow::Borrowed(v) = self {
                Some(v)
            } else {
                None
            }
        }
    }

    impl<'a> Source<'a> for Cow<'a, str> {
        fn stream<'b, R: Receiver<'b>>(&mut self, receiver: R) -> crate::Result<source::Stream>
        where
            'a: 'b,
        {
            self.stream_to_end(receiver).map(|_| source::Stream::Done)
        }

        fn stream_to_end<'b, R: Receiver<'b>>(&mut self, mut receiver: R) -> crate::Result
        where
            'a: 'b,
        {
            match self {
                Cow::Borrowed(v) => receiver.str(v),
                Cow::Owned(v) => receiver.str(for_all(v)),
            }
        }
    }

    impl<'a> ValueSource<'a, str> for Cow<'a, str> {
        type Error = source::Impossible;

        #[inline]
        fn take(&mut self) -> Result<&str, source::TakeError<Self::Error>> {
            Ok(&**self)
        }

        #[inline]
        fn take_ref(&mut self) -> Result<&'a str, source::TakeRefError<&str, Self::Error>> {
            match self {
                Cow::Borrowed(v) => Ok(v),
                Cow::Owned(v) => Err(source::TakeRefError::from_value(v)),
            }
        }

        #[inline]
        fn take_owned(&mut self) -> Result<String, source::TakeError<Self::Error>> {
            Ok(mem::take(self).into_owned())
        }
    }
}
