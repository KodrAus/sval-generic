#[macro_use]
extern crate sval_derive;

#[derive(Value)]
pub struct Data {
    title: String,
    id: u64,
    tags: Vec<DataTag>,
}

#[derive(Value)]
pub struct DataTag {
    id: u64,
    label: String,
}

fn main() {}
