use std::borrow::Cow;

use std::{
    fmt::{self, Write as _},
    vec,
};

mod permute;
pub mod some;

#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum Token<'a> {
    Unit,
    Null,
    Bool(bool),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    F32(f32),
    F64(f64),
    Text(&'a str),
    TextBegin(Option<usize>),
    TextFragment(&'a str),
    TextFragmentComputed(Cow<'a, str>),
    TextEnd,
    Binary(&'a [u8]),
    BinaryBegin(Option<usize>),
    BinaryFragment(&'a [u8]),
    BinaryFragmentComputed(Cow<'a, [u8]>),
    BinaryEnd,
    MapBegin(Option<usize>),
    MapKeyBegin,
    MapKeyEnd,
    MapValueBegin,
    MapValueEnd,
    MapEnd,
    MapKey(Vec<Token<'a>>),
    MapValue(Vec<Token<'a>>),
    SeqBegin(Option<usize>),
    SeqValueBegin,
    SeqValueEnd,
    SeqEnd,
    SeqValue(Vec<Token<'a>>),
    DynamicBegin,
    DynamicEnd,
}

pub struct Source<'sval> {
    tokens: vec::IntoIter<Vec<Token<'sval>>>,
    window_size: usize,
    dynamic: Option<bool>,
}

impl<'sval> Source<'sval> {
    pub fn tokens<'a>(&'a self) -> impl Iterator<Item = Token<'sval>> + 'a {
        self.tokens
            .as_slice()
            .iter()
            .flat_map(|chunk| chunk.iter().cloned())
    }
}

pub fn assert() -> Assert {
    Default::default()
}

#[non_exhaustive]
pub struct Assert {
    pub basic_model: bool,
}

impl Default for Assert {
    fn default() -> Self {
        Assert { basic_model: false }
    }
}

impl Assert {
    pub fn in_basic_model(mut self) -> Self {
        self.basic_model = true;
        self
    }

    pub fn in_extended_model(mut self) -> Self {
        self.basic_model = false;
        self
    }

    pub fn stream_equal<'sval>(
        &self,
        source: &'sval (impl sval::Value),
        expected: &[Token<'sval>],
    ) {
        let mut reporting = Reporting {
            expected: String::new(),
            actual: String::new(),
            write_separator: false,
            error: None,
        };

        Expect::stream_to_end(source, &mut reporting, self, expected).expect("invalid source");

        if let Some(error) = reporting.error {
            panic!(
                "invalid stream: {}\nvalid to: `{}`",
                error, reporting.expected
            );
        } else {
            assert_eq!(reporting.expected, reporting.actual);
        }
    }
}

struct Reporting {
    expected: String,
    actual: String,
    write_separator: bool,
    error: Option<String>,
}

impl Reporting {
    fn push_token(&mut self, expected: &Token, actual: &Token) {
        Self::push(
            self.error.is_some(),
            self.write_separator,
            &mut self.expected,
            format_args!("{:?}", expected),
        );

        Self::push(
            self.error.is_some(),
            self.write_separator,
            &mut self.actual,
            format_args!("{:?}", actual),
        );

        self.write_separator = true;
    }

    fn begin_nested(&mut self, nested: &str) {
        Self::push(
            self.error.is_some(),
            self.write_separator,
            &mut self.expected,
            format_args!("{}([", nested),
        );
        Self::push(
            self.error.is_some(),
            self.write_separator,
            &mut self.actual,
            format_args!("{}([", nested),
        );

        self.write_separator = false;
    }

    fn end_nested(&mut self) {
        Self::push(self.error.is_some(), false, &mut self.expected, "])");
        Self::push(self.error.is_some(), false, &mut self.actual, "])");

        self.write_separator = true;
    }

    fn push(failed: bool, write_separator: bool, src: &mut String, value: impl fmt::Display) {
        if !failed {
            if write_separator {
                src.push_str(", ");
            }

            write!(src, "{}", value).unwrap();
        }
    }

    fn fail_unexpected(&mut self, expected: &Token, actual: impl fmt::Display) {
        if self.error.is_none() {
            self.error = Some(format!("unexpected {}, expected {:?}: this means the source produced some data that didn't align with the static list of tokens", actual, expected));
        }
    }

    fn fail_end_of_stream(&mut self, actual: impl fmt::Display) {
        if self.error.is_none() {
            self.error = Some(format!("unexpected {}, expected end of stream: this means the source produced more data than the static list of tokens", actual));
        }
    }
}

struct Expect<'data, 'brw> {
    reporting: &'brw mut Reporting,
    assert: &'brw Assert,
    tokens: &'brw [Token<'data>],
}

impl<'data, 'brw> Expect<'data, 'brw> {
    fn stream_to_end<'sval>(
        mut source: impl sval::Source<'sval>,
        reporting: &mut Reporting,
        assert: &Assert,
        tokens: &[Token<'data>],
    ) -> sval::Result
    where
        'sval: 'data,
    {
        let mut expect = Expect {
            assert,
            reporting,
            tokens,
        };

        if expect.assert.basic_model {
            source.stream_to_end(Basic(&mut expect))?;
        } else {
            source.stream_to_end(&mut expect)?;
        }

        Ok(())
    }

    fn advance(&mut self) {
        if self.tokens.len() > 1 {
            self.tokens = &self.tokens[1..];
        } else {
            self.tokens = &[];
        }
    }

    fn expect(&mut self, token: Token) -> sval::Result {
        match self.tokens.get(0) {
            Some(expected) => {
                self.reporting.push_token(expected, &token);
                self.advance();

                Ok(())
            }
            None => {
                self.reporting
                    .fail_end_of_stream(format_args!("{:?}", token));

                Ok(())
            }
        }
    }
}

impl<'data, 'b> sval::Stream<'data> for Expect<'data, 'b> {
    fn unit(&mut self) -> sval::Result {
        self.expect(Token::Unit)
    }

    fn null(&mut self) -> sval::Result {
        self.expect(Token::Null)
    }

    fn bool(&mut self, value: bool) -> sval::Result {
        self.expect(Token::Bool(value))
    }

    fn text_begin(&mut self, num_bytes_hint: Option<usize>) -> sval::Result {
        self.expect(Token::TextBegin(num_bytes_hint))
    }

    fn text_fragment(&mut self, fragment: &'data str) -> sval::Result {
        self.expect(Token::TextFragment(fragment))
    }

    fn text_fragment_computed(&mut self, fragment: &str) -> sval::Result {
        self.expect(Token::TextFragmentComputed(Cow::Borrowed(fragment)))
    }

    fn text_end(&mut self) -> sval::Result {
        self.expect(Token::TextEnd)
    }

    fn text(&mut self, value: &'data str) -> sval::Result {
        self.expect(Token::Text(value))
    }

    fn binary_begin(&mut self, num_bytes_hint: Option<usize>) -> sval::Result {
        self.expect(Token::BinaryBegin(num_bytes_hint))
    }

    fn binary_fragment(&mut self, fragment: &'data [u8]) -> sval::Result {
        self.expect(Token::BinaryFragment(fragment))
    }

    fn binary_fragment_computed(&mut self, fragment: &[u8]) -> sval::Result {
        self.expect(Token::BinaryFragmentComputed(Cow::Borrowed(fragment)))
    }

    fn binary_end(&mut self) -> sval::Result {
        self.expect(Token::BinaryEnd)
    }

    fn binary(&mut self, value: &'data [u8]) -> sval::Result {
        self.expect(Token::Binary(value))
    }

    fn u8(&mut self, value: u8) -> sval::Result {
        self.expect(Token::U8(value))
    }

    fn u16(&mut self, value: u16) -> sval::Result {
        self.expect(Token::U16(value))
    }

    fn u32(&mut self, value: u32) -> sval::Result {
        self.expect(Token::U32(value))
    }

    fn u64(&mut self, value: u64) -> sval::Result {
        self.expect(Token::U64(value))
    }

    fn u128(&mut self, value: u128) -> sval::Result {
        self.expect(Token::U128(value))
    }

    fn i8(&mut self, value: i8) -> sval::Result {
        self.expect(Token::I8(value))
    }

    fn i16(&mut self, value: i16) -> sval::Result {
        self.expect(Token::I16(value))
    }

    fn i32(&mut self, value: i32) -> sval::Result {
        self.expect(Token::I32(value))
    }

    fn i64(&mut self, value: i64) -> sval::Result {
        self.expect(Token::I64(value))
    }

    fn i128(&mut self, value: i128) -> sval::Result {
        self.expect(Token::I128(value))
    }

    fn f32(&mut self, value: f32) -> sval::Result {
        self.expect(Token::F32(value))
    }

    fn f64(&mut self, value: f64) -> sval::Result {
        self.expect(Token::F64(value))
    }

    fn map_begin(&mut self, num_entries_hint: Option<usize>) -> sval::Result {
        self.expect(Token::MapBegin(num_entries_hint))
    }

    fn map_key_begin(&mut self) -> sval::Result {
        self.expect(Token::MapKeyBegin)
    }

    fn map_key_end(&mut self) -> sval::Result {
        self.expect(Token::MapKeyEnd)
    }

    fn map_value_begin(&mut self) -> sval::Result {
        self.expect(Token::MapValueBegin)
    }

    fn map_value_end(&mut self) -> sval::Result {
        self.expect(Token::MapValueEnd)
    }

    fn map_end(&mut self) -> sval::Result {
        self.expect(Token::MapEnd)
    }

    fn map_key<'k: 'data, K: sval::Source<'k>>(&mut self, key: K) -> sval::Result {
        self.expect_map_key(key)
    }

    fn map_value<'v: 'data, V: sval::Source<'v>>(&mut self, value: V) -> sval::Result {
        self.expect_map_value(value)
    }

    fn seq_begin(&mut self, num_entries_hint: Option<usize>) -> sval::Result {
        self.expect(Token::SeqBegin(num_entries_hint))
    }

    fn seq_value_begin(&mut self) -> sval::Result {
        self.expect(Token::SeqValueBegin)
    }

    fn seq_value_end(&mut self) -> sval::Result {
        self.expect(Token::SeqValueEnd)
    }

    fn seq_end(&mut self) -> sval::Result {
        self.expect(Token::SeqEnd)
    }

    fn seq_value<'e: 'data, V: sval::Source<'e>>(&mut self, value: V) -> sval::Result {
        self.expect_seq_value(value)
    }

    fn dynamic_begin(&mut self) -> sval::Result {
        self.expect(Token::DynamicBegin)
    }

    fn dynamic_end(&mut self) -> sval::Result {
        self.expect(Token::DynamicEnd)
    }

    fn constant_size_begin(&mut self) -> sval::Result {
        todo!()
    }

    fn constant_size_end(&mut self) -> sval::Result {
        todo!()
    }

    fn tagged_begin(&mut self, tag: sval::data::Tag) -> sval::Result {
        todo!()
    }

    fn tagged_end(&mut self) -> sval::Result {
        todo!()
    }

    fn constant_begin(&mut self, tag: sval::data::Tag) -> sval::Result {
        todo!()
    }

    fn constant_end(&mut self) -> sval::Result {
        todo!()
    }

    fn struct_map_begin(
        &mut self,
        tag: sval::data::Tag,
        num_entries_hint: Option<usize>,
    ) -> sval::Result {
        todo!()
    }

    fn struct_map_key_begin(&mut self, tag: sval::data::Tag) -> sval::Result {
        todo!()
    }

    fn struct_map_key_end(&mut self) -> sval::Result {
        todo!()
    }

    fn struct_map_value_begin(&mut self, tag: sval::data::Tag) -> sval::Result {
        todo!()
    }

    fn struct_map_value_end(&mut self) -> sval::Result {
        todo!()
    }

    fn struct_map_end(&mut self) -> sval::Result {
        todo!()
    }

    fn struct_map_key<'k: 'data, K: sval::Source<'k>>(
        &mut self,
        tag: sval::data::Tag,
        key: K,
    ) -> sval::Result {
        todo!()
    }

    fn struct_map_value<'v: 'data, V: sval::Source<'v>>(
        &mut self,
        tag: sval::data::Tag,
        value: V,
    ) -> sval::Result {
        todo!()
    }

    fn struct_seq_begin(
        &mut self,
        tag: sval::data::Tag,
        num_entries_hint: Option<usize>,
    ) -> sval::Result {
        todo!()
    }

    fn struct_seq_value_begin(&mut self, tag: sval::data::Tag) -> sval::Result {
        todo!()
    }

    fn struct_seq_value_end(&mut self) -> sval::Result {
        todo!()
    }

    fn struct_seq_end(&mut self) -> sval::Result {
        todo!()
    }

    fn struct_seq_value<'v: 'data, V: sval::Source<'v>>(
        &mut self,
        tag: sval::data::Tag,
        value: V,
    ) -> sval::Result {
        todo!()
    }

    fn enum_begin(&mut self, tag: sval::data::Tag) -> sval::Result {
        todo!()
    }

    fn enum_end(&mut self) -> sval::Result {
        todo!()
    }

    fn nullable_begin(&mut self) -> sval::Result {
        todo!()
    }

    fn nullable_end(&mut self) -> sval::Result {
        todo!()
    }

    fn number_begin(&mut self) -> sval::Result {
        todo!()
    }

    fn number_end(&mut self) -> sval::Result {
        todo!()
    }
}

struct Basic<R>(R);

impl<'data, R: sval::Stream<'data>> sval::Stream<'data> for Basic<R> {
    fn unit(&mut self) -> sval::Result {
        self.0.unit()
    }

    fn null(&mut self) -> sval::Result {
        self.0.null()
    }

    fn text_begin(&mut self, num_bytes_hint: Option<usize>) -> sval::Result {
        self.0.text_begin(num_bytes_hint)
    }

    fn text_fragment_computed(&mut self, fragment: &str) -> sval::Result {
        self.0.text_fragment_computed(fragment)
    }

    fn text_end(&mut self) -> sval::Result {
        self.0.text_end()
    }

    fn binary_begin(&mut self, num_bytes_hint: Option<usize>) -> sval::Result {
        self.0.binary_begin(num_bytes_hint)
    }

    fn binary_fragment_computed(&mut self, fragment: &[u8]) -> sval::Result {
        self.0.binary_fragment_computed(fragment)
    }

    fn binary_end(&mut self) -> sval::Result {
        self.0.binary_end()
    }

    fn map_begin(&mut self, num_entries_hint: Option<usize>) -> sval::Result {
        self.0.map_begin(num_entries_hint)
    }

    fn map_key_begin(&mut self) -> sval::Result {
        self.0.map_key_begin()
    }

    fn map_key_end(&mut self) -> sval::Result {
        self.0.map_key_end()
    }

    fn map_value_begin(&mut self) -> sval::Result {
        self.0.map_value_begin()
    }

    fn map_value_end(&mut self) -> sval::Result {
        self.0.map_value_end()
    }

    fn map_end(&mut self) -> sval::Result {
        self.0.map_end()
    }

    fn seq_begin(&mut self, num_entries_hint: Option<usize>) -> sval::Result {
        self.0.seq_begin(num_entries_hint)
    }

    fn seq_value_begin(&mut self) -> sval::Result {
        self.0.seq_value_begin()
    }

    fn seq_value_end(&mut self) -> sval::Result {
        self.0.seq_value_end()
    }

    fn seq_end(&mut self) -> sval::Result {
        self.0.seq_end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use num_bigint::BigInt;

    use crate::Token::*;

    #[test]
    fn stream_unit() {
        // unit
        assert().stream_equal(&(), &[Unit]);
    }

    #[test]
    fn stream_boolean() {
        assert().stream_equal(&true, &[Bool(true)]);
        assert().stream_equal(&false, &[Bool(false)]);
    }

    #[test]
    fn stream_boolean_basic() {
        // Booleans stream as unit and null in the basic model

        assert().in_basic_model().stream_equal(&true, &[Unit]);
        assert().in_basic_model().stream_equal(&false, &[Null]);
    }

    #[test]
    fn stream_text() {
        unimplemented!()
    }

    #[test]
    fn stream_binary() {
        unimplemented!()
    }

    #[test]
    fn stream_integer() {
        assert().stream_equal(&1u8, &[U8(1)]);
        assert().stream_equal(&2u16, &[U16(2)]);
        assert().stream_equal(&3u32, &[U32(3)]);
        assert().stream_equal(&4u64, &[U64(4)]);
        assert().stream_equal(&5u128, &[U128(5)]);

        assert().stream_equal(&-1i8, &[I8(-1)]);
        assert().stream_equal(&-2i16, &[I16(-2)]);
        assert().stream_equal(&-3i32, &[I32(-3)]);
        assert().stream_equal(&-4i64, &[I64(-4)]);
        assert().stream_equal(&-5i128, &[I128(-5)]);
    }

    #[test]
    fn stream_integer_basic() {
        fn assert_integer<'sval>(
            cases: &mut [(
                impl sval::Source<'sval> + Copy + Into<BigInt>,
                &'sval [&'sval str],
                &'sval [u8],
            )],
        ) {
            for (src, expected_text, expected_binary) in cases {
                let from_text: BigInt = (expected_text.join("")).parse().expect("invalid integer");
                let from_binary = BigInt::from_signed_bytes_le(expected_binary);

                assert_eq!((*src).into(), from_text);
                assert_eq!((*src).into(), from_binary);

                // Binary should be streamed in a single fragment
                assert().in_basic_model().when_binary_based().stream_equal(
                    *src,
                    &[
                        BinaryBegin(Some(expected_binary.len())),
                        BinaryFragmentComputed(Cow::Borrowed(expected_binary)),
                        BinaryEnd,
                    ],
                );

                // Text may be split into multiple fragments
                // This is determined by the standard library so it may change
                assert().in_basic_model().stream_equal(
                    *src,
                    &(Some(TextBegin(None))
                        .into_iter()
                        .chain(
                            expected_text
                                .iter()
                                .map(|fragment| TextFragmentComputed(Cow::Borrowed(fragment))),
                        )
                        .chain(Some(TextEnd))
                        .collect::<Vec<_>>()),
                );
            }
        }

        // Unsigned integers

        assert_integer(&mut [
            (u8::MIN, &["0"], &[0b00000000]),
            (u8::MAX, &["255"], &[0b11111111, 0b00000000]),
            (42u8, &["42"], &[0b00101010]),
        ]);

        assert_integer(&mut [
            (u16::MIN, &["0"], &[0b00000000]),
            (u16::MAX, &["65535"], &[0b11111111, 0b11111111, 0b00000000]),
            (42u16, &["42"], &[0b00101010]),
            (65322u16, &["65322"], &[0b00101010, 0b11111111, 0b00000000]),
        ]);

        assert_integer(&mut [
            (u32::MIN, &["0"], &[0b00000000]),
            (
                u32::MAX,
                &["4294967295"],
                &[0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b00000000],
            ),
            (42u32, &["42"], &[0b00101010]),
            (65322u32, &["65322"], &[0b00101010, 0b11111111, 0b00000000]),
            (
                4294901802u32,
                &["4294901802"],
                &[0b00101010, 0b00000000, 0b11111111, 0b11111111, 0b00000000],
            ),
        ]);

        assert_integer(&mut [
            (u64::MIN, &["0"], &[0b00000000]),
            (
                u64::MAX,
                &["18446744073709551615"],
                &[
                    0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111,
                    0b11111111, 0b11111111, 0b00000000,
                ],
            ),
            (42u64, &["42"], &[0b00101010]),
            (65322u64, &["65322"], &[0b00101010, 0b11111111, 0b00000000]),
            (
                4294901802u64,
                &["4294901802"],
                &[0b00101010, 0b00000000, 0b11111111, 0b11111111, 0b00000000],
            ),
            (
                18446744069414584362u64,
                &["18446744069414584362"],
                &[
                    0b00101010, 0b00000000, 0b00000000, 0b00000000, 0b11111111, 0b11111111,
                    0b11111111, 0b11111111, 0b00000000,
                ],
            ),
        ]);

        assert_integer(&mut [
            (u128::MIN, &["0"], &[0b00000000]),
            (
                u128::MAX,
                &["340282366920938463463374607431768211455"],
                &[
                    0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111,
                    0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111,
                    0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b00000000,
                ],
            ),
            (42u128, &["42"], &[0b00101010]),
            (65322u128, &["65322"], &[0b00101010, 0b11111111, 0b00000000]),
            (
                4294901802u128,
                &["4294901802"],
                &[0b00101010, 0b00000000, 0b11111111, 0b11111111, 0b00000000],
            ),
            (
                18446744069414584362u128,
                &["18446744069414584362"],
                &[
                    0b00101010, 0b00000000, 0b00000000, 0b00000000, 0b11111111, 0b11111111,
                    0b11111111, 0b11111111, 0b00000000,
                ],
            ),
            (
                340282366920938463444927863358058659882u128,
                &["340282366920938463444927863358058659882"],
                &[
                    0b00101010, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
                    0b00000000, 0b00000000, 0b11111111, 0b11111111, 0b11111111, 0b11111111,
                    0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b00000000,
                ],
            ),
        ]);

        // Signed integers

        assert_integer(&mut [
            (i8::MIN, &["-", "128"], &[0b10000000]),
            (42i8, &["42"], &[0b00101010]),
            (-42i8, &["-", "42"], &[0b11010110]),
        ]);

        assert_integer(&mut [
            (i16::MIN, &["-", "32768"], &[0b00000000, 0b10000000]),
            (169i16, &["169"], &[0b10101001, 0b00000000]),
            (-169i16, &["-", "169"], &[0b01010111, 0b11111111]),
        ]);

        assert_integer(&mut [
            (
                i32::MIN,
                &["-", "2147483648"],
                &[0b00000000, 0b00000000, 0b00000000, 0b10000000],
            ),
            (
                32809i32,
                &["32809"],
                &[0b00101001, 0b10000000, 0b00000000, 0b00000000],
            ),
            (
                -32809i32,
                &["-", "32809"],
                &[0b11010111, 0b01111111, 0b11111111, 0b11111111],
            ),
        ]);

        assert_integer(&mut [
            (
                i64::MIN,
                &["-", "9223372036854775808"],
                &[
                    0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
                    0b00000000, 0b10000000,
                ],
            ),
            (
                2147483689i64,
                &["2147483689"],
                &[
                    0b00101001, 0b00000000, 0b00000000, 0b10000000, 0b00000000, 0b00000000,
                    0b00000000, 0b00000000,
                ],
            ),
            (
                -2147483689i64,
                &["-", "2147483689"],
                &[
                    0b11010111, 0b11111111, 0b11111111, 0b01111111, 0b11111111, 0b11111111,
                    0b11111111, 0b11111111,
                ],
            ),
        ]);

        assert_integer(&mut [
            (
                i128::MIN,
                &["-", "170141183460469231731687303715884105728"],
                &[
                    0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
                    0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
                    0b00000000, 0b00000000, 0b00000000, 0b10000000,
                ],
            ),
            (
                9223372036854775849i128,
                &["9223372036854775849"],
                &[
                    0b00101001, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
                    0b00000000, 0b10000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
                    0b00000000, 0b00000000, 0b00000000, 0b00000000,
                ],
            ),
            (
                -9223372036854775849i128,
                &["-", "9223372036854775849"],
                &[
                    0b11010111, 0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111,
                    0b11111111, 0b01111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111,
                    0b11111111, 0b11111111, 0b11111111, 0b11111111,
                ],
            ),
        ]);
    }

    #[test]
    fn stream_float() {
        assert().stream_equal(&1f32, &[F32(1f32)]);
        assert().stream_equal(&2f64, &[F64(2f64)]);
    }

    #[test]
    fn stream_float_basic() {
        unimplemented!("floats as bytes and text")
    }

    #[test]
    fn stream_dynamic() {
        // Non-dynamic values are wrapped as dynamic
        assert().stream_equal(
            sval::data::dynamic(42i32),
            &[DynamicBegin, I32(42), DynamicEnd],
        );

        Source::permute(|src| {
            let mut expected = src.tokens().collect::<Vec<_>>();

            if expected[0] != Token::DynamicBegin {
                expected.insert(0, Token::DynamicBegin);
                expected.push(Token::DynamicEnd);
            }

            assert().stream_equal(sval::data::dynamic(src), &expected)
        });
    }
}
