use crate::{Result, Stream, Value};

impl<T: Value> Value for [T] {
    fn stream<'a, S: Stream<'a>>(&'a self, mut stream: S) -> Result {
        stream.seq_begin(Some(self.len()))?;

        for elem in self {
            stream.seq_value_begin()?;
            stream.value(elem)?;
            stream.seq_value_end()?;
        }

        stream.seq_end()
    }

    fn is_dynamic(&self) -> bool {
        false
    }
}

impl<T: Value, const N: usize> Value for [T; N] {
    fn stream<'a, S: Stream<'a>>(&'a self, mut stream: S) -> Result {
        stream.fixed_size_begin()?;
        stream.seq_begin(Some(self.len()))?;

        for elem in self {
            stream.seq_value_begin()?;
            stream.value(elem)?;
            stream.seq_value_end()?;
        }

        stream.seq_end()?;
        stream.fixed_size_end()
    }

    fn is_dynamic(&self) -> bool {
        false
    }
}

macro_rules! tuple {
    ($(
        $len:expr => ( $(self.$i:tt: $ty:ident,)+ ),
    )+) => {
        $(
            impl<$($ty: Value),+> Value for ($($ty,)+) {
                fn stream<'sval, S: Stream<'sval>>(&'sval self, mut stream: S) -> Result {
                    stream.struct_seq_begin(None, Some($len))?;

                    $(
                        stream.struct_seq_value_begin(crate::Tag::Unlabeled { id: $i })?;
                        stream.value(&self.$i)?;
                        stream.struct_seq_value_end()?;
                    )+

                    stream.struct_seq_end()
                }

                fn is_dynamic(&self) -> bool {
                    false
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
        fn stream<'a, S: Stream<'a>>(&'a self, stream: S) -> Result {
            (&**self).stream(stream)
        }

        fn is_dynamic(&self) -> bool {
            false
        }
    }
}
