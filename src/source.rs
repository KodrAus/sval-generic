use crate::{Receiver, Result, Value};

/**
A streamed and resumable source of structured data.

# Implementation notes

Valid implementations of `Source` must adhere to the following requirements:

1. All instances of this type must always stream with the same shape.
2. The result of [`Source::stream_to_end`] must be the same as continually calling [`Source::stream_resume`]
until [`Resume::Done`] is returned. This is guaranteed by the default implementation of [`Source::stream_to_end`].
*/
pub trait Source<'a> {
    fn stream_resume<'b, R: Receiver<'b>>(&mut self, receiver: R) -> Result<Resume>
    where
        'a: 'b;

    fn stream_to_end<'b, R: Receiver<'b>>(&mut self, mut receiver: R) -> Result
    where
        'a: 'b,
    {
        while let Resume::Continue = self.stream_resume(&mut receiver)? {}

        Ok(())
    }

    #[inline]
    fn maybe_dynamic(&self) -> Option<bool> {
        None
    }
}

#[must_use]
pub enum Resume {
    Continue,
    Done,
}

impl Resume {
    pub fn is_continue(&self) -> bool {
        matches!(self, Resume::Continue)
    }

    pub fn is_done(&self) -> bool {
        matches!(self, Resume::Done)
    }
}

impl<'a, T: Value + ?Sized> Source<'a> for &'a T {
    fn stream_resume<'b, R: Receiver<'b>>(&mut self, receiver: R) -> Result<Resume>
    where
        'a: 'b,
    {
        self.stream_to_end(receiver).map(|_| Resume::Done)
    }

    fn stream_to_end<'b, R: Receiver<'b>>(&mut self, mut receiver: R) -> Result
    where
        'a: 'b,
    {
        receiver.value(*self)
    }
}

macro_rules! impl_source_forward {
    ({ $($r:tt)* } => $bind:ident => { $($forward:tt)* }) => {
        $($r)* {
            fn stream_resume<'c, S: Receiver<'c>>(&mut self, receiver: S) -> Result<Resume>
            where
                'a: 'c,
            {
                let $bind = self;
                ($($forward)*).stream_resume(receiver)
            }

            fn stream_to_end<'c, R: Receiver<'c>>(&mut self, receiver: R) -> Result
            where
                'a: 'c,
            {
                let $bind = self;
                ($($forward)*).stream_to_end(receiver)
            }

            #[inline]
            fn maybe_dynamic(&self) -> Option<bool> {
                let $bind = self;
                ($($forward)*).maybe_dynamic()
            }
        }
    };
}

impl_source_forward!({impl<'a, 'b, T: Source<'a> + ?Sized> Source<'a> for &'b mut T} => x => { **x });

#[cfg(feature = "alloc")]
mod alloc_support {
    use super::*;

    use crate::{std::boxed::Box, Receiver, Result, Source};

    impl_source_forward!({impl<'a, T: Source<'a> + ?Sized> Source<'a> for Box<T>} => x => { **x });
}
