#[macro_use]
extern crate sval_derive;

#[derive(Value)]
pub enum Data {
    EmptyVariant,
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
}
