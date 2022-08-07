use crate::{some, Source, Token};
use std::panic;
use std::panic::AssertUnwindSafe;

fn each() -> impl Iterator<Item = Vec<Token<'static>>> {
    vec![
        some::unit(),
        some::null(),
        some::bool(),
        some::number_u8(),
        some::number_u16(),
        some::number_u32(),
        some::number_u64(),
        some::number_u128(),
        some::number_i8(),
        some::number_i16(),
        some::number_i32(),
        some::number_i64(),
        some::number_i128(),
        some::number_32(),
        some::number_64(),
        some::text(),
        some::binary(),
    ]
    .into_iter()
}

impl<'src> Source<'src> {
    pub fn permute(mut f: impl FnMut(Source)) {
        for src in each() {
            for detect in [true, false] {
                let windows = if src.len() > 3 { 3 } else { 1 };

                for window_size in &[1, src.len() / 2, src.len()][..windows] {
                    if let Err(_) = panic::catch_unwind(AssertUnwindSafe(|| {
                        f(Source::new(detect, *window_size, &src))
                    })) {
                        panic!(
                            "failed with source `{:?}` detect dynamic `{}` and window size `{}`",
                            src, detect, window_size
                        );
                    }
                }
            }
        }
    }

    fn new(detect: bool, window_size: usize, src: &[Token<'src>]) -> Self {
        let dynamic = detect.then(|| src[0] == Token::DynamicBegin);

        let tokens = src
            .chunks(window_size)
            .map(|chunk| chunk.iter().cloned().collect())
            .collect::<Vec<Vec<Token<'src>>>>()
            .into_iter();

        Source {
            tokens,
            window_size,
            dynamic,
        }
    }
}

impl<'src> sval::Source<'src> for Source<'src> {
    fn stream_resume<'data, R: sval::Receiver<'data>>(
        &mut self,
        mut receiver: R,
    ) -> sval::Result<sval::Resume>
    where
        'src: 'data,
    {
        if let Some(tokens) = self.tokens.next() {
            for token in tokens {
                match token {
                    Token::Unit => receiver.unit(),
                    Token::Null => receiver.null(),
                    Token::Bool(v) => receiver.bool(v),
                    Token::I8(v) => receiver.i8(v),
                    Token::I16(v) => receiver.i16(v),
                    Token::I32(v) => receiver.i32(v),
                    Token::I64(v) => receiver.i64(v),
                    Token::I128(v) => receiver.i128(v),
                    Token::U8(v) => receiver.u8(v),
                    Token::U16(v) => receiver.u16(v),
                    Token::U32(v) => receiver.u32(v),
                    Token::U64(v) => receiver.u64(v),
                    Token::U128(v) => receiver.u128(v),
                    Token::F32(v) => receiver.f32(v),
                    Token::F64(v) => receiver.f64(v),
                    Token::Text(v) => receiver.text(v),
                    Token::TextBegin(v) => receiver.text_begin(v),
                    Token::TextFragment(v) => receiver.text_fragment(v),
                    Token::TextFragmentComputed(v) => receiver.text_fragment_computed(&*v),
                    Token::TextEnd => receiver.text_end(),
                    Token::Binary(v) => receiver.binary(v),
                    Token::BinaryBegin(v) => receiver.binary_begin(v),
                    Token::BinaryFragment(v) => receiver.binary_fragment(v),
                    Token::BinaryFragmentComputed(v) => receiver.binary_fragment_computed(&*v),
                    Token::BinaryEnd => receiver.binary_end(),
                    Token::MapBegin(v) => receiver.map_begin(v),
                    Token::MapKeyBegin => receiver.map_key_begin(),
                    Token::MapKeyEnd => receiver.map_key_end(),
                    Token::MapValueBegin => receiver.map_value_begin(),
                    Token::MapValueEnd => receiver.map_value_end(),
                    Token::MapEnd => receiver.map_end(),
                    Token::MapKey(v) => {
                        receiver.map_key(Source::new(self.dynamic.is_some(), self.window_size, &v))
                    }
                    Token::MapValue(v) => receiver.map_value(Source::new(
                        self.dynamic.is_some(),
                        self.window_size,
                        &v,
                    )),
                    Token::SeqBegin(v) => receiver.seq_begin(v),
                    Token::SeqValueBegin => receiver.seq_value_begin(),
                    Token::SeqValueEnd => receiver.seq_value_end(),
                    Token::SeqEnd => receiver.seq_end(),
                    Token::SeqValue(v) => receiver.seq_value(Source::new(
                        self.dynamic.is_some(),
                        self.window_size,
                        &v,
                    )),
                    Token::DynamicBegin => receiver.dynamic_begin(),
                    Token::DynamicEnd => receiver.dynamic_end(),
                }?
            }

            Ok(sval::Resume::Continue)
        } else {
            Ok(sval::Resume::Done)
        }
    }

    fn maybe_dynamic(&self) -> Option<bool> {
        self.dynamic
    }
}
