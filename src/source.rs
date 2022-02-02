mod source_ref;
mod source_value;

pub use self::{source_ref::*, source_value::*};

use crate::{Receiver, Result};

pub fn stream_to_end<'a>(s: impl Receiver<'a>, mut v: impl Source<'a>) -> Result {
    v.stream_all(s)
}

pub trait Source<'a> {
    fn stream_begin<'b, R: Receiver<'b>>(&mut self, receiver: R) -> Result
    where
        'a: 'b,
    {
        let _ = receiver;
        Ok(())
    }

    fn stream_next<'b, R: Receiver<'b>>(&mut self, receiver: R) -> Result<Next>
    where
        'a: 'b;

    fn stream_end<'b, R: Receiver<'b>>(&mut self, receiver: R) -> Result
    where
        'a: 'b,
    {
        let _ = receiver;
        Ok(())
    }

    fn stream_all<'b, R: Receiver<'b>>(&mut self, mut receiver: R) -> Result
    where
        'a: 'b,
    {
        self.stream_begin()?;
        while let Next::Continue = self.stream_next(&mut receiver)? {}
        self.stream_end()?;

        Ok(())
    }
}

#[must_use]
pub enum Next {
    Continue,
    Done,
}

impl<'a, 'b, T: Source<'a> + ?Sized> Source<'a> for &'b mut T {
    fn stream_next<'c, S: Receiver<'c>>(&mut self, receiver: S) -> Result<Next>
    where
        'a: 'c,
    {
        (**self).stream_next(receiver)
    }

    fn stream_all<'c, R: Receiver<'c>>(&mut self, receiver: R) -> Result
    where
        'a: 'c,
    {
        (**self).stream_all(receiver)
    }
}

impl<'a, T: SourceValue + ?Sized> Source<'a> for &'a T {
    fn stream_next<'b, R: Receiver<'b>>(&mut self, receiver: R) -> Result<Next>
    where
        'a: 'b,
    {
        self.stream_all(receiver).map(|_| Next::Done)
    }

    fn stream_all<'b, R: Receiver<'b>>(&mut self, mut receiver: R) -> Result
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
        fn stream_next<'c, S: Receiver<'c>>(&mut self, receiver: S) -> Result<Next>
        where
            'a: 'c,
        {
            (**self).stream_resume(receiver)
        }

        fn stream_all<'c, R: Receiver<'c>>(&mut self, receiver: R) -> Result
        where
            'a: 'c,
        {
            (**self).stream_to_end(receiver)
        }
    }

    impl<'a, V: ToOwned + SourceValue + ?Sized> Source<'a> for Cow<'a, V> {
        fn stream_next<'b, R: Receiver<'b>>(&mut self, receiver: R) -> crate::Result<source::Next>
        where
            'a: 'b,
        {
            self.stream_all(receiver).map(|_| source::Next::Done)
        }

        fn stream_all<'b, R: Receiver<'b>>(&mut self, receiver: R) -> crate::Result
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

    impl<'a, 'b, V: ToOwned + SourceValue + ?Sized> Source<'a> for &'b Cow<'a, V> {
        fn stream_next<'c, R: Receiver<'c>>(&mut self, receiver: R) -> crate::Result<source::Next>
        where
            'a: 'c,
        {
            self.stream_all(receiver).map(|_| source::Next::Done)
        }

        fn stream_all<'c, R: Receiver<'c>>(&mut self, receiver: R) -> crate::Result
        where
            'a: 'c,
        {
            match self {
                Cow::Borrowed(v) => (*v).stream(receiver),
                Cow::Owned(v) => v.borrow().stream(for_all(receiver)),
            }
        }
    }
}
