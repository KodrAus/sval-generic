use crate::{source, Receiver, Source, Value};

#[inline]
pub fn binary<T: AsRef<[u8]>>(value: T) -> Binary<T> {
    Binary::new(value)
}

pub struct Binary<T>(T);

impl<T: AsRef<[u8]>> AsRef<[u8]> for Binary<T> {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl<T: AsRef<[u8]>> Value for Binary<T> {
    fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> crate::Result {
        let bytes = self.0.as_ref();

        receiver.binary_begin(Some(bytes.len() as u64))?;
        receiver.binary_fragment(bytes)?;
        receiver.binary_end()
    }
}

impl<'a, T: AsRef<[u8]>> Source<'a> for Binary<T> {
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
        let bytes = self.0.as_ref();

        receiver.binary_begin(Some(bytes.len() as u64))?;
        receiver.binary_fragment(bytes)?;
        receiver.binary_end()
    }
}

impl<T> Binary<T> {
    pub fn new(value: T) -> Self {
        Binary(value)
    }

    pub fn by_ref(&self) -> Binary<&T> {
        Binary(&self.0)
    }

    pub fn by_mut(&mut self) -> Binary<&mut T> {
        Binary(&mut self.0)
    }

    pub fn into_inner(self) -> T {
        self.0
    }
}
