use crate::{
    data::{self, Position},
    Receiver, Result, Resume, Source, Value,
};

impl<T: Value> Value for [T] {
    fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> Result {
        receiver.seq_begin(Some(self.len()))?;

        for elem in self {
            receiver.seq_value(elem)?;
        }

        receiver.seq_end()
    }
}

impl<T: Value, const N: usize> Value for [T; N] {
    fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> Result {
        receiver.fixed_size_begin()?;
        receiver.seq_begin(Some(self.len()))?;

        for elem in self {
            receiver.seq_value(elem)?;
        }

        receiver.seq_end()?;
        receiver.fixed_size_end()
    }
}

macro_rules! tuple {
    ($(
        $len:expr => ( $(self.$i:tt: $ty:ident,)+ ),
    )+) => {
        $(
            impl<$($ty: Value),+> Value for ($($ty,)+) {
                fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> Result {
                    receiver.struct_seq_begin(data::tag(), Some($len))?;

                    $(
                        receiver.struct_seq_value(data::tag().with_id($i), &self.$i)?;
                    )+

                    receiver.struct_seq_end()
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

pub fn seq<S: Iterator>(seq: S) -> Seq<S> {
    Seq::new(seq)
}

pub struct Seq<S: Iterator> {
    seq: S,
    current: Option<S::Item>,
    position: Position,
}

impl<S: Iterator> Seq<S> {
    pub fn new(seq: S) -> Self {
        Seq {
            seq,
            position: Position::Begin,
            current: None,
        }
    }
}

impl<'src, S: Iterator> Source<'src> for Seq<S>
where
    S::Item: Source<'src>,
{
    fn stream_resume<'data, R: Receiver<'data>>(&mut self, mut receiver: R) -> Result<Resume>
    where
        'src: 'data,
    {
        loop {
            if let Some(current) = self.current.as_mut() {
                match current.stream_resume(&mut receiver)? {
                    Resume::Continue => return Ok(Resume::Continue),
                    Resume::Done => self.current = None,
                }
            }

            debug_assert!(self.current.is_none());

            match self.position {
                Position::Begin => {
                    receiver.seq_begin(None)?;
                    self.position = Position::Value;
                }
                Position::Value => match self.seq.next() {
                    Some(next) => self.current = Some(next),
                    None => self.position = Position::End,
                },
                Position::End => {
                    receiver.seq_end()?;
                    self.position = Position::Done;
                }
                Position::Done => return Ok(Resume::Done),
            }
        }
    }

    fn maybe_dynamic(&self) -> Option<bool> {
        Some(false)
    }
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
}
