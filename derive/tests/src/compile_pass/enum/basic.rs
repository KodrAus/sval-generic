#[macro_use]
extern crate sval_derive;

#[derive(Value)]
pub enum Data {
    EmptyVariant,
    NewtypeVariant(u64),
    TupleVariant(String, u64),
    StructVariant { title: String, id: u64 },
}

fn main() {}
