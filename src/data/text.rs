use crate::{
    source::{self, TakeError, ValueSource},
    std::fmt,
    Receiver, Result, Source, Value,
};

pub fn text(text: &impl fmt::Display) -> &Text {
    Text::new(text)
}

#[repr(transparent)]
pub struct Text(dyn fmt::Display);

impl Text {
    pub fn new(text: &impl fmt::Display) -> &Text {
        // SAFETY: `Text` and `dyn Display` have the same ABI
        unsafe { &*(text as *const dyn fmt::Display as *const Text) }
    }
}

impl fmt::Display for Text {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Value for Text {
    fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> Result {
        receiver.text(self)
    }
}

impl Value for char {
    fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> crate::Result {
        receiver.char(*self)
    }
}

impl<'a> Source<'a> for char {
    fn stream_resume<'b, R: Receiver<'b>>(&mut self, receiver: R) -> crate::Result<source::Stream>
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

impl<'a> ValueSource<'a, Text> for &'a char {
    type Error = source::Impossible;

    #[inline]
    fn take(&mut self) -> Result<&Text, source::TakeError<Self::Error>> {
        Ok(Text::new(*self))
    }

    #[inline]
    fn try_take_ref(&mut self) -> Result<&'a Text, source::TryTakeError<&Text, Self::Error>> {
        Ok(Text::new(*self))
    }
}

impl<'a> ValueSource<'a, Text> for &'a str {
    type Error = source::Impossible;

    #[inline]
    fn take(&mut self) -> Result<&Text, TakeError<Self::Error>> {
        Ok(Text::new(self))
    }
}

#[cfg(feature = "alloc")]
mod alloc_support {
    use super::*;

    use crate::source::TryTakeError;
    use crate::{
        for_all,
        std::{
            borrow::{Borrow, Cow, ToOwned},
            mem,
            string::{String, ToString},
        },
    };

    impl ToOwned for Text {
        type Owned = String;

        fn to_owned(&self) -> Self::Owned {
            self.0.to_string()
        }
    }

    impl Borrow<Text> for String {
        fn borrow(&self) -> &Text {
            Text::new(self)
        }
    }

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
        fn stream_resume<'b, R: Receiver<'b>>(
            &mut self,
            receiver: R,
        ) -> crate::Result<source::Stream>
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
        fn take_owned(&mut self) -> Result<Cow<str>, source::TakeError<Self::Error>> {
            Ok(Cow::Owned(mem::take(self)))
        }

        #[inline]
        fn try_take_owned(&mut self) -> Result<&'a str, TryTakeError<Cow<str>, Self::Error>> {
            Err(TryTakeError::Fallback(Cow::Owned(mem::take(self))))
        }
    }

    impl<'a> ValueSource<'a, str> for &'a String {
        type Error = source::Impossible;

        #[inline]
        fn take(&mut self) -> Result<&str, source::TakeError<Self::Error>> {
            Ok(&**self)
        }

        #[inline]
        fn try_take_ref(&mut self) -> Result<&'a str, TryTakeError<&str, Self::Error>> {
            Ok(&**self)
        }
    }

    impl<'a> ValueSource<'a, Text> for String {
        type Error = source::Impossible;

        #[inline]
        fn take(&mut self) -> Result<&Text, source::TakeError<Self::Error>> {
            Ok(Text::new(&*self))
        }

        #[inline]
        fn take_owned(&mut self) -> Result<Cow<Text>, source::TakeError<Self::Error>> {
            Ok(Cow::Owned(mem::take(self)))
        }

        #[inline]
        fn try_take_owned(&mut self) -> Result<&'a Text, TryTakeError<Cow<Text>, Self::Error>> {
            Err(TryTakeError::Fallback(Cow::Owned(mem::take(self))))
        }
    }

    impl<'a> ValueSource<'a, Text> for &'a String {
        type Error = source::Impossible;

        #[inline]
        fn take(&mut self) -> Result<&Text, source::TakeError<Self::Error>> {
            Ok(Text::new(&**self))
        }

        #[inline]
        fn try_take_ref(&mut self) -> Result<&'a Text, TryTakeError<&Text, Self::Error>> {
            Ok(Text::new(&**self))
        }
    }
}
