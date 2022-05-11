use std::{fmt, io, str};

use crate::EncodingValue;
use num_bigint::{BigInt, BigUint};
use num_traits::{Num, ToPrimitive, Zero};

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

#[derive(Debug)]
enum Sign {
    Positive,
    Negative,
}

// All we want to do is translate between text and binary encodings
// We also need to implement equality
// We could consider only storing normalized text and binary strings for floats
// along with enough metadata to tell if they're equal or not
#[derive(Debug)]
enum Float {
    NaN { payload: Vec<u8> },
    Infinity,
    Finite { scale: u64, mantissa: BigUint },
}

#[derive(Debug)]
pub struct DecimalFloat {
    sign: Sign,
    data: Float,
}

#[derive(Debug)]
struct BinaryFloat {
    sign: Sign,
    data: Float,
}

impl EncodingValue for DecimalFloat {
    fn encode_text<W: fmt::Write>(&self, mut writer: W) -> sval::Result {
        match self.data {
            Float::Finite {
                ref scale,
                ref mantissa,
            } => {
                if let Sign::Negative = self.sign {
                    writer.write_str("-")?;
                }

                let mut digits = mantissa.to_radix_be(10);
                for d in &mut digits {
                    *d += b'0';
                }

                let digits = match scale.to_usize().ok_or(sval::Error::unsupported())? {
                    // If the scale is zero then we don't have a decimal place to insert
                    0 => digits,
                    // If the scale is within the digits then insert it at the right place
                    scale if scale < digits.len() => {
                        let index = digits.len() - scale;
                        digits.insert(index, b'.');

                        digits
                    }
                    // If the scale is past the digits then fill the front with zeroes
                    scale => {
                        let mut scaled_digits = Vec::new();

                        scaled_digits.extend_from_slice(b"0.");
                        scaled_digits.extend(std::iter::repeat(b'0').take(scale - digits.len()));
                        scaled_digits.append(&mut digits);

                        scaled_digits
                    }
                };

                writer.write_str(str::from_utf8(&digits)?)?;

                Ok(())
            }
            _ => todo!(),
        }
    }

    fn encode_bytes<W: io::Write>(&self, mut writer: W) -> sval::Result {
        todo!()
    }

    fn decode_text(text: &str) -> sval::Result<Self> {
        let mut buf = text.as_bytes();

        let mut sign = Sign::Positive;

        if buf.starts_with(b"-") {
            sign = Sign::Negative;
            buf = &buf[1..];
        }

        let mut scale = 0;
        let mut scale_step = 0;

        let mut mantissa = BigUint::zero();

        for b in buf {
            match b {
                b'0'..=b'9' => {
                    mantissa *= 10u8;
                    mantissa += b - b'0';

                    scale += 1 * scale_step;
                }
                b'.' if scale_step == 0 => {
                    scale_step = 1;
                }
                _ => return sval::result::unsupported(),
            }
        }

        while scale > 0 && &mantissa % 10u8 == BigUint::zero() {
            scale -= 1;
            mantissa /= 10u8;
        }

        Ok(DecimalFloat {
            sign,
            data: Float::Finite { scale, mantissa },
        })
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

        let it = Int::decode(&text).unwrap();

        let mut binary = Vec::new();
        it.encode(&mut binary).unwrap();

        let ib = Int::decode(&binary).unwrap();

        assert_eq!(it, ib);
    }

    #[test]
    fn decode_decimal_integer() {
        let text = String::from("0001234567890.00000");

        let dt = DecimalFloat::decode(&text).unwrap();

        let mut encoded = String::new();
        dt.encode(&mut encoded).unwrap();

        assert_eq!("1234567890", encoded);
    }

    #[test]
    fn decode_decimal_tiny() {
        let text = String::from("0.000000000000000235732800000");

        let dt = DecimalFloat::decode(&text).unwrap();

        let mut encoded = String::new();
        dt.encode(&mut encoded).unwrap();

        assert_eq!("0.0000000000000002357328", encoded);
    }

    #[test]
    fn decode_decimal() {
        let text = String::from("0001234567890.09876543210000000");

        let dt = DecimalFloat::decode(&text).unwrap();

        let mut encoded = String::new();
        dt.encode(&mut encoded).unwrap();

        assert_eq!("1234567890.0987654321", encoded);
    }

    #[bench]
    fn bench_decode_decimal(b: &mut test::Bencher) {
        let text = String::from("0001234567890.09876543210000000");

        b.iter(|| DecimalFloat::decode(&text).unwrap());
    }

    #[bench]
    fn bench_encode_decimal(b: &mut test::Bencher) {
        let dt = DecimalFloat::decode(&String::from("1234567890.0987654321")).unwrap();

        let mut buf = String::new();
        dt.encode(&mut buf).unwrap();
        buf.clear();

        b.iter(|| {
            dt.encode(&mut buf).unwrap();
            buf.clear();

            buf.len()
        });
    }
}