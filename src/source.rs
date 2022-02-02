mod source_ref;
mod source_value;

pub use self::{source_ref::*, source_value::*};

use crate::{Receiver, Result};

pub fn stream_to_end<'a>(s: impl Receiver<'a>, mut v: impl Source<'a>) -> Result {
    v.stream_to_end(s)
}

pub trait Source<'a> {
    fn stream_resume<'b, R: Receiver<'b>>(&mut self, receiver: R) -> Result<Resume>
    where
        'a: 'b;

    fn stream_to_end<'b, R: Receiver<'b>>(&mut self, mut receiver: R) -> Result
    where
        'a: 'b,
    {
        while let Resume::Yield = self.stream_resume(&mut receiver)? {}

        Ok(())
    }
}

#[must_use]
pub enum Resume {
    Yield,
    Done,
}

impl<'a, 'b, T: Source<'a> + ?Sized> Source<'a> for &'b mut T {
    fn stream_resume<'c, S: Receiver<'c>>(&mut self, receiver: S) -> Result<Resume>
    where
        'a: 'c,
    {
        (**self).stream_resume(receiver)
    }

    fn stream_to_end<'c, R: Receiver<'c>>(&mut self, receiver: R) -> Result
    where
        'a: 'c,
    {
        (**self).stream_to_end(receiver)
    }
}

impl<'a, T: SourceValue + ?Sized> Source<'a> for &'a T {
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

#[cfg(feature = "alloc")]
mod alloc_support {
    use super::*;

    use crate::{
        for_all, source,
        std::{
            borrow::{Borrow, Cow, ToOwned},
            boxed::Box,
        },
        Receiver, Result, Source, SourceValue,
    };

    impl<'a, T: Source<'a> + ?Sized> Source<'a> for Box<T> {
        fn stream_resume<'c, S: Receiver<'c>>(&mut self, receiver: S) -> Result<Resume>
        where
            'a: 'c,
        {
            (**self).stream_resume(receiver)
        }

        fn stream_to_end<'c, R: Receiver<'c>>(&mut self, receiver: R) -> Result
        where
            'a: 'c,
        {
            (**self).stream_to_end(receiver)
        }
    }

    impl<'a, V: ToOwned + SourceValue + ?Sized> Source<'a> for Cow<'a, V> {
        fn stream_resume<'b, R: Receiver<'b>>(
            &mut self,
            receiver: R,
        ) -> crate::Result<source::Resume>
        where
            'a: 'b,
        {
            self.stream_to_end(receiver).map(|_| source::Resume::Done)
        }

        fn stream_to_end<'b, R: Receiver<'b>>(&mut self, receiver: R) -> crate::Result
        where
            'a: 'b,
        {
            match self {
                Cow::Borrowed(v) => v.stream(receiver),
                Cow::Owned(v) => {
                    let v: &V = (*v).borrow();
                    v.stream(for_all(receiver))
                }
            }
        }
    }
}
