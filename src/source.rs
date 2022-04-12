use crate::{Receiver, Result, Value};

/**
A stateful source of structured data.

# State management

Sources internally maintain whatever state they need to stream a single, complete [value](trait.Receiver.html#values) across potentially many calls to [`Source::stream_resume`].

You can think of sources like the standard `Read` trait, but for structured data instead of raw bytes.
One key difference between `Source` and `Read` is who's in control of the amount of data streamed per call.
For `Read`, that's determined by the caller through their supplied buffer.
For `Source`, that's determined by the source itself.
Each call to [`Source::stream_resume`] is expected to be the smallest reasonable chunk that source can produce.

When sources are passed as arguments it's assumed that they'll be clean: that no calls to [`Source::stream_resume`] have been made yet.

# Rust type and data type must match

A type that implements `Source` must guarantee that it will always stream a single value with the same [data type](trait.Receiver.html#data-types).
For sources that don't know what that data type should be ahead-of-time (if they're parsing a format for example) then they can use the [dynamic data type](trait.Receiver.html#method.dynamic_begin).

# `Value` and `Source` must match

If a type that implements `Source` also implements [`Value`] then the [value](trait.Receiver.html#values) streamed by either implementation must be the same.
*/
pub trait Source<'src> {
    /**
    Stream the next chunk of the source.

    This method returns a [`Resume`] that indicates whether or not the source has any data left.
    If this method returns an error then callers can't assume it will ever return valid data again.

    It's up to the source to decide how much or little of its input to stream.
    It may stream everything in a single call, or it may break on each call to the given receiver.

    Sources are resumable, but still blocking. It must be valid to call this method in a loop and continue to produce data.
    */
    fn stream_resume<'data, R: Receiver<'data>>(&mut self, receiver: R) -> Result<Resume>
    where
        'src: 'data;

    /**
    Stream the rest of the source.

    The default implementation of this method calls [`Source::stream_resume`] in a loop until it returns [`Resume::Done`].
    Implementors are encouraged to override it if they can implement the same functionality more efficiently.
    */
    fn stream_to_end<'data, R: Receiver<'data>>(&mut self, mut receiver: R) -> Result
    where
        'src: 'data,
    {
        while let Resume::Continue = self.stream_resume(&mut receiver)? {}

        Ok(())
    }

    #[inline]
    #[cfg(not(test))]
    fn maybe_dynamic(&self) -> Option<bool> {
        None
    }

    #[cfg(test)]
    fn maybe_dynamic(&self) -> Option<bool>;
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

    fn maybe_dynamic(&self) -> Option<bool> {
        Some(self.is_dynamic())
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
