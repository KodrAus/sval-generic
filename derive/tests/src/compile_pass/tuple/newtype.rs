#[macro_use]
extern crate sval_derive;

#[derive(Value)]
pub struct Data(u64);

fn main() {
    use sval::data::{Tag, TagShape::*};
    use sval_test::{assert_stream, Token::*};

    assert_stream(
        true,
        &Data(42),
        &[Tagged(
            Tag {
                label: Some("Data"),
                id: None,
                shape: Contents,
            },
            &[U64(42)],
        )],
    );
}
