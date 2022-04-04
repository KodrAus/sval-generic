use crate::{
    std::{mem, ops},
    Receiver, Result, Value,
};

pub fn binary(bytes: &[u8]) -> &Binary {
    Binary::new(bytes)
}

#[repr(transparent)]
pub struct Binary([u8]);

impl Binary {
    pub fn new<'a>(bytes: &'a [u8]) -> &'a Self {
        unsafe { mem::transmute::<&'a [u8], &'a Binary>(bytes) }
    }
}

impl Value for Binary {
    fn stream<'b, R: Receiver<'b>>(&'b self, mut receiver: R) -> Result {
        receiver.binary(&self.0)
    }
}

impl AsRef<[u8]> for Binary {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl ops::Deref for Binary {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        &self.0
    }
}
