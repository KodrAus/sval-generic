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
            TaggedBegin(Tag {
                label: Some("Data"),
                id: None,
                shape: Struct,
            }),
            SeqBegin(Some(2)),
            SeqElem(&[Tagged(
                Tag {
                    label: None,
                    id: Some(0),
                    shape: StructValue,
                },
                &[Str("Title")],
            )]),
            SeqElem(&[Tagged(
                Tag {
                    label: None,
                    id: Some(1),
                    shape: StructValue,
                },
                &[U64(42)],
            )]),
            SeqEnd,
            TaggedEnd(Tag {
                label: Some("Data"),
                id: None,
                shape: Struct,
            }),
        ],
    );
}
