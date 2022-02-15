use crate::{Receiver, Result, Value};

// Implementation: The `'a` lifetime needs to be bounded by the target type
// This can be wrapped, as in `Box<impl SourceRef<'a>>` or external as in
// `&'a impl SourceValue` and `Cow<'a, impl SourceValue>`
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
