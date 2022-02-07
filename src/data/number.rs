use crate::{source, Receiver, Source, Value};

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

digits!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64,);
