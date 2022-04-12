use crate::{
    std::{mem, ops},
    Receiver, Result, Value,
};

pub fn bytes(bytes: &[u8]) -> &Bytes {
    Bytes::new_ref(bytes)
}

#[repr(transparent)]
pub struct Bytes([u8]);

impl Bytes {
    pub fn new_ref(bytes: &[u8]) -> &Self {
        unsafe { mem::transmute::<&[u8], &Bytes>(bytes) }
    }
}

impl Value for Bytes {
    fn stream<'b, R: Receiver<'b>>(&'b self, mut receiver: R) -> Result {
        receiver.binary(&self.0)
    }

    fn to_binary(&self) -> Option<&[u8]> {
        Some(&**self)
    }
}

impl AsRef<[u8]> for Bytes {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl ops::Deref for Bytes {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        &self.0
    }
}
