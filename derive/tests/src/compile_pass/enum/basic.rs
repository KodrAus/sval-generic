#[macro_use]
extern crate sval_derive;

#[derive(Value)]
pub enum Data {
    EmptyVariant,
    NewtypeVariant(u64),
    TupleVariant(&'static str, u64),
    StructVariant { title: &'static str, id: u64 },
}

fn main() {
    use sval::data::{Tag, TagShape::*};
    use sval_test::{assert_stream, Token::*};

    assert_stream(
        true,
        &Data::EmptyVariant,
        &[
            TaggedBegin(Tag { label: Some("Data"), id: None, shape: Enum }),
            Tagged(
                Tag { label: Some("EmptyVariant"), id: Some(0), shape: EnumConstant },
                &[
                    Str("EmptyVariant"),
                ],
            ),
            TaggedEnd(Tag { label: Some("Data"), id: None, shape: Enum }),
        ]);

    assert_stream(
        true,
        &Data::NewtypeVariant(42),
        &[
            TaggedBegin(Tag { label: Some("Data"), id: None, shape: Enum }),
            Tagged(
                Tag { label: Some("NewtypeVariant"), id: Some(1), shape: Unspecified },
                &[
                    U64(42),
                ],
            ),
            TaggedEnd(Tag { label: Some("Data"), id: None, shape: Enum }),
        ]);

    assert_stream(
        true,
        &Data::TupleVariant("Title", 42),
        &[
            TaggedBegin(Tag { label: Some("Data"), id: None, shape: Enum }),
            TaggedBegin(Tag { label: Some("TupleVariant"), id: Some(2), shape: Tuple }),
            SeqBegin(Some(2)),
            SeqElem(&[Str("Title")]),
            SeqElem(&[U64(42)]),
            SeqEnd,
            TaggedEnd(Tag { label: Some("TupleVariant"), id: Some(2), shape: Tuple }),
            TaggedEnd(Tag { label: Some("Data"), id: None, shape: Enum }),
        ]);

    assert_stream(
        true,
        &Data::StructVariant { title: "Title", id: 42 },
        &[
            TaggedBegin(Tag { label: Some("Data"), id: None, shape: Enum }),
            TaggedBegin(Tag { label: Some("StructVariant"), id: Some(3), shape: Struct }),
            MapBegin(Some(2)),
            MapEntry(
                &[Tagged(Tag { label: Some("title"), id: Some(0), shape: StructField }, &[Str("title")])],
                &[Str("Title")],
            ),
            MapEntry(
                &[Tagged(Tag { label: Some("id"), id: Some(1), shape: StructField }, &[Str("id")])],
                &[U64(42)],
            ),
            MapEnd,
            TaggedEnd(Tag { label: Some("StructVariant"), id: Some(3), shape: Struct }),
            TaggedEnd(Tag { label: Some("Data"), id: None, shape: Enum }),
        ]);
}
