#![no_std]

#[derive(Debug, Clone, Copy)]
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
    Char(char),
    Str(&'a str),
    TextBegin(Option<usize>),
    TextFragment(&'a str),
    TextFragmentComputed(&'a str),
    TextEnd,
    Bytes(&'a [u8]),
    BinaryBegin(Option<usize>),
    BinaryFragment(&'a [u8]),
    BinaryFragmentComputed(&'a [u8]),
    BinaryEnd,
    Tagged(sval::data::Tag, &'a [Token<'a>]),
    TaggedBegin(sval::data::Tag),
    TaggedEnd(sval::data::Tag),
    MapBegin(Option<usize>),
    MapKeyBegin,
    MapKeyEnd,
    MapValueBegin,
    MapValueEnd,
    MapEnd,
    MapEntry(&'a [Token<'a>], &'a [Token<'a>]),
    MapKey(&'a [Token<'a>]),
    MapValue(&'a [Token<'a>]),
    SeqBegin(Option<usize>),
    SeqElemBegin,
    SeqElemEnd,
    SeqEnd,
    SeqElem(&'a [Token<'a>]),
}

impl<'a, 'b> PartialEq<Token<'b>> for Token<'a> {
    fn eq(&self, other: &Token<'b>) -> bool {
        match (self, other) {
            (Token::Unit, Token::Unit) => true,
            (Token::Null, Token::Null) => true,
            (Token::Bool(a), Token::Bool(b)) => a == b,
            (Token::U8(a), Token::U8(b)) => a == b,
            (Token::U16(a), Token::U16(b)) => a == b,
            (Token::U32(a), Token::U32(b)) => a == b,
            (Token::U64(a), Token::U64(b)) => a == b,
            (Token::U128(a), Token::U128(b)) => a == b,
            (Token::I8(a), Token::I8(b)) => a == b,
            (Token::I16(a), Token::I16(b)) => a == b,
            (Token::I32(a), Token::I32(b)) => a == b,
            (Token::I64(a), Token::I64(b)) => a == b,
            (Token::I128(a), Token::I128(b)) => a == b,
            (Token::F32(a), Token::F32(b)) => a == b,
            (Token::F64(a), Token::F64(b)) => a == b,
            (Token::Char(a), Token::Char(b)) => a == b,
            (Token::Str(a), Token::Str(b)) => a == b,
            (Token::TextBegin(a), Token::TextBegin(b)) => a == b,
            (Token::TextFragment(a), Token::TextFragment(b)) => a == b,
            (Token::TextFragmentComputed(a), Token::TextFragmentComputed(b)) => a == b,
            (Token::TextEnd, Token::TextEnd) => true,
            (Token::Bytes(a), Token::Bytes(b)) => a == b,
            (Token::BinaryBegin(a), Token::BinaryBegin(b)) => a == b,
            (Token::BinaryFragment(a), Token::BinaryFragment(b)) => a == b,
            (Token::BinaryFragmentComputed(a), Token::BinaryFragmentComputed(b)) => a == b,
            (Token::BinaryEnd, Token::BinaryEnd) => true,
            (Token::TaggedBegin(a), Token::TaggedBegin(b)) => a == b,
            (Token::TaggedEnd(a), Token::TaggedEnd(b)) => a == b,
            (Token::Tagged(at, av), Token::Tagged(bt, bv)) => at == bt && av == bv,
            (Token::MapBegin(a), Token::MapBegin(b)) => a == b,
            (Token::MapKeyBegin, Token::MapKeyBegin) => true,
            (Token::MapKeyEnd, Token::MapKeyEnd) => true,
            (Token::MapValueBegin, Token::MapValueBegin) => true,
            (Token::MapValueEnd, Token::MapValueEnd) => true,
            (Token::MapEnd, Token::MapEnd) => true,
            (Token::MapEntry(ak, av), Token::MapEntry(bk, bv)) => ak == bk && av == bv,
            (Token::MapKey(a), Token::MapKey(b)) => a == b,
            (Token::MapValue(a), Token::MapValue(b)) => a == b,
            (Token::SeqBegin(a), Token::SeqBegin(b)) => a == b,
            (Token::SeqElemBegin, Token::SeqElemBegin) => true,
            (Token::SeqElemEnd, Token::SeqElemEnd) => true,
            (Token::SeqEnd, Token::SeqEnd) => true,
            (Token::SeqElem(a), Token::SeqElem(b)) => a == b,
            _ => false,
        }
    }
}

pub fn assert_stream<'a>(text_based: bool, source: impl sval::Source<'a>, expected: &[Token<'a>]) {
    struct Expect<'a, 'b> {
        text_based: bool,
        tokens: &'b [Token<'a>],
    }

    impl<'a, 'b> Expect<'a, 'b> {
        fn stream_to_end<'c>(
            text_based: bool,
            tokens: &'b [Token<'a>],
            mut source: impl sval::Source<'c>,
        ) -> sval::Result {
            let mut expect = Expect { text_based, tokens };

            source.stream_to_end(&mut expect)?;

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

        fn expect_tagged<'c>(
            &mut self,
            tag: sval::data::Tag,
            source: impl sval::Source<'c>,
        ) -> sval::Result {
            match self.tokens.get(0) {
                Some(Token::Tagged(expected_tag, expected)) => {
                    assert_eq!(*expected_tag, tag);

                    Expect::stream_to_end(self.text_based, expected, source)?;
                    self.advance();

                    Ok(())
                }
                Some(token) => panic!("unexpected `{:?}`; expected `Tagged`", token),
                None => panic!("unexpected end of stream; expected `Tagged`"),
            }
        }

        fn expect_map_key_value<'c, 'd>(
            &mut self,
            key: impl sval::Source<'c>,
            value: impl sval::Source<'d>,
        ) -> sval::Result {
            match self.tokens.get(0) {
                Some(Token::MapEntry(key_tokens, value_tokens)) => {
                    Expect::stream_to_end(self.text_based, key_tokens, key)?;
                    Expect::stream_to_end(self.text_based, value_tokens, value)?;
                    self.advance();

                    Ok(())
                }
                Some(token) => panic!("unexpected `{:?}`; expected `MapEntry`", token),
                None => panic!("unexpected end of stream; expected `MapEntry`"),
            }
        }

        fn expect_map_key<'c>(&mut self, key: impl sval::Source<'c>) -> sval::Result {
            match self.tokens.get(0) {
                Some(Token::MapKey(expected)) => {
                    Expect::stream_to_end(self.text_based, expected, key)?;
                    self.advance();

                    Ok(())
                }
                Some(token) => panic!("unexpected `{:?}`; expected `MapKey`", token),
                None => panic!("unexpected end of stream; expected `MapKey`"),
            }
        }

        fn expect_map_value<'c>(&mut self, value: impl sval::Source<'c>) -> sval::Result {
            match self.tokens.get(0) {
                Some(Token::MapValue(expected)) => {
                    Expect::stream_to_end(self.text_based, expected, value)?;
                    self.advance();

                    Ok(())
                }
                Some(token) => panic!("unexpected `{:?}`; expected `MapValue`", token),
                None => panic!("unexpected end of stream; expected `MapValue`"),
            }
        }

        fn expect_seq_value<'c>(&mut self, elem: impl sval::Source<'c>) -> sval::Result {
            match self.tokens.get(0) {
                Some(Token::SeqElem(expected)) => {
                    Expect::stream_to_end(self.text_based, expected, elem)?;
                    self.advance();

                    Ok(())
                }
                Some(token) => panic!("unexpected `{:?}`; expected `SeqElem`", token),
                None => panic!("unexpected end of stream; expected `SeqElem`"),
            }
        }
    }

    impl<'a, 'b> sval::Receiver<'a> for Expect<'a, 'b> {
        fn is_text_based(&self) -> bool {
            self.text_based
        }

        fn unit(&mut self) -> sval::Result {
            self.expect(Token::Unit)
        }

        fn null(&mut self) -> sval::Result {
            self.expect(Token::Null)
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

        fn bool(&mut self, value: bool) -> sval::Result {
            self.expect(Token::Bool(value))
        }

        fn char(&mut self, value: char) -> sval::Result {
            self.expect(Token::Char(value))
        }

        fn text(&mut self, value: &'a str) -> sval::Result {
            self.expect(Token::Str(value))
        }

        fn text_begin(&mut self, num_bytes_hint: Option<usize>) -> sval::Result {
            self.expect(Token::TextBegin(num_bytes_hint))
        }

        fn text_fragment(&mut self, fragment: &'a str) -> sval::Result {
            self.expect(Token::TextFragment(fragment))
        }

        fn text_fragment_computed(&mut self, fragment: &str) -> sval::Result {
            self.expect(Token::TextFragmentComputed(fragment))
        }

        fn text_end(&mut self) -> sval::Result {
            self.expect(Token::TextEnd)
        }

        fn binary(&mut self, value: &'a [u8]) -> sval::Result {
            self.expect(Token::Bytes(value))
        }

        fn binary_begin(&mut self, num_bytes_hint: Option<usize>) -> sval::Result {
            self.expect(Token::BinaryBegin(num_bytes_hint))
        }

        fn binary_fragment(&mut self, fragment: &'a [u8]) -> sval::Result {
            self.expect(Token::BinaryFragment(fragment))
        }

        fn binary_fragment_computed(&mut self, fragment: &[u8]) -> sval::Result {
            self.expect(Token::BinaryFragmentComputed(fragment))
        }

        fn binary_end(&mut self) -> sval::Result {
            self.expect(Token::BinaryEnd)
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

        fn map_key_value<'k: 'a, 'v: 'a, K: sval::Source<'k>, V: sval::Source<'v>>(
            &mut self,
            key: K,
            value: V,
        ) -> sval::Result {
            self.expect_map_key_value(key, value)
        }

        fn map_key<'k: 'a, K: sval::Source<'k>>(&mut self, key: K) -> sval::Result {
            self.expect_map_key(key)
        }

        fn map_value<'v: 'a, V: sval::Source<'v>>(&mut self, value: V) -> sval::Result {
            self.expect_map_value(value)
        }

        fn seq_begin(&mut self, num_elems_hint: Option<usize>) -> sval::Result {
            self.expect(Token::SeqBegin(num_elems_hint))
        }

        fn seq_value_begin(&mut self) -> sval::Result {
            self.expect(Token::SeqElemBegin)
        }

        fn seq_value_end(&mut self) -> sval::Result {
            self.expect(Token::SeqElemEnd)
        }

        fn seq_end(&mut self) -> sval::Result {
            self.expect(Token::SeqEnd)
        }

        fn seq_value<'e: 'a, V: sval::Source<'e>>(&mut self, value: V) -> sval::Result {
            self.expect_seq_value(value)
        }
    }

    Expect::stream_to_end(text_based, expected, source).expect("invalid source");
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::Token::*;

    #[test]
    fn stream_primitive() {
        // unit
        assert_stream(true, &(), &[Unit]);

        // nullable
        assert_stream(
            true,
            &Option::<()>::None,
            &[Tagged(
                sval::data::tag()
                    .for_nullable()
                    .with_label("None")
                    .with_id(0),
                &[Null],
            )],
        );
        assert_stream(
            true,
            &Some(()),
            &[Tagged(
                sval::data::tag()
                    .for_nullable()
                    .with_label("Some")
                    .with_id(1),
                &[Unit],
            )],
        );

        assert_stream(true, &true, &[Bool(true)]);

        assert_stream(true, 'a', &[Char('a')]);
        assert_stream(true, "string value", &[Str("string value")]);

        // unsigned integers
        assert_stream(true, &1u8, &[U8(1)]);
        assert_stream(true, &2u16, &[U16(2)]);
        assert_stream(true, &3u32, &[U32(3)]);
        assert_stream(true, &4u64, &[U64(4)]);
        assert_stream(true, &5u128, &[U128(5)]);

        // signed integers
        assert_stream(true, &-1i8, &[I8(-1)]);
        assert_stream(true, &-2i16, &[I16(-2)]);
        assert_stream(true, &-3i32, &[I32(-3)]);
        assert_stream(true, &-4i64, &[I64(-4)]);
        assert_stream(true, &-5i128, &[I128(-5)]);

        // floats
        assert_stream(true, &1f32, &[F32(1f32)]);
        assert_stream(true, &2f64, &[F64(2f64)]);
    }

    #[test]
    fn stream_slice() {
        assert_stream(
            true,
            &[1, 2, 3, 4, 5] as &[i32],
            &[
                TaggedBegin(sval::data::tag().for_slice()),
                SeqBegin(Some(5)),
                SeqElem(&[I32(1)]),
                SeqElem(&[I32(2)]),
                SeqElem(&[I32(3)]),
                SeqElem(&[I32(4)]),
                SeqElem(&[I32(5)]),
                SeqEnd,
                TaggedEnd(sval::data::tag().for_slice()),
            ],
        );

        assert_stream(
            true,
            &[1, 2, 3, 4, 5],
            &[
                TaggedBegin(sval::data::tag().for_array()),
                SeqBegin(Some(5)),
                SeqElem(&[I32(1)]),
                SeqElem(&[I32(2)]),
                SeqElem(&[I32(3)]),
                SeqElem(&[I32(4)]),
                SeqElem(&[I32(5)]),
                SeqEnd,
                TaggedEnd(sval::data::tag().for_array()),
            ],
        );
    }

    #[test]
    fn stream_tuple() {
        assert_stream(
            true,
            &("Title", 42u64),
            &[
                TaggedBegin(sval::data::tag().for_struct()),
                SeqBegin(Some(2)),
                SeqElem(&[Tagged(
                    sval::data::tag().for_struct_value().with_id(0),
                    &[Str("Title")],
                )]),
                SeqElem(&[Tagged(
                    sval::data::tag().for_struct_value().with_id(1),
                    &[U64(42)],
                )]),
                SeqEnd,
                TaggedEnd(sval::data::tag().for_struct()),
            ],
        );
    }
}
