mod binary;
mod for_all;
mod number;
mod seq;
mod tag;
mod text;

use crate::{source, Receiver, Source, Value};

#[doc(inline)]
pub use self::{binary::*, for_all::*, tag::*, text::*};

impl Value for () {
    fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> crate::Result {
        receiver.null()
    }
}

impl<'a> Source<'a> for () {
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
        receiver.null()
    }
}

impl<T: Value> Value for Option<T> {
    fn stream<'a, R: Receiver<'a>>(&'a self, receiver: R) -> crate::Result {
        self.as_ref().stream_to_end(receiver)
    }
}

impl<'a, T: Source<'a>> Source<'a> for Option<T> {
    fn stream_resume<'b, R: Receiver<'b>>(&mut self, receiver: R) -> crate::Result<source::Resume>
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
            None => tagged((), tag().for_nullable().with_id(0).with_label("None"))
                .stream_to_end(receiver),
            Some(v) => tagged(v, tag().for_nullable().with_id(1).with_label("Some"))
                .stream_to_end(receiver),
        }
    }
}

impl Value for bool {
    fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> crate::Result {
        receiver.bool(*self)
    }
}

impl<'a> Source<'a> for bool {
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
        receiver.bool(*self)
    }
}
