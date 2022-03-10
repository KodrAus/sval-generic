#[macro_use]
extern crate sval_derive;

#[derive(Value)]
#[repr(u8)]
pub enum Data {
    A = 5,
    B = 17,
    C = 39,
}

fn main() {
    use sval::data::{Tag, TagShape::*};
    use sval_test::{assert_stream, Token::*};

    assert_stream(
        true,
        &Data::A,
        &[
            TaggedBegin(Tag { label: Some("Data"), id: None, shape: Enum }),
            Tagged(
                Tag { label: Some("A"), id: Some(0), shape: Constant },
                &[
                    Str("A"),
                ],
            ),
            TaggedEnd(Tag { label: Some("Data"), id: None, shape: Enum }),
        ]);
}
