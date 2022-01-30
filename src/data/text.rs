use crate::source::{Stream, TakeError, TryTakeError};
use crate::{
    source::{self, ValueSource},
    std::{fmt, marker::PhantomData},
    Receiver, Result, Source, Value,
};

pub fn text(text: &impl fmt::Display) -> &Text {
    Text::new(text)
}

// TODO: If this works, try generalize for any `T` and `U` (not sure about owned)
pub(crate) fn text_value_source<
    'a,
    T: AsRef<Text> + Value + ?Sized,
    U: AsRef<Text> + Value + ?Sized + 'a,
>(
    v: impl ValueSource<'a, T, U>,
) -> impl ValueSource<'a, Text> {
    struct AsText<'a, T: Value + ?Sized, U: Value + ?Sized, V: ValueSource<'a, T, U>>(
        V,
        PhantomData<fn(&V) -> &T>,
        PhantomData<fn(&V) -> &'a U>,
    );

    impl<
            'a,
            T: AsRef<Text> + Value + ?Sized,
            U: AsRef<Text> + Value + ?Sized,
            V: ValueSource<'a, T, U>,
        > ValueSource<'a, Text> for AsText<'a, T, U, V>
    {
        type Error = V::Error;

        fn take(&mut self) -> Result<&Text, TakeError<Self::Error>> {
            self.0.take().map(|v| v.as_ref())
        }

        fn try_take_ref(&mut self) -> Result<&'a Text, TryTakeError<&Text, Self::Error>> {
            match self.0.try_take_ref() {
                Ok(v) => Ok(v.as_ref()),
                Err(TryTakeError::Fallback(v)) => Err(TryTakeError::Fallback(v.as_ref())),
                Err(TryTakeError::Err(e)) => Err(TryTakeError::Err(e)),
            }
        }
    }

    impl<'a, T: Value + ?Sized, U: Value + ?Sized, V: ValueSource<'a, T, U>> Source<'a>
        for AsText<'a, T, U, V>
    {
        fn stream_resume<'b, R: Receiver<'b>>(&mut self, receiver: R) -> Result<Stream>
        where
            'a: 'b,
        {
            self.0.stream_resume(receiver)
        }

        fn stream_to_end<'b, R: Receiver<'b>>(&mut self, receiver: R) -> Result
        where
            'a: 'b,
        {
            self.0.stream_to_end(receiver)
        }
    }

    AsText(v, PhantomData, PhantomData)
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

impl AsRef<Text> for str {
    fn as_ref(&self) -> &Text {
        Text::new(self)
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
        fn take_owned(&mut self) -> Result<String, source::TakeError<Self::Error>> {
            Ok(mem::take(self))
        }

        #[inline]
        fn try_take_owned(&mut self) -> Result<String, TryTakeError<&str, Self::Error>> {
            Ok(mem::take(self))
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
        fn try_take_ref(&mut self) -> Result<&'a str, source::TryTakeError<&str, Self::Error>> {
            match self {
                Cow::Borrowed(v) => Ok(v),
                Cow::Owned(v) => Err(source::TryTakeError::Fallback(v)),
            }
        }

        #[inline]
        fn take_owned(&mut self) -> Result<String, source::TakeError<Self::Error>> {
            Ok(mem::take(self).into_owned())
        }

        #[inline]
        fn try_take_owned(&mut self) -> Result<String, TryTakeError<&str, Self::Error>> {
            match self {
                Cow::Borrowed(v) => Err(source::TryTakeError::Fallback(v)),
                Cow::Owned(v) => Ok(mem::take(v)),
            }
        }
    }

    impl<'a> ValueSource<'a, Text> for String {
        type Error = source::Impossible;

        #[inline]
        fn take(&mut self) -> Result<&Text, source::TakeError<Self::Error>> {
            Ok(Text::new(&**self))
        }

        #[inline]
        fn take_owned(&mut self) -> Result<String, source::TakeError<Self::Error>> {
            Ok(mem::take(self))
        }

        #[inline]
        fn try_take_owned(&mut self) -> Result<String, TryTakeError<&Text, Self::Error>> {
            Ok(mem::take(self))
        }
    }

    impl<'a> ValueSource<'a, Text> for &'a String {
        type Error = source::Impossible;

        #[inline]
        fn take(&mut self) -> Result<&Text, source::TakeError<Self::Error>> {
            Ok(Text::new(&**self))
        }

        #[inline]
        fn try_take_ref(&mut self) -> Result<&'a Text, TryTakeError<&str, Self::Error>> {
            Ok(Text::new(&**self))
        }
    }
}
