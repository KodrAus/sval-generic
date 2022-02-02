use crate::{data, source, Receiver, Result, SourceRef, SourceValue};

impl<T: SourceValue> SourceValue for [T] {
    fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> crate::Result {
        receiver.seq_begin(Some(self.len() as u64))?;

        for elem in self {
            receiver.seq_elem(elem)?;
        }

        receiver.seq_end()
    }
}

impl<T: SourceValue, const N: usize> SourceValue for [T; N] {
    fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> crate::Result {
        receiver.tagged_begin(data::tag().with_kind(data::tag::Kind::Array))?;
        receiver.seq_begin(Some(self.len() as u64))?;

        for elem in self {
            receiver.seq_elem(elem)?;
        }

        receiver.seq_end()?;
        receiver.tagged_end(data::tag().with_kind(data::tag::Kind::Array))
    }
}

impl<'a, T: SourceValue, const N: usize> SourceRef<'a, [T]> for &'a [T; N] {
    type Error = source::Impossible;

    #[inline]
    fn take(&mut self) -> Result<&[T], source::TakeError<Self::Error>> {
        Ok(*self)
    }

    #[inline]
    fn try_take(&mut self) -> Result<&'a [T], source::TryTakeError<&[T], Self::Error>> {
        Ok(*self)
    }
}

macro_rules! tuple {
    ($(
        $len:expr => ( $(self.$i:tt: $ty:ident,)+ ),
    )+) => {
        $(
            impl<$($ty: SourceValue),+> SourceValue for ($($ty,)+) {
                fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> crate::Result {
                    receiver.tagged_begin(data::tag().with_kind(data::tag::Kind::Tuple))?;
                    receiver.seq_begin(Some($len))?;
                    $(
                        receiver.seq_elem(&self.$i)?;
                    )+
                    receiver.seq_end()?;
                    receiver.tagged_end(data::tag().with_kind(data::tag::Kind::Tuple))
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
        source,
        std::{
            borrow::{Cow, ToOwned},
            mem,
            vec::Vec,
        },
        Source,
    };

    impl<T: SourceValue> SourceValue for Vec<T> {
        fn stream<'a, S: Receiver<'a>>(&'a self, stream: S) -> crate::Result {
            (&**self).stream(stream)
        }
    }

    impl<'a, T: SourceValue> SourceRef<'a, [T]> for Vec<T> {
        type Error = source::Impossible;

        #[inline]
        fn take(&mut self) -> Result<&[T], source::TakeError<Self::Error>> {
            Ok(&**self)
        }

        #[inline]
        fn take_owned(&mut self) -> Result<Cow<[T]>, source::TakeError<Self::Error>>
        where
            [T]: ToOwned,
        {
            Ok(Cow::Owned(mem::take(self)))
        }

        #[inline]
        fn take_ref(&mut self) -> Result<Cow<'a, [T]>, source::TakeError<Self::Error>>
        where
            [T]: ToOwned,
        {
            Ok(Cow::Owned(mem::take(self)))
        }
    }

    impl<'a, T: SourceValue> SourceRef<'a, [T]> for &'a Vec<T> {
        type Error = source::Impossible;

        #[inline]
        fn take(&mut self) -> Result<&[T], source::TakeError<Self::Error>> {
            Ok(&**self)
        }

        #[inline]
        fn try_take(&mut self) -> Result<&'a [T], source::TryTakeError<&[T], Self::Error>> {
            Ok(&**self)
        }
    }

    impl<'a, 'b, T: Source<'a>> Source<'a> for &'b mut [T] {
        fn stream_begin<'c, R: Receiver<'c>>(
            &mut self,
            mut receiver: R,
        ) -> crate::Result<source::Next>
        where
            'a: 'c,
        {
            receiver.seq_begin(Some(self.len() as u64))
        }

        fn stream_next<'c, R: Receiver<'c>>(&mut self, receiver: R) -> crate::Result<source::Next>
        where
            'a: 'c,
        {
            if let Some(next) = self.get(0) {
                let r = next.stream_all(receiver);
                *self = self[1..];
                r?;

                Ok(source::Next::Continue)
            } else {
                Ok(source::Next::Done)
            }
        }

        fn stream_end<'c, R: Receiver<'c>>(
            &mut self,
            mut receiver: R,
        ) -> crate::Result<source::Next>
        where
            'a: 'c,
        {
            receiver.seq_end()
        }

        fn stream_all<'c, R: Receiver<'c>>(&mut self, mut receiver: R) -> crate::Result
        where
            'a: 'c,
        {
            receiver.seq_begin(Some(self.len() as u64))?;

            for elem in self {
                receiver.seq_elem(elem)?;
            }

            receiver.seq_end()
        }
    }

    impl<'a, T: Source<'a>> Source<'a> for Vec<T> {
        fn stream_begin<'b, R: Receiver<'b>>(
            &mut self,
            mut receiver: R,
        ) -> crate::Result<source::Next>
        where
            'a: 'b,
        {
            receiver.seq_begin(Some(self.len() as u64))
        }

        fn stream_next<'b, R: Receiver<'b>>(&mut self, receiver: R) -> crate::Result<source::Next>
        where
            'a: 'b,
        {
            if let Some(mut next) = self.remove(0) {
                next.stream_all(receiver)?;

                Ok(source::Next::Continue)
            } else {
                Ok(source::Next::Done)
            }
        }

        fn stream_end<'b, R: Receiver<'b>>(
            &mut self,
            mut receiver: R,
        ) -> crate::Result<source::Next>
        where
            'a: 'b,
        {
            receiver.seq_end()
        }

        fn stream_all<'b, R: Receiver<'b>>(&mut self, mut receiver: R) -> crate::Result
        where
            'a: 'b,
        {
            receiver.seq_begin(Some(self.len() as u64))?;

            for elem in self.drain(..) {
                receiver.seq_elem(elem)?;
            }

            receiver.seq_end()
        }
    }
}
