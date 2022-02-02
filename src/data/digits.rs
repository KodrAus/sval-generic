use crate::{source, Receiver, Source, SourceValue};

macro_rules! digits {
    ($(
        $ty:ident,
    )+) => {
        $(
            impl SourceValue for $ty {
                fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> crate::Result {
                    receiver.$ty(*self)
                }
            }

            impl<'a> Source<'a> for $ty {
                fn stream_next<'b, R: Receiver<'b>>(&mut self, receiver: R) -> crate::Result<source::Next>
                where
                    'a: 'b,
                {
                    self.stream_all(receiver).map(|_| source::Next::Done)
                }

                fn stream_all<'b, R: Receiver<'b>>(&mut self, mut receiver: R) -> crate::Result
                where
                    'a: 'b,
                {
                    receiver.$ty(*self)
                }
            }
        )+
    };
}

digits!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64,);
