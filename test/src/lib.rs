use sval::data::Tag;
use sval::Source;

#[derive(Debug, Clone, Copy, PartialEq)]
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
    TextFragmentComputed(&'a str),
    TextEnd,
    Binary(&'a [u8]),
    BinaryBegin(Option<usize>),
    BinaryFragment(&'a [u8]),
    BinaryFragmentComputed(&'a [u8]),
    BinaryEnd,
    MapBegin(Option<usize>),
    MapKeyBegin,
    MapKeyEnd,
    MapValueBegin,
    MapValueEnd,
    MapEnd,
    MapKey(&'a [Token<'a>]),
    MapValue(&'a [Token<'a>]),
    SeqBegin(Option<usize>),
    SeqValueBegin,
    SeqValueEnd,
    SeqEnd,
    SeqValue(&'a [Token<'a>]),
}

pub fn assert() -> Assert {
    Default::default()
}

#[non_exhaustive]
pub struct Assert {
    pub text_based: bool,
    pub basic_model: bool,
}

impl Default for Assert {
    fn default() -> Self {
        Assert {
            text_based: true,
            basic_model: false,
        }
    }
}

impl Assert {
    pub fn text_based(mut self, text_based: bool) -> Self {
        self.text_based = text_based;
        self
    }

    pub fn basic_model(mut self, basic_model: bool) -> Self {
        self.basic_model = basic_model;
        self
    }

    pub fn equal<'src>(&self, source: impl sval::Source<'src>, expected: &[Token<'src>]) {
        Expect::stream_to_end(source, self, expected).expect("invalid source")
    }
}

struct Expect<'data, 'brw> {
    assert: &'brw Assert,
    tokens: &'brw [Token<'data>],
}

impl<'data, 'brw> Expect<'data, 'brw> {
    fn stream_to_end<'src>(
        mut source: impl sval::Source<'src>,
        assert: &Assert,
        tokens: &[Token<'data>],
    ) -> sval::Result
    where
        'src: 'data,
    {
        let mut expect = Expect { assert, tokens };

        if expect.assert.basic_model {
            source.stream_to_end(Basic(&mut expect))?;
        } else {
            source.stream_to_end(&mut expect)?;
        }

        assert_eq!(0, expect.tokens.len());

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
                assert_eq!(token, *expected);
                self.advance();

                Ok(())
            }
            None => panic!("unexpected end of stream"),
        }
    }

    fn expect_map_key<'src>(&mut self, key: impl sval::Source<'src>) -> sval::Result {
        match self.tokens.get(0) {
            Some(Token::MapKey(expected)) => {
                Expect::stream_to_end(key, self.assert, expected)?;
                self.advance();

                Ok(())
            }
            Some(token) => panic!("unexpected `{:?}`; expected `MapKey`", token),
            None => panic!("unexpected end of stream; expected `MapKey`"),
        }
    }

    fn expect_map_value<'src>(&mut self, value: impl sval::Source<'src>) -> sval::Result {
        match self.tokens.get(0) {
            Some(Token::MapValue(expected)) => {
                Expect::stream_to_end(value, self.assert, expected)?;
                self.advance();

                Ok(())
            }
            Some(token) => panic!("unexpected `{:?}`; expected `MapValue`", token),
            None => panic!("unexpected end of stream; expected `MapValue`"),
        }
    }

    fn expect_seq_value<'src>(&mut self, value: impl sval::Source<'src>) -> sval::Result {
        match self.tokens.get(0) {
            Some(Token::SeqValue(expected)) => {
                Expect::stream_to_end(value, self.assert, expected)?;
                self.advance();

                Ok(())
            }
            Some(token) => panic!("unexpected `{:?}`; expected `SeqValue`", token),
            None => panic!("unexpected end of stream; expected `SeqValue`"),
        }
    }
}

impl<'data, 'b> sval::Receiver<'data> for Expect<'data, 'b> {
    fn is_text_based(&self) -> bool {
        self.assert.text_based
    }

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
        self.expect(Token::TextFragmentComputed(fragment))
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
        self.expect(Token::BinaryFragmentComputed(fragment))
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

    fn map_key<'k: 'data, K: Source<'k>>(&mut self, key: K) -> sval::Result {
        self.expect_map_key(key)
    }

    fn map_value<'v: 'data, V: Source<'v>>(&mut self, value: V) -> sval::Result {
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

    fn seq_value<'e: 'data, V: Source<'e>>(&mut self, value: V) -> sval::Result {
        self.expect_seq_value(value)
    }

    fn dynamic_begin(&mut self) -> sval::Result {
        todo!()
    }

    fn dynamic_end(&mut self) -> sval::Result {
        todo!()
    }

    fn fixed_size_begin(&mut self) -> sval::Result {
        todo!()
    }

    fn fixed_size_end(&mut self) -> sval::Result {
        todo!()
    }

    fn tagged_begin(&mut self, tag: Tag) -> sval::Result {
        todo!()
    }

    fn tagged_end(&mut self) -> sval::Result {
        todo!()
    }

    fn constant_begin(&mut self, tag: Tag) -> sval::Result {
        todo!()
    }

    fn constant_end(&mut self) -> sval::Result {
        todo!()
    }

    fn struct_map_begin(&mut self, tag: Tag, num_entries_hint: Option<usize>) -> sval::Result {
        todo!()
    }

    fn struct_map_key_begin(&mut self, tag: Tag) -> sval::Result {
        todo!()
    }

    fn struct_map_key_end(&mut self) -> sval::Result {
        todo!()
    }

    fn struct_map_value_begin(&mut self, tag: Tag) -> sval::Result {
        todo!()
    }

    fn struct_map_value_end(&mut self) -> sval::Result {
        todo!()
    }

    fn struct_map_end(&mut self) -> sval::Result {
        todo!()
    }

    fn struct_map_key<'k: 'data, K: Source<'k>>(&mut self, tag: Tag, key: K) -> sval::Result {
        todo!()
    }

    fn struct_map_value<'v: 'data, V: Source<'v>>(&mut self, tag: Tag, value: V) -> sval::Result {
        todo!()
    }

    fn struct_seq_begin(&mut self, tag: Tag, num_entries_hint: Option<usize>) -> sval::Result {
        todo!()
    }

    fn struct_seq_value_begin(&mut self, tag: Tag) -> sval::Result {
        todo!()
    }

    fn struct_seq_value_end(&mut self) -> sval::Result {
        todo!()
    }

    fn struct_seq_end(&mut self) -> sval::Result {
        todo!()
    }

    fn struct_seq_value<'v: 'data, V: Source<'v>>(&mut self, tag: Tag, value: V) -> sval::Result {
        todo!()
    }

    fn enum_begin(&mut self, tag: Tag) -> sval::Result {
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

    fn int_begin(&mut self) -> sval::Result {
        todo!()
    }

    fn int_end(&mut self) -> sval::Result {
        todo!()
    }

    fn binfloat_begin(&mut self) -> sval::Result {
        todo!()
    }

    fn binfloat_end(&mut self) -> sval::Result {
        todo!()
    }

    fn decfloat_begin(&mut self) -> sval::Result {
        todo!()
    }

    fn decfloat_end(&mut self) -> sval::Result {
        todo!()
    }
}

struct Basic<R>(R);

impl<'data, R: sval::Receiver<'data>> sval::Receiver<'data> for Basic<R> {
    fn is_text_based(&self) -> bool {
        self.0.is_text_based()
    }

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

    use crate::Token::*;

    #[test]
    fn stream_unit() {
        // unit
        assert().equal(&(), &[Unit]);
    }

    #[test]
    fn stream_boolean() {
        assert().equal(&true, &[Bool(true)]);
        assert().equal(&false, &[Bool(false)]);
    }

    #[test]
    fn stream_boolean_basic() {
        unimplemented!("booleans as unit and null")
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
        assert().equal(&1u8, &[U8(1)]);
        assert().equal(&2u16, &[U16(2)]);
        assert().equal(&3u32, &[U32(3)]);
        assert().equal(&4u64, &[U64(4)]);
        assert().equal(&5u128, &[U128(5)]);

        assert().equal(&-1i8, &[I8(-1)]);
        assert().equal(&-2i16, &[I16(-2)]);
        assert().equal(&-3i32, &[I32(-3)]);
        assert().equal(&-4i64, &[I64(-4)]);
        assert().equal(&-5i128, &[I128(-5)]);
    }

    #[test]
    fn stream_integer_basic() {
        unimplemented!("integers as bytes and text")
    }

    #[test]
    fn stream_float() {
        assert().equal(&1f32, &[F32(1f32)]);
        assert().equal(&2f64, &[F64(2f64)]);
    }

    #[test]
    fn stream_float_basic() {
        unimplemented!("floats as bytes and text")
    }
}
