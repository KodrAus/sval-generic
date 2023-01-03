use crate::{tags, Stream, Value};

macro_rules! int {
    ($($fi:ident => $i:ty, $fu:ident => $u:ty,)*) => {
        $(
            pub(crate) fn $fi<'sval>(v: $i, stream: &mut (impl Stream<'sval> + ?Sized)) -> crate::Result {
                stream.tagged_begin(Some(tags::NUMBER), None, None)?;

                crate::stream_fmt(stream, v)?;

                stream.tagged_end(Some(tags::NUMBER), None, None)
            }

            pub(crate) fn $fu<'sval>(v: $u, stream: &mut (impl Stream<'sval> + ?Sized)) -> crate::Result {
                stream.tagged_begin(Some(tags::NUMBER), None, None)?;

                crate::stream_fmt(stream, v)?;

                stream.tagged_end(Some(tags::NUMBER), None, None)
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
    stream_i128 => i128,
    stream_u128 => u128,
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

#[cfg(test)]
mod tests {
    #[test]
    fn number_cast() {
        todo!()
    }
}
