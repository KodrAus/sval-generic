use std::borrow::Cow;
use std::ops::Range;
use std::vec;

use crate::Token;
use rand::{random, thread_rng, Rng as _};

fn between(range: Range<usize>) -> usize {
    thread_rng().gen_range(range)
}

fn branch() -> bool {
    thread_rng().gen()
}

fn text_fragment() -> &'static str {
    const FRAGMENTS: &[&str] = &["", "Hello World", "🦫"];

    FRAGMENTS[between(0..FRAGMENTS.len())]
}

fn computed_text_fragment() -> String {
    thread_rng()
        .sample_iter::<char, _>(rand::distributions::Standard)
        .take(between(0..100))
        .collect::<String>()
}

pub fn text() -> Vec<Token<'static>> {
    let mut text = Vec::new();
    let mut actual_len = 0;

    text.push(Token::TextBegin(None));
    for _ in 0..between(0..10) {
        if branch() {
            let fragment = text_fragment();
            actual_len += fragment.len();

            text.push(Token::TextFragment(fragment));
        } else {
            let fragment = computed_text_fragment();
            actual_len += fragment.len();

            text.push(Token::TextFragmentComputed(Cow::Owned(fragment)));
        }
    }
    text.push(Token::TextEnd);

    if branch() {
        text[0] = Token::TextBegin(Some(actual_len));
    }

    text
}

fn binary_fragment() -> &'static [u8] {
    const FRAGMENTS: &[&[u8]] = &[&[0x00], &[0xff], b"Hello World"];

    FRAGMENTS[between(0..FRAGMENTS.len())]
}

fn computed_binary_fragment() -> Vec<u8> {
    thread_rng()
        .sample_iter::<u8, _>(rand::distributions::Standard)
        .take(between(0..100))
        .collect::<Vec<_>>()
}

pub fn binary() -> Vec<Token<'static>> {
    let mut text = Vec::new();
    let mut actual_len = 0;

    text.push(Token::BinaryBegin(None));
    for _ in 0..between(0..10) {
        if branch() {
            let fragment = binary_fragment();
            actual_len += fragment.len();

            text.push(Token::BinaryFragment(fragment));
        } else {
            let fragment = computed_binary_fragment();
            actual_len += fragment.len();

            text.push(Token::BinaryFragmentComputed(Cow::Owned(fragment)));
        }
    }
    text.push(Token::BinaryEnd);

    if branch() {
        text[0] = Token::TextBegin(Some(actual_len));
    }

    text
}

pub fn number_u8() -> Vec<Token<'static>> {
    vec![Token::U8(random())]
}

pub fn number_u16() -> Vec<Token<'static>> {
    vec![Token::U16(random())]
}

pub fn number_u32() -> Vec<Token<'static>> {
    vec![Token::U32(random())]
}

pub fn number_u64() -> Vec<Token<'static>> {
    vec![Token::U64(random())]
}

pub fn number_u128() -> Vec<Token<'static>> {
    vec![Token::U128(random())]
}

pub fn number_i8() -> Vec<Token<'static>> {
    vec![Token::I8(random())]
}

pub fn number_i16() -> Vec<Token<'static>> {
    vec![Token::I16(random())]
}

pub fn number_i32() -> Vec<Token<'static>> {
    vec![Token::I32(random())]
}

pub fn number_i64() -> Vec<Token<'static>> {
    vec![Token::I64(random())]
}

pub fn number_i128() -> Vec<Token<'static>> {
    vec![Token::I128(random())]
}

pub fn number_32() -> Vec<Token<'static>> {
    vec![Token::F32(random())]
}

pub fn number_64() -> Vec<Token<'static>> {
    vec![Token::U64(random())]
}

pub fn bool() -> Vec<Token<'static>> {
    vec![Token::Bool(random())]
}

pub fn unit() -> Vec<Token<'static>> {
    vec![Token::Unit]
}

pub fn null() -> Vec<Token<'static>> {
    vec![Token::Null]
}

pub fn source() -> Vec<Token<'static>> {
    Vec::new()
}
