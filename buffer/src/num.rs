use std::{fmt, io};

use crate::EncodingValue;
use num_bigint::BigInt;
use num_traits::Num;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Int(BigInt);

impl EncodingValue for Int {
    fn encode_text<W: fmt::Write>(&self, mut writer: W) -> sval::Result {
        writer.write_fmt(format_args!("{}", self.0))?;

        Ok(())
    }

    fn encode_bytes<W: io::Write>(&self, mut writer: W) -> sval::Result {
        let bytes = self.0.to_signed_bytes_le();
        writer.write_all(&bytes)?;

        Ok(())
    }

    fn decode_text(text: &str) -> sval::Result<Self> {
        let int = BigInt::from_str_radix(text, 10).map_err(|_| sval::Error::unsupported())?;

        Ok(Int(int))
    }

    fn decode_binary(binary: &[u8]) -> sval::Result<Self> {
        let int = BigInt::from_signed_bytes_le(binary);

        Ok(Int(int))
    }
}

enum Sign {
    Positive,
    Negative,
}

// All we want to do is translate between text and binary encodings
// We also need to implement equality
enum Float {
    NaN { payload: Vec<u8> },
    Infinity,
    Finite { exp: BigInt, mantissa: BigInt },
}

pub struct DecimalFloat {
    sign: Sign,
    data: Float,
}

pub struct BinaryFloat {
    sign: Sign,
    data: Float,
}

impl EncodingValue for DecimalFloat {
    fn encode_text<W: fmt::Write>(&self, mut writer: W) -> sval::Result {
        todo!()
    }

    fn encode_bytes<W: io::Write>(&self, mut writer: W) -> sval::Result {
        todo!()
    }

    fn decode_text(text: &str) -> sval::Result<Self> {
        todo!()
    }

    fn decode_binary(binary: &[u8]) -> sval::Result<Self> {
        todo!()
    }
}

impl EncodingValue for BinaryFloat {
    fn encode_text<W: fmt::Write>(&self, mut writer: W) -> sval::Result {
        todo!()
    }

    fn encode_bytes<W: io::Write>(&self, mut writer: W) -> sval::Result {
        todo!()
    }

    fn decode_text(text: &str) -> sval::Result<Self> {
        todo!()
    }

    fn decode_binary(binary: &[u8]) -> sval::Result<Self> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode_int() {
        let text = String::from("1235");

        let from_text = Int::decode(&text).unwrap();

        let mut binary = Vec::new();
        from_text.encode(&mut binary).unwrap();

        let from_binary = Int::decode(&binary).unwrap();

        assert_eq!(from_text, from_binary);
    }
}
