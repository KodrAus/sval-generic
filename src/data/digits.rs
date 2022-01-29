use crate::{
    source::{self, ValueSource},
    Receiver, Result, Source, Value,
};

macro_rules! digits {
    ($(
        $ty:ident,
    )+) => {
        $(
            impl Value for $ty {
                fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> crate::Result {
                    receiver.$ty(*self)
                }
            }

            impl<'a> Source<'a> for $ty {
                fn stream_resume<'b, R: Receiver<'b>>(&mut self, receiver: R) -> crate::Result<source::Stream>
                where
                    'a: 'b,
                {
                    self.stream_to_end(receiver).map(|_| source::Stream::Done)
                }

                fn stream_to_end<'b, R: Receiver<'b>>(&mut self, mut receiver: R) -> crate::Result
                where
                    'a: 'b,
                {
                    receiver.$ty(*self)
                }
            }

            impl<'a> ValueSource<'a, $ty> for $ty {
                type Error = source::Impossible;

                #[inline]
                fn take(&mut self) -> Result<&$ty, source::TakeError<Self::Error>> {
                    Ok(self)
                }
            }
        )+
    };
}

digits!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64,);
