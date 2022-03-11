#[macro_use]
extern crate sval_derive;

#[derive(Value)]
pub enum Data {
    StructVariant { title: &'static str, id: u64 },
}

fn main() {
    use sval::data::{Tag, TagShape::*};
    use sval_test::{assert_stream, Token::*};

    assert_stream(
        true,
        &Data::StructVariant {
            title: "Title",
            id: 42,
        },
        &[
            TaggedBegin(Tag {
                label: Some("Data"),
                id: None,
                shape: Enum,
            }),
            TaggedBegin(Tag {
                label: Some("StructVariant"),
                id: Some(0),
                shape: Struct,
            }),
            MapBegin(Some(2)),
            MapEntry(
                &[Tagged(
                    Tag {
                        label: Some("title"),
                        id: Some(0),
                        shape: StructKey,
                    },
                    &[Str("title")],
                )],
                &[Tagged(
                    Tag {
                        label: Some("title"),
                        id: Some(0),
                        shape: StructValue,
                    },
                    &[Str("Title")],
                )],
            ),
            MapEntry(
                &[Tagged(
                    Tag {
                        label: Some("id"),
                        id: Some(1),
                        shape: StructKey,
                    },
                    &[Str("id")],
                )],
                &[Tagged(
                    Tag {
                        label: Some("id"),
                        id: Some(1),
                        shape: StructValue,
                    },
                    &[U64(42)],
                )],
            ),
            MapEnd,
            TaggedEnd(Tag {
                label: Some("StructVariant"),
                id: Some(0),
                shape: Struct,
            }),
            TaggedEnd(Tag {
                label: Some("Data"),
                id: None,
                shape: Enum,
            }),
        ],
    );
}
