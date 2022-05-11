use std::{fmt, io, str};

mod num;

trait EncodingValue {
    fn encode_text<W: fmt::Write>(&self, writer: W) -> sval::Result;
    fn encode_bytes<W: io::Write>(&self, writer: W) -> sval::Result;

    fn encode<B: EncodingBuffer>(&self, buffer: &mut B) -> sval::Result {
        if buffer.is_text_based() {
            struct Writer<'a, B>(&'a mut B);

            impl<'a, B: EncodingBuffer> fmt::Write for Writer<'a, B> {
                fn write_str(&mut self, s: &str) -> fmt::Result {
                    self.0.push_text(s)?;

                    Ok(())
                }
            }

            self.encode_text(Writer(buffer))
        } else {
            struct Writer<'a, B>(&'a mut B);

            impl<'a, B: EncodingBuffer> io::Write for Writer<'a, B> {
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

    fn decode<B: EncodingBuffer>(buffer: &B) -> sval::Result<Self>
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

    fn to_text(&self) -> sval::Result<&str>;
    fn to_binary(&self) -> sval::Result<&[u8]>;
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

    fn to_text(&self) -> sval::Result<&str> {
        Ok(str::from_utf8(&self)?)
    }

    fn to_binary(&self) -> sval::Result<&[u8]> {
        Ok(&**self)
    }
}
