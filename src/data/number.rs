use crate::{source, Receiver, Source, Value};

pub(crate) fn i128_bigint<'a>(v: i128, mut receiver: impl Receiver<'a>) -> crate::Result {
    receiver.tagged_begin(crate::data::tag().for_bigint())?;

    if receiver.is_human_readable() {
        crate::data::text(v, &mut receiver)?;
    } else {
        let bytes = v.to_le_bytes();

        receiver.binary_begin(Some(bytes.len()))?;
        receiver.binary_fragment_computed(&bytes)?;
        receiver.binary_end()?;
    }

    receiver.tagged_end(crate::data::tag().for_bigint())
}

pub(crate) fn u128_bigint<'a>(v: u128, mut receiver: impl Receiver<'a>) -> crate::Result {
    receiver.tagged_begin(crate::data::tag().for_bigint())?;

    if receiver.is_human_readable() {
        crate::data::text(v, &mut receiver)?;
    } else {
        // If the value fits in a signed 128bit number then write out its bytes
        if let Ok(v) = v.try_into::<i128>() {
            let bytes = v.to_le_bytes();

            receiver.binary_begin(Some(bytes.len()))?;
            receiver.binary_fragment_computed(&bytes)?;
            receiver.binary_end()?;
        }
        // If the value doesn't fit in a signed 128bit number then we need to
        // append an extra byte to make it signed. This byte will always be empty
        // ensuring the sign is kept positive.
        else {
            let bytes = v.to_le_bytes();

            receiver.binary_begin(Some(bytes.len() + 1))?;
            receiver.binary_fragment_computed(&bytes)?;
            receiver.binary_fragment_computed(&[0])?;
            receiver.binary_end()?;
        }
    }

    receiver.tagged_end(crate::data::tag().for_bigint())
}

macro_rules! digits {
    ($(
        $convert:ident => $ty:ident,
    )+) => {
        $(
            impl Value for $ty {
                fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> crate::Result {
                    receiver.$ty(*self)
                }

                fn $convert(&self) -> Option<$ty> {
                    Some(*self)
                }
            }

            impl<'a> Source<'a> for $ty {
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
                    receiver.$ty(*self)
                }
            }
        )+
    };
}

digits!(
    to_u8 => u8,
    to_u16 => u16,
    to_u32 => u32,
    to_u64 => u64,
    to_u128 => u128,
    to_i8 => i8,
    to_i16 => i16,
    to_i32 => i32,
    to_i64 => i64,
    to_i128 => i128,
    to_f32 => f32,
    to_f64 => f64,
);
