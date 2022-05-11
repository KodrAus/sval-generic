#![feature(test)]

extern crate test;

use std::{fmt, io, str};

mod num;

trait EncodingValue {
    fn encode_text<W: fmt::Write>(&self, writer: W) -> sval::Result;
    fn encode_bytes<W: io::Write>(&self, writer: W) -> sval::Result;

    fn encode_to_string(&self) -> sval::Result<String> {
        let mut text = String::new();
        self.encode(&mut text)?;

        Ok(text)
    }

    fn encode_to_vec(&self) -> sval::Result<Vec<u8>> {
        let mut binary = Vec::new();
        self.encode(&mut binary)?;

        Ok(binary)
    }

    fn encode<B: EncodingBuffer>(&self, mut buffer: B) -> sval::Result {
        if buffer.is_text_based() {
            struct Writer<B>(B);

            impl<B: EncodingBuffer> fmt::Write for Writer<B> {
                fn write_str(&mut self, s: &str) -> fmt::Result {
                    self.0.push_text(s)?;

                    Ok(())
                }
            }

            self.encode_text(Writer(buffer))
        } else {
            struct Writer<B>(B);

            impl<B: EncodingBuffer> io::Write for Writer<B> {
                fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
                    self.0.push_binary(buf)?;

                    Ok(buf.len())
                }

                fn flush(&mut self) -> io::Result<()> {
                    Ok(())
                }
            }

            self.encode_bytes(Writer(buffer))
        }
    }

    fn decode_text(text: &str) -> sval::Result<Self>
    where
        Self: Sized;
    fn decode_binary(binary: &[u8]) -> sval::Result<Self>
    where
        Self: Sized;

    fn decode<B: DecodingBuffer>(buffer: B) -> sval::Result<Self>
    where
        Self: Sized,
    {
        if buffer.is_text_based() {
            Self::decode_text(buffer.to_text()?)
        } else {
            Self::decode_binary(buffer.to_binary()?)
        }
    }
}

trait EncodingBuffer {
    fn is_text_based(&self) -> bool;

    fn push_text(&mut self, text: &str) -> sval::Result;
    fn push_binary(&mut self, binary: &[u8]) -> sval::Result;
}

impl<'a, B: EncodingBuffer + ?Sized> EncodingBuffer for &'a mut B {
    fn is_text_based(&self) -> bool {
        (**self).is_text_based()
    }

    fn push_text(&mut self, text: &str) -> sval::Result {
        (**self).push_text(text)
    }

    fn push_binary(&mut self, binary: &[u8]) -> sval::Result {
        (**self).push_binary(binary)
    }
}

trait DecodingBuffer {
    fn is_text_based(&self) -> bool;

    fn to_text(&self) -> sval::Result<&str>;
    fn to_binary(&self) -> sval::Result<&[u8]>;
}

impl<'a, B: DecodingBuffer + ?Sized> DecodingBuffer for &'a B {
    fn is_text_based(&self) -> bool {
        (**self).is_text_based()
    }

    fn to_text(&self) -> sval::Result<&str> {
        (**self).to_text()
    }

    fn to_binary(&self) -> sval::Result<&[u8]> {
        (**self).to_binary()
    }
}

impl EncodingBuffer for String {
    fn is_text_based(&self) -> bool {
        true
    }

    fn push_text(&mut self, text: &str) -> sval::Result {
        self.push_str(text);

        Ok(())
    }

    fn push_binary(&mut self, binary: &[u8]) -> sval::Result {
        self.push_text(str::from_utf8(binary)?)
    }
}

impl DecodingBuffer for String {
    fn is_text_based(&self) -> bool {
        (**self).is_text_based()
    }

    fn to_text(&self) -> sval::Result<&str> {
        (**self).to_text()
    }

    fn to_binary(&self) -> sval::Result<&[u8]> {
        (**self).to_binary()
    }
}

impl DecodingBuffer for str {
    fn is_text_based(&self) -> bool {
        true
    }

    fn to_text(&self) -> sval::Result<&str> {
        Ok(self)
    }

    fn to_binary(&self) -> sval::Result<&[u8]> {
        Ok(self.as_bytes())
    }
}

impl EncodingBuffer for Vec<u8> {
    fn is_text_based(&self) -> bool {
        false
    }

    fn push_text(&mut self, text: &str) -> sval::Result {
        self.push_binary(text.as_bytes())
    }

    fn push_binary(&mut self, binary: &[u8]) -> sval::Result {
        self.extend(binary);

        Ok(())
    }
}

impl DecodingBuffer for Vec<u8> {
    fn is_text_based(&self) -> bool {
        (**self).is_text_based()
    }

    fn to_text(&self) -> sval::Result<&str> {
        (**self).to_text()
    }

    fn to_binary(&self) -> sval::Result<&[u8]> {
        (**self).to_binary()
    }
}

impl DecodingBuffer for [u8] {
    fn is_text_based(&self) -> bool {
        false
    }

    fn to_text(&self) -> sval::Result<&str> {
        Ok(str::from_utf8(self)?)
    }

    fn to_binary(&self) -> sval::Result<&[u8]> {
        Ok(self)
    }
}
