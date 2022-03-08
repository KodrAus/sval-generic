#[macro_use]
extern crate sval_derive;

#[derive(Value)]
pub struct Data(&'static str, u64);

fn main() {
    use sval::data::{Tag, TagShape::*};
    use sval_test::{assert_stream, Token::*};

    assert_stream(
        true,
        &Data("Title", 42),
        &[
            TaggedBegin(Tag { label: Some("Data"), id: None, shape: Tuple }),
            SeqBegin(Some(2)),
            SeqElem(&[Str("Title")]),
            SeqElem(&[U64(42)]),
            SeqEnd,
            TaggedEnd(Tag { label: Some("Data"), id: None, shape: Tuple }),
        ]);
}