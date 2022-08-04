use crate::{Stream, Value};

macro_rules! int {
    ($($fi:ident => $i:ty, $fu:ident => $u:ty,)*) => {
        $(
            pub(crate) fn $fi<'sval>(v: $i, stream: &mut (impl Stream<'sval> + ?Sized)) -> crate::Result {
                stream.number_begin()?;

                crate::data::text::display(v, stream)?;

                stream.number_end()
            }

            pub(crate) fn $fu<'sval>(v: $u, stream: &mut (impl Stream<'sval> + ?Sized)) -> crate::Result {
                stream.number_begin()?;

                crate::data::text::display(v, stream)?;

                stream.number_end()
            }
        )*
    };
}

macro_rules! float {
    ($($f:ident => $n:ty,)*) => {
        $(
            pub(crate) fn $f<'sval>(v: $n, stream: &mut (impl Stream<'sval> + ?Sized)) -> crate::Result {
                stream.number_begin()?;

                crate::data::text::display(v, stream)?;

                stream.number_end()
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
                fn stream<'sval, S: Stream<'sval> + ?Sized>(&'sval self, stream: &mut S) -> crate::Result {
                    stream.$ty(*self)
                }

                fn $convert(&self) -> Option<$ty> {
                    Some(*self)
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
