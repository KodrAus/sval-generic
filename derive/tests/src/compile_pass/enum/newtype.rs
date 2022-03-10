#[macro_use]
extern crate sval_derive;

#[derive(Value)]
pub enum Data {
    NewtypeVariant(u64),
}

fn main() {
    use sval::data::{Tag, TagShape::*};
    use sval_test::{assert_stream, Token::*};

    assert_stream(
        true,
        &Data::NewtypeVariant(42),
        &[
            TaggedBegin(Tag {
                label: Some("Data"),
                id: None,
                shape: Enum,
            }),
            Tagged(
                Tag {
                    label: Some("NewtypeVariant"),
                    id: Some(0),
                    shape: Contents,
                },
                &[U64(42)],
            ),
            TaggedEnd(Tag {
                label: Some("Data"),
                id: None,
                shape: Enum,
            }),
        ],
    );
}
