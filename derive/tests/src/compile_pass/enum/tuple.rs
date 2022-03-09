#[macro_use]
extern crate sval_derive;

#[derive(Value)]
pub enum Data {
    TupleVariant(&'static str, u64),
}

fn main() {
    use sval::data::{Tag, TagShape::*};
    use sval_test::{assert_stream, Token::*};

    assert_stream(
        true,
        &Data::TupleVariant("Title", 42),
        &[
            TaggedBegin(Tag { label: Some("Data"), id: None, shape: Enum }),
            TaggedBegin(Tag { label: Some("TupleVariant"), id: Some(0), shape: Tuple }),
            SeqBegin(Some(2)),
            SeqElem(&[
                Tagged(
                    Tag { label: None, id: Some(0), shape: Unspecified },
                    &[Str("Title")]
                ),
            ]),
            SeqElem(&[
                Tagged(
                    Tag { label: None, id: Some(1), shape: Unspecified },
                    &[U64(42)]
                ),
            ]),
            SeqEnd,
            TaggedEnd(Tag { label: Some("TupleVariant"), id: Some(0), shape: Tuple }),
            TaggedEnd(Tag { label: Some("Data"), id: None, shape: Enum }),
        ]);
}
