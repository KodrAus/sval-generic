use crate::{
    source::{self, ValueSource},
    std::{
        fmt,
        ops::{Deref, DerefMut},
        str,
    },
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

impl fmt::Display for Bytes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Unstructured encoding for bytes is base64
        base64(&self.0, f)
    }
}

fn base64(src: &[u8], mut w: impl fmt::Write) -> fmt::Result {
    fn encode(src: &[u8; 3]) -> [u8; 4] {
        // The base64 alphabet that inputs are encoded into
        // Each group of 3 arbitrary bytes is encoded as 4 ASCII bytes
        const BASE_64: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
        const MASK: u32 = 0b00000000_00000000_00000000_00111111;

        let incoming_group = ((src[0] as u32) << 16) | ((src[1] as u32) << 8) | (src[2] as u32);

        [
            BASE_64[((incoming_group >> 18) & MASK) as usize],
            BASE_64[((incoming_group >> 12) & MASK) as usize],
            BASE_64[((incoming_group >> 6) & MASK) as usize],
            BASE_64[(incoming_group & MASK) as usize],
        ]
    }

    // Work through each group of 3 bytes in the input together
    let mut i = 3;
    while i <= src.len() {
        w.write_str(
            str::from_utf8(&encode(
                &src[i - 3..i].try_into().expect("infallible conversion"),
            ))
            .expect("invalid UTF8"),
        )?;

        i += 3;
    }

    // Deal with the remaining bytes
    // These are null-padded to the right length and then encoded
    match i - src.len() {
        1 => {
            let mut encoded = encode(&[src[i - 3], src[i - 2], b'\0']);
            encoded[3] = b'=';

            w.write_str(str::from_utf8(&encoded).expect("invalid UTF8"))
        }
        2 => {
            let mut encoded = encode(&[src[i - 3], b'\0', b'\0']);
            encoded[2] = b'=';
            encoded[3] = b'=';

            w.write_str(str::from_utf8(&encoded).expect("invalid UTF8"))
        }
        0 | 3 => Ok(()),
        _ => unreachable!(),
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

impl<'a> ValueSource<'a, Bytes> for &'a [u8] {
    type Error = source::Impossible;

    #[inline]
    fn take(&mut self) -> Result<&Bytes, source::TakeError<Self::Error>> {
        Ok(bytes(self))
    }

    #[inline]
    fn try_take_ref(&mut self) -> Result<&'a Bytes, source::TryTakeError<&Bytes, Self::Error>> {
        Ok(bytes(*self))
    }
}

impl<'a, const N: usize> ValueSource<'a, Bytes> for &'a [u8; N] {
    type Error = source::Impossible;

    #[inline]
    fn take(&mut self) -> Result<&Bytes, source::TakeError<Self::Error>> {
        Ok(bytes(self))
    }

    #[inline]
    fn try_take_ref(&mut self) -> Result<&'a Bytes, source::TryTakeError<&Bytes, Self::Error>> {
        Ok(bytes(*self))
    }
}

impl<'a> ValueSource<'a, Bytes> for &'a str {
    type Error = crate::Error;

    #[inline]
    fn take(&mut self) -> Result<&Bytes, source::TakeError<Self::Error>> {
        Ok(bytes(self))
    }

    #[inline]
    fn try_take_ref(&mut self) -> Result<&'a Bytes, source::TryTakeError<&Bytes, Self::Error>> {
        Ok(bytes(*self))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base64_encode() {
        for &(input, expected) in &[
            (
                "",
                ""
            ),
            (
                "7",
                "Nw==",
            ),
            (
                "ab",
                "YWI=",
            ),
            (
                "8bj",
                "OGJq",
            ),
            (
                "hliuvrahuloisegrm8opig y98pw45yt9849pmg s8y v xjn zd;lfgseo;,",
                "aGxpdXZyYWh1bG9pc2Vncm04b3BpZyB5OThwdzQ1eXQ5ODQ5cG1nIHM4eSB2IHhqbiB6ZDtsZmdzZW87LA==",
            ),
        ] {
            let mut actual = String::new();
            base64(input.as_bytes(), &mut actual).expect("failed to encode");

            assert_eq!(expected, &*actual);
        }
    }
}
