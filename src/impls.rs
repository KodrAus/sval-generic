use crate::{receiver, source, Receiver, Source, Value};

impl Value for () {
    fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> crate::Result {
        receiver.none()
    }
}

impl<'a> Source<'a> for () {
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
        receiver.none()
    }
}

impl<'a> source::ValueSource<'a, ()> for () {
    type Error = source::Impossible;

    #[inline]
    fn take(&mut self) -> Result<&(), source::TakeError<Self::Error>> {
        Ok(self)
    }
}

impl Value for bool {
    fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> crate::Result {
        receiver.bool(*self)
    }
}

impl<'a> Source<'a> for bool {
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
        receiver.bool(*self)
    }
}

impl<'a> source::ValueSource<'a, bool> for bool {
    type Error = source::Impossible;

    fn take(&mut self) -> Result<&bool, source::TakeError<Self::Error>> {
        Ok(self)
    }
}

impl Value for u8 {
    fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> crate::Result {
        receiver.u8(*self)
    }
}

impl<'a> Source<'a> for u8 {
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
        receiver.u8(*self)
    }
}

impl<'a> source::ValueSource<'a, u8> for u8 {
    type Error = source::Impossible;

    #[inline]
    fn take(&mut self) -> Result<&u8, source::TakeError<Self::Error>> {
        Ok(self)
    }
}

impl Value for i8 {
    fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> crate::Result {
        receiver.i8(*self)
    }
}

impl<'a> Source<'a> for i8 {
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
        receiver.i8(*self)
    }
}

impl<'a> source::ValueSource<'a, i8> for i8 {
    type Error = source::Impossible;

    #[inline]
    fn take(&mut self) -> Result<&i8, source::TakeError<Self::Error>> {
        Ok(self)
    }
}

impl Value for u16 {
    fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> crate::Result {
        receiver.u16(*self)
    }
}

impl<'a> Source<'a> for u16 {
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
        receiver.u16(*self)
    }
}

impl<'a> source::ValueSource<'a, u16> for u16 {
    type Error = source::Impossible;

    #[inline]
    fn take(&mut self) -> Result<&u16, source::TakeError<Self::Error>> {
        Ok(self)
    }
}

impl Value for i16 {
    fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> crate::Result {
        receiver.i16(*self)
    }
}

impl<'a> Source<'a> for i16 {
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
        receiver.i16(*self)
    }
}

impl<'a> source::ValueSource<'a, i16> for i16 {
    type Error = source::Impossible;

    #[inline]
    fn take(&mut self) -> Result<&i16, source::TakeError<Self::Error>> {
        Ok(self)
    }
}

impl Value for u32 {
    fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> crate::Result {
        receiver.u32(*self)
    }
}

impl<'a> Source<'a> for u32 {
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
        receiver.u32(*self)
    }
}

impl<'a> source::ValueSource<'a, u32> for u32 {
    type Error = source::Impossible;

    #[inline]
    fn take(&mut self) -> Result<&u32, source::TakeError<Self::Error>> {
        Ok(self)
    }
}

impl Value for i32 {
    fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> crate::Result {
        receiver.i32(*self)
    }
}

impl<'a> Source<'a> for i32 {
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
        receiver.i32(*self)
    }
}

impl<'a> source::ValueSource<'a, i32> for i32 {
    type Error = source::Impossible;

    #[inline]
    fn take(&mut self) -> Result<&i32, source::TakeError<Self::Error>> {
        Ok(self)
    }
}

impl Value for u64 {
    fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> crate::Result {
        receiver.u64(*self)
    }
}

impl<'a> Source<'a> for u64 {
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
        receiver.u64(*self)
    }
}

impl<'a> source::ValueSource<'a, u64> for u64 {
    type Error = source::Impossible;

    #[inline]
    fn take(&mut self) -> Result<&u64, source::TakeError<Self::Error>> {
        Ok(self)
    }
}

impl Value for i64 {
    fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> crate::Result {
        receiver.i64(*self)
    }
}

impl<'a> Source<'a> for i64 {
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
        receiver.i64(*self)
    }
}

impl<'a> source::ValueSource<'a, i64> for i64 {
    type Error = source::Impossible;

    #[inline]
    fn take(&mut self) -> Result<&i64, source::TakeError<Self::Error>> {
        Ok(self)
    }
}

impl Value for u128 {
    fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> crate::Result {
        receiver.u128(*self)
    }
}

impl<'a> Source<'a> for u128 {
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
        receiver.u128(*self)
    }
}

impl<'a> source::ValueSource<'a, u128> for u128 {
    type Error = source::Impossible;

    #[inline]
    fn take(&mut self) -> Result<&u128, source::TakeError<Self::Error>> {
        Ok(self)
    }
}

impl Value for i128 {
    fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> crate::Result {
        receiver.i128(*self)
    }
}

impl<'a> Source<'a> for i128 {
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
        receiver.i128(*self)
    }
}

impl<'a> source::ValueSource<'a, i128> for i128 {
    type Error = source::Impossible;

    #[inline]
    fn take(&mut self) -> Result<&i128, source::TakeError<Self::Error>> {
        Ok(self)
    }
}

impl Value for f32 {
    fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> crate::Result {
        receiver.f32(*self)
    }
}

impl<'a> Source<'a> for f32 {
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
        receiver.f32(*self)
    }
}

impl<'a> source::ValueSource<'a, f32> for f32 {
    type Error = source::Impossible;

    #[inline]
    fn take(&mut self) -> Result<&f32, source::TakeError<Self::Error>> {
        Ok(self)
    }
}

impl Value for f64 {
    fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> crate::Result {
        receiver.f64(*self)
    }
}

impl<'a> Source<'a> for f64 {
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
        receiver.f64(*self)
    }
}

impl<'a> source::ValueSource<'a, f64> for f64 {
    type Error = source::Impossible;

    #[inline]
    fn take(&mut self) -> Result<&f64, source::TakeError<Self::Error>> {
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

impl<'a> source::ValueSource<'a, str> for str {
    type Error = source::Impossible;

    #[inline]
    fn take(&mut self) -> Result<&str, source::TakeError<Self::Error>> {
        Ok(self)
    }
}

impl<T> Value for Option<T>
where
    T: Value,
{
    fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> crate::Result {
        match self {
            Some(v) => v.stream(receiver),
            None => receiver.none(),
        }
    }
}

impl<T> Value for [T]
where
    T: Value,
{
    fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> crate::Result {
        receiver.seq_begin(Some(self.len()))?;

        for elem in self {
            receiver.seq_elem(elem)?;
        }

        receiver.seq_end()
    }
}

impl<T, const N: usize> Value for [T; N]
where
    T: Value,
{
    fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> crate::Result {
        receiver.seq_begin(Some(self.len()))?;

        for elem in self {
            receiver.seq_elem(elem)?;
        }

        receiver.seq_end()
    }
}

impl<T, U> Value for (T, U)
where
    T: Value,
    U: Value,
{
    fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> crate::Result {
        receiver.seq_begin(Some(2))?;
        receiver.seq_elem(&self.0)?;
        receiver.seq_elem(&self.1)?;
        receiver.seq_end()
    }
}

impl Value for dyn receiver::Error + 'static {
    fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> crate::Result {
        receiver.error(self)
    }
}

#[cfg(feature = "alloc")]
mod alloc_support {
    use super::*;

    use crate::{
        for_all,
        std::{borrow::Cow, boxed::Box, string::String, vec::Vec},
    };

    impl<T> Value for Vec<T>
    where
        T: Value,
    {
        fn stream<'a, S: Receiver<'a>>(&'a self, stream: S) -> crate::Result {
            (&**self).stream(stream)
        }
    }

    impl<T: ?Sized> Value for Box<T>
    where
        T: Value,
    {
        fn stream<'a, S: Receiver<'a>>(&'a self, stream: S) -> crate::Result {
            (**self).stream(stream)
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

    impl<'a> source::ValueSource<'a, str> for String {
        type Error = source::Impossible;

        #[inline]
        fn take(&mut self) -> Result<&str, source::TakeError<Self::Error>> {
            Ok(&**self)
        }

        #[inline]
        fn take_owned(&mut self) -> Result<String, source::TakeError<Self::Error>> {
            Ok(std::mem::take(self))
        }
    }

    impl<'a> source::ValueSource<'a, str> for &'a String {
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

    impl<'a> source::ValueSource<'a, str> for Cow<'a, str> {
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
            Ok(std::mem::take(self).into_owned())
        }
    }
}
