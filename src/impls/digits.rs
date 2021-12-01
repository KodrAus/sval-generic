use crate::{
    data,
    source::{self, ValueSource},
    Receiver, Result, Source, Value,
};

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

impl<'a> ValueSource<'a, u8> for u8 {
    type Error = source::Impossible;

    #[inline]
    fn take(&mut self) -> Result<&u8, source::TakeError<Self::Error>> {
        Ok(self)
    }
}

impl<'a> ValueSource<'a, data::Digits> for &'a u8 {
    type Error = source::Impossible;

    #[inline]
    fn take(&mut self) -> Result<&data::Digits, source::TakeError<Self::Error>> {
        Ok(data::digits_unchecked(self))
    }

    #[inline]
    fn take_ref(
        &mut self,
    ) -> Result<&'a data::Digits, source::TakeRefError<&data::Digits, Self::Error>> {
        Ok(data::digits_unchecked(*self))
    }
}

impl<'a> ValueSource<'a, data::Digits> for u8 {
    type Error = source::Impossible;

    #[inline]
    fn take(&mut self) -> Result<&data::Digits, source::TakeError<Self::Error>> {
        Ok(data::digits_unchecked(self))
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

impl<'a> ValueSource<'a, i8> for i8 {
    type Error = source::Impossible;

    #[inline]
    fn take(&mut self) -> Result<&i8, source::TakeError<Self::Error>> {
        Ok(self)
    }
}

impl<'a> ValueSource<'a, data::Digits> for &'a i8 {
    type Error = source::Impossible;

    #[inline]
    fn take(&mut self) -> Result<&data::Digits, source::TakeError<Self::Error>> {
        Ok(data::digits_unchecked(self))
    }

    #[inline]
    fn take_ref(
        &mut self,
    ) -> Result<&'a data::Digits, source::TakeRefError<&data::Digits, Self::Error>> {
        Ok(data::digits_unchecked(*self))
    }
}

impl<'a> ValueSource<'a, data::Digits> for i8 {
    type Error = source::Impossible;

    #[inline]
    fn take(&mut self) -> Result<&data::Digits, source::TakeError<Self::Error>> {
        Ok(data::digits_unchecked(self))
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

impl<'a> ValueSource<'a, u16> for u16 {
    type Error = source::Impossible;

    #[inline]
    fn take(&mut self) -> Result<&u16, source::TakeError<Self::Error>> {
        Ok(self)
    }
}

impl<'a> ValueSource<'a, data::Digits> for &'a u16 {
    type Error = source::Impossible;

    #[inline]
    fn take(&mut self) -> Result<&data::Digits, source::TakeError<Self::Error>> {
        Ok(data::digits_unchecked(self))
    }

    #[inline]
    fn take_ref(
        &mut self,
    ) -> Result<&'a data::Digits, source::TakeRefError<&data::Digits, Self::Error>> {
        Ok(data::digits_unchecked(*self))
    }
}

impl<'a> ValueSource<'a, data::Digits> for u16 {
    type Error = source::Impossible;

    #[inline]
    fn take(&mut self) -> Result<&data::Digits, source::TakeError<Self::Error>> {
        Ok(data::digits_unchecked(self))
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

impl<'a> ValueSource<'a, i16> for i16 {
    type Error = source::Impossible;

    #[inline]
    fn take(&mut self) -> Result<&i16, source::TakeError<Self::Error>> {
        Ok(self)
    }
}

impl<'a> ValueSource<'a, data::Digits> for &'a i16 {
    type Error = source::Impossible;

    #[inline]
    fn take(&mut self) -> Result<&data::Digits, source::TakeError<Self::Error>> {
        Ok(data::digits_unchecked(self))
    }

    #[inline]
    fn take_ref(
        &mut self,
    ) -> Result<&'a data::Digits, source::TakeRefError<&data::Digits, Self::Error>> {
        Ok(data::digits_unchecked(*self))
    }
}

impl<'a> ValueSource<'a, data::Digits> for i16 {
    type Error = source::Impossible;

    #[inline]
    fn take(&mut self) -> Result<&data::Digits, source::TakeError<Self::Error>> {
        Ok(data::digits_unchecked(self))
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

impl<'a> ValueSource<'a, u32> for u32 {
    type Error = source::Impossible;

    #[inline]
    fn take(&mut self) -> Result<&u32, source::TakeError<Self::Error>> {
        Ok(self)
    }
}

impl<'a> ValueSource<'a, data::Digits> for &'a u32 {
    type Error = source::Impossible;

    #[inline]
    fn take(&mut self) -> Result<&data::Digits, source::TakeError<Self::Error>> {
        Ok(data::digits_unchecked(self))
    }

    #[inline]
    fn take_ref(
        &mut self,
    ) -> Result<&'a data::Digits, source::TakeRefError<&data::Digits, Self::Error>> {
        Ok(data::digits_unchecked(*self))
    }
}

impl<'a> ValueSource<'a, data::Digits> for u32 {
    type Error = source::Impossible;

    #[inline]
    fn take(&mut self) -> Result<&data::Digits, source::TakeError<Self::Error>> {
        Ok(data::digits_unchecked(self))
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

impl<'a> ValueSource<'a, i32> for i32 {
    type Error = source::Impossible;

    #[inline]
    fn take(&mut self) -> Result<&i32, source::TakeError<Self::Error>> {
        Ok(self)
    }
}

impl<'a> ValueSource<'a, data::Digits> for &'a i32 {
    type Error = source::Impossible;

    #[inline]
    fn take(&mut self) -> Result<&data::Digits, source::TakeError<Self::Error>> {
        Ok(data::digits_unchecked(self))
    }

    #[inline]
    fn take_ref(
        &mut self,
    ) -> Result<&'a data::Digits, source::TakeRefError<&data::Digits, Self::Error>> {
        Ok(data::digits_unchecked(*self))
    }
}

impl<'a> ValueSource<'a, data::Digits> for i32 {
    type Error = source::Impossible;

    #[inline]
    fn take(&mut self) -> Result<&data::Digits, source::TakeError<Self::Error>> {
        Ok(data::digits_unchecked(self))
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

impl<'a> ValueSource<'a, u64> for u64 {
    type Error = source::Impossible;

    #[inline]
    fn take(&mut self) -> Result<&u64, source::TakeError<Self::Error>> {
        Ok(self)
    }
}

impl<'a> ValueSource<'a, data::Digits> for &'a u64 {
    type Error = source::Impossible;

    #[inline]
    fn take(&mut self) -> Result<&data::Digits, source::TakeError<Self::Error>> {
        Ok(data::digits_unchecked(self))
    }

    #[inline]
    fn take_ref(
        &mut self,
    ) -> Result<&'a data::Digits, source::TakeRefError<&data::Digits, Self::Error>> {
        Ok(data::digits_unchecked(*self))
    }
}

impl<'a> ValueSource<'a, data::Digits> for u64 {
    type Error = source::Impossible;

    #[inline]
    fn take(&mut self) -> Result<&data::Digits, source::TakeError<Self::Error>> {
        Ok(data::digits_unchecked(self))
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

impl<'a> ValueSource<'a, i64> for i64 {
    type Error = source::Impossible;

    #[inline]
    fn take(&mut self) -> Result<&i64, source::TakeError<Self::Error>> {
        Ok(self)
    }
}

impl<'a> ValueSource<'a, data::Digits> for &'a i64 {
    type Error = source::Impossible;

    #[inline]
    fn take(&mut self) -> Result<&data::Digits, source::TakeError<Self::Error>> {
        Ok(data::digits_unchecked(self))
    }

    #[inline]
    fn take_ref(
        &mut self,
    ) -> Result<&'a data::Digits, source::TakeRefError<&data::Digits, Self::Error>> {
        Ok(data::digits_unchecked(*self))
    }
}

impl<'a> ValueSource<'a, data::Digits> for i64 {
    type Error = source::Impossible;

    #[inline]
    fn take(&mut self) -> Result<&data::Digits, source::TakeError<Self::Error>> {
        Ok(data::digits_unchecked(self))
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

impl<'a> ValueSource<'a, u128> for u128 {
    type Error = source::Impossible;

    #[inline]
    fn take(&mut self) -> Result<&u128, source::TakeError<Self::Error>> {
        Ok(self)
    }
}

impl<'a> ValueSource<'a, data::Digits> for &'a u128 {
    type Error = source::Impossible;

    #[inline]
    fn take(&mut self) -> Result<&data::Digits, source::TakeError<Self::Error>> {
        Ok(data::digits_unchecked(self))
    }

    #[inline]
    fn take_ref(
        &mut self,
    ) -> Result<&'a data::Digits, source::TakeRefError<&data::Digits, Self::Error>> {
        Ok(data::digits_unchecked(*self))
    }
}

impl<'a> ValueSource<'a, data::Digits> for u128 {
    type Error = source::Impossible;

    #[inline]
    fn take(&mut self) -> Result<&data::Digits, source::TakeError<Self::Error>> {
        Ok(data::digits_unchecked(self))
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

impl<'a> ValueSource<'a, i128> for i128 {
    type Error = source::Impossible;

    #[inline]
    fn take(&mut self) -> Result<&i128, source::TakeError<Self::Error>> {
        Ok(self)
    }
}

impl<'a> ValueSource<'a, data::Digits> for &'a i128 {
    type Error = source::Impossible;

    #[inline]
    fn take(&mut self) -> Result<&data::Digits, source::TakeError<Self::Error>> {
        Ok(data::digits_unchecked(self))
    }

    #[inline]
    fn take_ref(
        &mut self,
    ) -> Result<&'a data::Digits, source::TakeRefError<&data::Digits, Self::Error>> {
        Ok(data::digits_unchecked(*self))
    }
}

impl<'a> ValueSource<'a, data::Digits> for i128 {
    type Error = source::Impossible;

    #[inline]
    fn take(&mut self) -> Result<&data::Digits, source::TakeError<Self::Error>> {
        Ok(data::digits_unchecked(self))
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

impl<'a> ValueSource<'a, f32> for f32 {
    type Error = source::Impossible;

    #[inline]
    fn take(&mut self) -> Result<&f32, source::TakeError<Self::Error>> {
        Ok(self)
    }
}

impl<'a> ValueSource<'a, data::Digits> for &'a f32 {
    type Error = source::Impossible;

    #[inline]
    fn take(&mut self) -> Result<&data::Digits, source::TakeError<Self::Error>> {
        Ok(data::digits_unchecked(self))
    }

    #[inline]
    fn take_ref(
        &mut self,
    ) -> Result<&'a data::Digits, source::TakeRefError<&data::Digits, Self::Error>> {
        Ok(data::digits_unchecked(*self))
    }
}

impl<'a> ValueSource<'a, data::Digits> for f32 {
    type Error = source::Impossible;

    #[inline]
    fn take(&mut self) -> Result<&data::Digits, source::TakeError<Self::Error>> {
        Ok(data::digits_unchecked(self))
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

impl<'a> ValueSource<'a, f64> for f64 {
    type Error = source::Impossible;

    #[inline]
    fn take(&mut self) -> Result<&f64, source::TakeError<Self::Error>> {
        Ok(self)
    }
}

impl<'a> ValueSource<'a, data::Digits> for &'a f64 {
    type Error = source::Impossible;

    #[inline]
    fn take(&mut self) -> Result<&data::Digits, source::TakeError<Self::Error>> {
        Ok(data::digits_unchecked(self))
    }

    #[inline]
    fn take_ref(
        &mut self,
    ) -> Result<&'a data::Digits, source::TakeRefError<&data::Digits, Self::Error>> {
        Ok(data::digits_unchecked(*self))
    }
}

impl<'a> ValueSource<'a, data::Digits> for f64 {
    type Error = source::Impossible;

    #[inline]
    fn take(&mut self) -> Result<&data::Digits, source::TakeError<Self::Error>> {
        Ok(data::digits_unchecked(self))
    }
}

impl<'a> ValueSource<'a, data::Digits> for &'a str {
    type Error = crate::Error;

    #[inline]
    fn take(&mut self) -> Result<&data::Digits, source::TakeError<Self::Error>> {
        data::digits(self).map_err(source::TakeError::from_error)
    }
}
