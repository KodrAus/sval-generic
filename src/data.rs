mod binary;
mod computed;
mod dynamic;
mod map;
mod nullable;
mod number;
mod seq;
mod tag;
mod text;

use crate::{source, Receiver, Source, Value};

#[doc(inline)]
pub use self::{binary::*, computed::*, dynamic::*, map::*, nullable::*, seq::*, tag::*, text::*};

#[cfg(not(test))]
pub(crate) use self::number::*;

impl Value for () {
    fn stream<'a, R: Receiver<'a>>(&'a self, receiver: R) -> crate::Result {
        { *self }.stream_to_end(receiver)
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
        receiver.unit()
    }

    fn maybe_dynamic(&self) -> Option<bool> {
        Some(false)
    }
}

impl Value for bool {
    fn stream<'a, R: Receiver<'a>>(&'a self, receiver: R) -> crate::Result {
        { *self }.stream_to_end(receiver)
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

    fn maybe_dynamic(&self) -> Option<bool> {
        Some(false)
    }
}

#[derive(Debug, Clone, Copy)]
enum Position {
    Begin,
    Value,
    End,
    Done,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn option_cast() {
        assert_eq!(Some(1u8), Some(1u8).to_u8());
        assert_eq!(Some(2u16), Some(2u16).to_u16());
        assert_eq!(Some(3u32), Some(3u32).to_u32());
        assert_eq!(Some(4u64), Some(4u64).to_u64());
        assert_eq!(Some(42u128), Some(42u128).to_u128());

        assert_eq!(Some(1i8), Some(1i8).to_i8());
        assert_eq!(Some(2i16), Some(2i16).to_i16());
        assert_eq!(Some(3i32), Some(3i32).to_i32());
        assert_eq!(Some(4i64), Some(4i64).to_i64());
        assert_eq!(Some(42i128), Some(42i128).to_i128());

        assert_eq!(Some(3f32), Some(3f32).to_f32());
        assert_eq!(Some(4f64), Some(4f64).to_f64());

        assert_eq!(Some(true), Some(true).to_bool());

        assert_eq!(Some("a string"), Some("a string").to_text());
    }
}
