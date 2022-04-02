use crate::{source, std::ops, Receiver, Result, Source, Value};

pub fn binary(bytes: &impl AsRef<[u8]>) -> Binary {
    Binary::new(bytes)
}

pub struct Binary<'a>(&'a [u8]);

impl<'a> Binary<'a> {
    pub fn new(bytes: &'a impl AsRef<[u8]>) -> Self {
        Binary(bytes.as_ref())
    }
}

impl<'a> Value for Binary<'a> {
    fn stream<'b, R: Receiver<'b>>(&'b self, mut receiver: R) -> Result {
        receiver.binary(self.0)
    }
}

impl<'a> Source<'a> for Binary<'a> {
    fn stream_resume<'b, R: Receiver<'b>>(&mut self, receiver: R) -> Result<source::Resume>
    where
        'a: 'b,
    {
        self.stream_to_end(receiver).map(|_| source::Resume::Done)
    }

    fn stream_to_end<'b, R: Receiver<'b>>(&mut self, mut receiver: R) -> Result
    where
        'a: 'b,
    {
        receiver.binary(self.0)
    }
}

impl<'a> AsRef<[u8]> for Binary<'a> {
    fn as_ref(&self) -> &[u8] {
        self.0
    }
}

impl<'a> ops::Deref for Binary<'a> {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        self.0
    }
}
