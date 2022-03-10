#[macro_use]
extern crate sval_derive;

#[derive(Value)]
pub struct Data {
    title: &'static str,
    id: u64,
}

fn main() {
    use sval::data::{Tag, TagShape::*};
    use sval_test::{assert_stream, Token::*};

    assert_stream(
        true,
        &Data { title: "Title", id: 42 },
        &[
            TaggedBegin(Tag { label: Some("Data"), id: None, shape: Struct }),
            MapBegin(Some(2)),
            MapEntry(
                &[Str("title")],
                &[Tagged(Tag { label: Some("title"), id: Some(0), shape: Field }, &[Str("Title")])],
            ),
            MapEntry(
                &[Str("id")],
                &[Tagged(Tag { label: Some("id"), id: Some(1), shape: Field }, &[U64(42)])],
            ),
            MapEnd,
            TaggedEnd(Tag { label: Some("Data"), id: None, shape: Struct }),
        ]);
}
