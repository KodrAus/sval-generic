use crate::{
    data,
    source::{self, ValueSource},
    Result,
};

impl<'a> ValueSource<'a, data::Bytes> for &'a [u8] {
    type Error = source::Impossible;

    #[inline]
    fn take(&mut self) -> Result<&data::Bytes, source::TakeError<Self::Error>> {
        Ok(data::bytes(self))
    }

    #[inline]
    fn take_ref(
        &mut self,
    ) -> Result<&'a data::Bytes, source::TakeRefError<&data::Bytes, Self::Error>> {
        Ok(data::bytes(*self))
    }
}

impl<'a, const N: usize> ValueSource<'a, data::Bytes> for &'a [u8; N] {
    type Error = source::Impossible;

    #[inline]
    fn take(&mut self) -> Result<&data::Bytes, source::TakeError<Self::Error>> {
        Ok(data::bytes(self))
    }

    #[inline]
    fn take_ref(
        &mut self,
    ) -> Result<&'a data::Bytes, source::TakeRefError<&data::Bytes, Self::Error>> {
        Ok(data::bytes(*self))
    }
}

impl<'a> ValueSource<'a, data::Bytes> for &'a str {
    type Error = crate::Error;

    #[inline]
    fn take(&mut self) -> Result<&data::Bytes, source::TakeError<Self::Error>> {
        Ok(data::bytes(self))
    }

    #[inline]
    fn take_ref(
        &mut self,
    ) -> Result<&'a data::Bytes, source::TakeRefError<&data::Bytes, Self::Error>> {
        Ok(data::bytes(*self))
    }
}
