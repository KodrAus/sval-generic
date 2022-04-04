use crate::{Receiver, Result, Value};

/**
A streamed and resumable source of structured data.

# Implementation notes

Valid implementations of `Source` must adhere to the following requirements:

1. All instances of this type must always stream with the same shape.
2. The result of [`Source::stream_to_end`] must be the same as continually calling [`Source::stream_resume`]
until [`Resume::Done`] is returned. This is guaranteed by the default implementation of [`Source::stream_to_end`].
*/
pub trait Source<'src> {
    fn stream_resume<'data, R: Receiver<'data>>(&mut self, receiver: R) -> Result<Resume>
    where
        'src: 'data;

    fn stream_to_end<'data, R: Receiver<'data>>(&mut self, mut receiver: R) -> Result
    where
        'src: 'data,
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

impl<'src, T: Value + ?Sized> Source<'src> for &'src T {
    fn stream_resume<'data, R: Receiver<'data>>(&mut self, receiver: R) -> Result<Resume>
    where
        'src: 'data,
    {
        self.stream_to_end(receiver).map(|_| Resume::Done)
    }

    fn stream_to_end<'data, R: Receiver<'data>>(&mut self, mut receiver: R) -> Result
    where
        'src: 'data,
    {
        receiver.value(*self)
    }
}

macro_rules! impl_source_forward {
    ({ $($r:tt)* } => $bind:ident => { $($forward:tt)* }) => {
        $($r)* {
            fn stream_resume<'data, S: Receiver<'data>>(&mut self, receiver: S) -> Result<Resume>
            where
                'src: 'data,
            {
                let $bind = self;
                ($($forward)*).stream_resume(receiver)
            }

            fn stream_to_end<'data, R: Receiver<'data>>(&mut self, receiver: R) -> Result
            where
                'src: 'data,
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

impl_source_forward!({impl<'src, 'a, T: Source<'src> + ?Sized> Source<'src> for &'a mut T} => x => { **x });

#[cfg(feature = "alloc")]
mod alloc_support {
    use super::*;

    use crate::{std::boxed::Box, Receiver, Result, Source};

    impl_source_forward!({impl<'src, T: Source<'src> + ?Sized> Source<'src> for Box<T>} => x => { **x });
}
