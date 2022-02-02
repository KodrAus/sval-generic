use crate::{
    source::{self, SourceRef, TakeError},
    std::fmt,
    Receiver, Result, Source, SourceValue,
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

impl SourceValue for Text {
    fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> Result {
        receiver.text(self)
    }
}

impl SourceValue for char {
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

impl SourceValue for str {
    fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> crate::Result {
        receiver.str(self)
    }
}

impl<'a> SourceRef<'a, Text> for &'a char {
    type Error = source::Impossible;

    #[inline]
    fn take(&mut self) -> Result<&Text, source::TakeError<Self::Error>> {
        Ok(Text::new(*self))
    }

    #[inline]
    fn try_take(&mut self) -> Result<&'a Text, source::TryTakeError<&Text, Self::Error>> {
        Ok(Text::new(*self))
    }
}

impl<'a> SourceRef<'a, Text> for &'a str {
    type Error = source::Impossible;

    #[inline]
    fn take(&mut self) -> Result<&Text, TakeError<Self::Error>> {
        Ok(Text::new(self))
    }
}

#[cfg(feature = "alloc")]
mod alloc_support {
    use super::*;

    use crate::{
        for_all, source,
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

    impl SourceValue for String {
        fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> crate::Result {
            receiver.str(&**self)
        }

        #[inline]
        fn to_str(&self) -> Option<&str> {
            Some(self)
        }
    }

    impl<'a> Source<'a> for String {
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
            receiver.str(mem::take(self))
        }
    }

    impl<'a> SourceRef<'a, str> for String {
        type Error = source::Impossible;

        #[inline]
        fn take(&mut self) -> Result<&str, source::TakeError<Self::Error>> {
            Ok(&**self)
        }

        #[inline]
        fn take_owned(&mut self) -> Result<String, source::TakeError<Self::Error>> {
            Ok(mem::take(self))
        }

        #[inline]
        fn try_take_owned(&mut self) -> Result<&'a str, source::TryTakeError<String, Self::Error>> {
            Err(source::TryTakeError::Fallback(mem::take(self)))
        }
    }

    impl<'a> SourceRef<'a, str> for &'a String {
        type Error = source::Impossible;

        #[inline]
        fn take(&mut self) -> Result<&str, source::TakeError<Self::Error>> {
            Ok(&**self)
        }

        #[inline]
        fn try_take(&mut self) -> Result<&'a str, source::TryTakeError<&str, Self::Error>> {
            Ok(&**self)
        }
    }

    impl<'a> SourceRef<'a, Text> for String {
        type Error = source::Impossible;

        #[inline]
        fn take(&mut self) -> Result<&Text, source::TakeError<Self::Error>> {
            Ok(Text::new(&*self))
        }

        #[inline]
        fn take_owned(&mut self) -> Result<String, source::TakeError<Self::Error>> {
            Ok(mem::take(self))
        }

        #[inline]
        fn try_take_owned(
            &mut self,
        ) -> Result<&'a Text, source::TryTakeError<String, Self::Error>> {
            Err(source::TryTakeError::Fallback(mem::take(self)))
        }
    }

    impl<'a> SourceRef<'a, Text> for &'a String {
        type Error = source::Impossible;

        #[inline]
        fn take(&mut self) -> Result<&Text, source::TakeError<Self::Error>> {
            Ok(Text::new(&**self))
        }

        #[inline]
        fn try_take(&mut self) -> Result<&'a Text, source::TryTakeError<&Text, Self::Error>> {
            Ok(Text::new(&**self))
        }
    }

    impl<'a> SourceRef<'a, Text> for Cow<'a, str> {
        type Error = source::Impossible;

        #[inline]
        fn take(&mut self) -> Result<&Text, source::TakeError<Self::Error>> {
            Ok(Text::new(&*self))
        }

        #[inline]
        fn take_owned(&mut self) -> Result<String, source::TakeError<Self::Error>> {
            Ok(mem::take(self).into_owned())
        }

        #[inline]
        fn try_take(&mut self) -> Result<&'a Text, source::TryTakeError<&Text, Self::Error>> {
            Err(source::TryTakeError::Fallback(Text::new(&*self)))
        }

        #[inline]
        fn try_take_owned(
            &mut self,
        ) -> Result<&'a Text, source::TryTakeError<String, Self::Error>> {
            Err(source::TryTakeError::Fallback(mem::take(self).into_owned()))
        }
    }
}
