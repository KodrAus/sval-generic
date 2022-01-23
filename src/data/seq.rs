use crate::{
    data, receiver,
    source::{self, ValueSource},
    Receiver, Result, Value,
};

impl<T: Value> Value for [T] {
    fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> crate::Result {
        receiver.seq_begin(Some(self.len()))?;

        for elem in self {
            receiver.seq_elem(elem)?;
        }

        receiver.seq_end()
    }
}

impl<T: Value, const N: usize> Value for [T; N] {
    fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> crate::Result {
        receiver.tagged_seq_begin(
            data::tag("").with_content_hint(data::tag::ContentHint::Array),
            Some(self.len()),
        )?;

        for elem in self {
            receiver.seq_elem(elem)?;
        }

        receiver.tagged_seq_end()
    }
}

impl<'a, T: Value, const N: usize> ValueSource<'a, [T]> for &'a [T; N] {
    type Error = source::Impossible;

    #[inline]
    fn take(&mut self) -> Result<&[T], source::TakeError<Self::Error>> {
        Ok(*self)
    }

    #[inline]
    fn try_take_ref(&mut self) -> Result<&'a [T], source::TryTakeError<&[T], Self::Error>> {
        Ok(*self)
    }
}

macro_rules! tuple {
    ($(
        $len:expr => ( $(self.$i:tt: $ty:ident,)+ ),
    )+) => {
        $(
            impl<$($ty: Value),+> Value for ($($ty,)+) {
                fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> crate::Result {
                    receiver.seq_begin(receiver::Size::Fixed($len))?;
                    $(
                        receiver.seq_elem(&self.$i)?;
                    )+
                    receiver.seq_end()
                }
            }
        )+
    }
}

tuple! {
    1 => (
        self.0: T0,
    ),
    2 => (
        self.0: T0,
        self.1: T1,
    ),
    3 => (
        self.0: T0,
        self.1: T1,
        self.2: T2,
    ),
    4 => (
        self.0: T0,
        self.1: T1,
        self.2: T2,
        self.3: T3,
    ),
    5 => (
        self.0: T0,
        self.1: T1,
        self.2: T2,
        self.3: T3,
        self.4: T4,
    ),
    6 => (
        self.0: T0,
        self.1: T1,
        self.2: T2,
        self.3: T3,
        self.4: T4,
        self.5: T5,
    ),
    7 => (
        self.0: T0,
        self.1: T1,
        self.2: T2,
        self.3: T3,
        self.4: T4,
        self.5: T5,
        self.6: T6,
    ),
    8 => (
        self.0: T0,
        self.1: T1,
        self.2: T2,
        self.3: T3,
        self.4: T4,
        self.5: T5,
        self.6: T6,
        self.7: T7,
    ),
    9 => (
        self.0: T0,
        self.1: T1,
        self.2: T2,
        self.3: T3,
        self.4: T4,
        self.5: T5,
        self.6: T6,
        self.7: T7,
        self.8: T8,
    ),
    10 => (
        self.0: T0,
        self.1: T1,
        self.2: T2,
        self.3: T3,
        self.4: T4,
        self.5: T5,
        self.6: T6,
        self.7: T7,
        self.8: T8,
        self.9: T9,
    ),
    11 => (
        self.0: T0,
        self.1: T1,
        self.2: T2,
        self.3: T3,
        self.4: T4,
        self.5: T5,
        self.6: T6,
        self.7: T7,
        self.8: T8,
        self.9: T9,
        self.10: T10,
    ),
    12 => (
        self.0: T0,
        self.1: T1,
        self.2: T2,
        self.3: T3,
        self.4: T4,
        self.5: T5,
        self.6: T6,
        self.7: T7,
        self.8: T8,
        self.9: T9,
        self.10: T10,
        self.11: T11,
    ),
    13 => (
        self.0: T0,
        self.1: T1,
        self.2: T2,
        self.3: T3,
        self.4: T4,
        self.5: T5,
        self.6: T6,
        self.7: T7,
        self.8: T8,
        self.9: T9,
        self.10: T10,
        self.11: T11,
        self.12: T12,
    ),
    14 => (
        self.0: T0,
        self.1: T1,
        self.2: T2,
        self.3: T3,
        self.4: T4,
        self.5: T5,
        self.6: T6,
        self.7: T7,
        self.8: T8,
        self.9: T9,
        self.10: T10,
        self.11: T11,
        self.12: T12,
        self.13: T13,
    ),
    15 => (
        self.0: T0,
        self.1: T1,
        self.2: T2,
        self.3: T3,
        self.4: T4,
        self.5: T5,
        self.6: T6,
        self.7: T7,
        self.8: T8,
        self.9: T9,
        self.10: T10,
        self.11: T11,
        self.12: T12,
        self.13: T13,
        self.14: T14,
    ),
    16 => (
        self.0: T0,
        self.1: T1,
        self.2: T2,
        self.3: T3,
        self.4: T4,
        self.5: T5,
        self.6: T6,
        self.7: T7,
        self.8: T8,
        self.9: T9,
        self.10: T10,
        self.11: T11,
        self.12: T12,
        self.13: T13,
        self.14: T14,
        self.15: T15,
    ),
}

#[cfg(feature = "alloc")]
mod alloc_support {
    use super::*;

    use crate::{
        for_all,
        std::{
            borrow::{Cow, ToOwned},
            mem,
            vec::Vec,
        },
        Source,
    };

    impl<T: Value> Value for Vec<T> {
        fn stream<'a, S: Receiver<'a>>(&'a self, stream: S) -> crate::Result {
            (&**self).stream(stream)
        }
    }

    impl<'a, T: Value + Clone> Value for Cow<'a, [T]> {
        fn stream<'b, R: Receiver<'b>>(&'b self, receiver: R) -> crate::Result {
            (&**self).stream(receiver)
        }
    }

    impl<'a, T: Value + Clone> Source<'a> for Cow<'a, [T]> {
        fn stream_resume<'b, R: Receiver<'b>>(
            &mut self,
            receiver: R,
        ) -> crate::Result<source::Stream>
        where
            'a: 'b,
        {
            self.stream_to_end(receiver).map(|_| source::Stream::Done)
        }

        fn stream_to_end<'b, R: Receiver<'b>>(&mut self, receiver: R) -> crate::Result
        where
            'a: 'b,
        {
            match self {
                Cow::Borrowed(v) => (&**v).stream(receiver),
                Cow::Owned(v) => (&**v).stream(for_all(receiver)),
            }
        }
    }

    impl<'a, T: Value + Clone> ValueSource<'a, [T]> for Cow<'a, [T]> {
        type Error = source::Impossible;

        #[inline]
        fn take(&mut self) -> Result<&[T], source::TakeError<Self::Error>> {
            Ok(&**self)
        }

        #[inline]
        fn try_take_ref(&mut self) -> Result<&'a [T], source::TryTakeError<&[T], Self::Error>> {
            match self {
                Cow::Borrowed(v) => Ok(v),
                Cow::Owned(v) => Err(source::TryTakeError::from_value(v)),
            }
        }

        #[inline]
        fn take_owned(
            &mut self,
        ) -> Result<<[T] as ToOwned>::Owned, source::TakeError<Self::Error>> {
            Ok(mem::take(self).into_owned())
        }
    }
}
