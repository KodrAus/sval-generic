use crate::{
    std::ops::{Deref, DerefMut},
    Receiver, Result, Value,
};

pub fn bytes(bytes: &(impl AsRef<[u8]> + ?Sized)) -> &Bytes {
    Bytes::new(bytes)
}

#[repr(transparent)]
pub struct Bytes([u8]);

impl Bytes {
    pub fn new<'a>(bytes: &'a (impl AsRef<[u8]> + ?Sized)) -> &'a Bytes {
        let bytes: &'a [u8] = bytes.as_ref();

        // SAFETY: `Bytes` and `[u8]` have the same ABI
        unsafe { &*(bytes as *const [u8] as *const Bytes) }
    }
}

impl Deref for Bytes {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        &self.0
    }
}

impl DerefMut for Bytes {
    fn deref_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }
}

impl Value for Bytes {
    fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> Result {
        receiver.bytes(self)
    }
}
