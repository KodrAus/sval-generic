mod computed;
mod number;
mod seq;
mod tag;
mod text;

use crate::{source, Receiver, Source, Value};

#[doc(inline)]
pub use self::{computed::*, tag::*, text::*};

impl Value for () {
    fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> crate::Result {
        receiver.unit()
    }
}

impl<'a> Source<'a> for () {
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
        receiver.null()
    }
}

impl<T: Value> Value for Option<T> {
    fn stream<'a, R: Receiver<'a>>(&'a self, receiver: R) -> crate::Result {
        self.as_ref().stream_to_end(receiver)
    }

    fn to_bool(&self) -> Option<bool> {
        self.as_ref().and_then(|value| value.to_bool())
    }

    fn to_f32(&self) -> Option<f32> {
        self.as_ref().and_then(|value| value.to_f32())
    }

    fn to_f64(&self) -> Option<f64> {
        self.as_ref().and_then(|value| value.to_f64())
    }

    fn to_i8(&self) -> Option<i8> {
        self.as_ref().and_then(|value| value.to_i8())
    }

    fn to_i16(&self) -> Option<i16> {
        self.as_ref().and_then(|value| value.to_i16())
    }

    fn to_i32(&self) -> Option<i32> {
        self.as_ref().and_then(|value| value.to_i32())
    }

    fn to_i64(&self) -> Option<i64> {
        self.as_ref().and_then(|value| value.to_i64())
    }

    fn to_i128(&self) -> Option<i128> {
        self.as_ref().and_then(|value| value.to_i128())
    }

    fn to_u8(&self) -> Option<u8> {
        self.as_ref().and_then(|value| value.to_u8())
    }

    fn to_u16(&self) -> Option<u16> {
        self.as_ref().and_then(|value| value.to_u16())
    }

    fn to_u32(&self) -> Option<u32> {
        self.as_ref().and_then(|value| value.to_u32())
    }

    fn to_u64(&self) -> Option<u64> {
        self.as_ref().and_then(|value| value.to_u64())
    }

    fn to_u128(&self) -> Option<u128> {
        self.as_ref().and_then(|value| value.to_u128())
    }

    fn to_char(&self) -> Option<char> {
        self.as_ref().and_then(|value| value.to_char())
    }

    fn to_str(&self) -> Option<&str> {
        self.as_ref().and_then(|value| value.to_str())
    }

    fn to_bytes(&self) -> Option<&[u8]> {
        self.as_ref().and_then(|value| value.to_bytes())
    }
}

impl<'a, T: Source<'a>> Source<'a> for Option<T> {
    fn stream_resume<'b, R: Receiver<'b>>(&mut self, receiver: R) -> crate::Result<source::Resume>
    where
        'a: 'b,
    {
        self.stream_to_end(receiver).map(|_| source::Resume::Done)
    }

    fn stream_to_end<'b, R: Receiver<'b>>(&mut self, receiver: R) -> crate::Result
    where
        'a: 'b,
    {
        struct Null;

        impl<'a> Source<'a> for Null {
            fn stream_resume<'b, R: Receiver<'b>>(
                &mut self,
                receiver: R,
            ) -> crate::Result<crate::Resume>
            where
                'a: 'b,
            {
                self.stream_to_end(receiver).map(|_| crate::Resume::Done)
            }

            fn stream_to_end<'b, R: Receiver<'b>>(&mut self, mut receiver: R) -> crate::Result
            where
                'a: 'b,
            {
                receiver.null()
            }
        }

        match self {
            None => tag()
                .for_nullable()
                .with_id(0)
                .with_label("None")
                .with_value(Null)
                .stream_to_end(receiver),
            Some(v) => tag()
                .for_nullable()
                .with_id(1)
                .with_label("Some")
                .with_value(v)
                .stream_to_end(receiver),
        }
    }
}

impl Value for bool {
    fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> crate::Result {
        receiver.bool(*self)
    }

    fn to_bool(&self) -> Option<bool> {
        Some(*self)
    }
}

impl<'a> Source<'a> for bool {
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
        receiver.bool(*self)
    }
}
