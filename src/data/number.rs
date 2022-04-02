use crate::{data, source, Receiver, Source, Value};

macro_rules! int {
    ($($fi:ident => $i:ty, $fu:ident => $u:ty,)*) => {
        $(
            pub(crate) fn $fi<'a>(v: $i, mut receiver: impl Receiver<'a>) -> crate::Result {
                receiver.int_begin()?;

                if receiver.is_text_based() {
                    data::text(v).stream_to_end(&mut receiver)?;
                } else {
                    let bytes = v.to_le_bytes();

                    data::binary(&bytes).stream_to_end(data::computed(&mut receiver))?;
                }

                receiver.int_end()
            }

            pub(crate) fn $fu<'a>(v: $u, mut receiver: impl Receiver<'a>) -> crate::Result {
                receiver.int_begin()?;

                if receiver.is_text_based() {
                    data::text(v).stream_to_end(data::computed(&mut receiver))?;
                } else {
                    if v <= (<$i>::MAX as $u) {
                        let mut bytes = [0; (<$u>::BITS as usize / 8) + 1];
                        let unsigned = v.to_le_bytes();

                        bytes[..unsigned.len()].copy_from_slice(&unsigned);

                        data::binary(&bytes).stream_to_end(data::computed(&mut receiver))?;
                    } else {
                        let bytes = v.to_le_bytes();

                        data::binary(&bytes).stream_to_end(data::computed(&mut receiver))?;
                    }
                }

                receiver.int_end()
            }
        )*
    };
}

macro_rules! float {
    ($($f:ident => $n:ty,)*) => {
        $(
            pub(crate) fn $f<'a>(v: $n, mut receiver: impl Receiver<'a>) -> crate::Result {
                receiver.binfloat_begin()?;

                if receiver.is_text_based() {
                    data::text(v).stream_to_end(&mut receiver)?;
                } else {
                    let bytes = v.to_le_bytes();

                    data::binary(&bytes).stream_to_end(data::computed(&mut receiver))?;
                }

                receiver.binfloat_end()
            }
        )*
    };
}

macro_rules! convert {
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

int!(
    i8_int => i8,
    u8_int => u8,
    i16_int => i16,
    u16_int => u16,
    i32_int => i32,
    u32_int => u32,
    i64_int => i64,
    u64_int => u64,
    i128_int => i128,
    u128_int => u128,
);

float!(
    f32_number => f32,
    f64_number => f64,
);

convert!(
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
