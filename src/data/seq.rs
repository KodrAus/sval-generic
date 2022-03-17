use crate::{data, source, std::ops, Receiver, Result, Source, Value};

pub fn bytes<'a>(bytes: &'a impl AsRef<[u8]>) -> Bytes<'a> {
    Bytes::new(bytes)
}

pub struct Bytes<'a>(&'a [u8]);

impl<'a> Bytes<'a> {
    pub fn new(bytes: &'a impl AsRef<[u8]>) -> Self {
        Bytes(bytes.as_ref())
    }
}

impl<'a> Value for Bytes<'a> {
    fn stream<'b, R: Receiver<'b>>(&'b self, mut receiver: R) -> Result {
        receiver.bytes(self.0)
    }
}

impl<'a> Source<'a> for Bytes<'a> {
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
        receiver.bytes(self.0)
    }
}

impl<'a> AsRef<[u8]> for Bytes<'a> {
    fn as_ref(&self) -> &[u8] {
        self.0
    }
}

impl<'a> ops::Deref for Bytes<'a> {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        self.0
    }
}

impl<T: Value> Value for [T] {
    fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> Result {
        receiver.tagged_begin(data::tag().for_slice())?;
        receiver.seq_begin(Some(self.len()))?;

        for elem in self {
            receiver.seq_value(elem)?;
        }

        receiver.seq_end()?;
        receiver.tagged_end(data::tag().for_slice())
    }
}

impl<T: Value, const N: usize> Value for [T; N] {
    fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> Result {
        receiver.tagged_begin(data::tag().for_array())?;
        receiver.seq_begin(Some(self.len()))?;

        for elem in self {
            receiver.seq_value(elem)?;
        }

        receiver.seq_end()?;
        receiver.tagged_end(data::tag().for_array())
    }
}

macro_rules! tuple {
    ($(
        $len:expr => ( $(self.$i:tt: $ty:ident,)+ ),
    )+) => {
        $(
            impl<$($ty: Value),+> Value for ($($ty,)+) {
                fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> Result {
                    receiver.tagged_begin(data::tag().for_struct())?;
                    receiver.seq_begin(Some($len))?;

                    $(
                        receiver.seq_value(data::tag().for_struct_value().with_id($i).with_value(&self.$i))?;
                    )+

                    receiver.seq_end()?;
                    receiver.tagged_end(data::tag().for_struct())
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

    use crate::std::vec::Vec;

    impl<T: Value> Value for Vec<T> {
        fn stream<'a, S: Receiver<'a>>(&'a self, receiver: S) -> Result {
            (&**self).stream(receiver)
        }
    }

    impl<'a, T: Source<'a>> Source<'a> for Vec<T> {
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
            receiver.tagged_begin(data::tag().for_slice())?;
            receiver.seq_begin(Some(self.len()))?;

            for elem in self.drain(..) {
                receiver.seq_value(elem)?;
            }

            receiver.seq_end()?;
            receiver.tagged_end(data::tag().for_slice())
        }
    }
}
