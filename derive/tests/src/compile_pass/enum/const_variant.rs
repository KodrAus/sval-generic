#[macro_use]
extern crate sval_derive;

#[derive(Value)]
#[repr(u8)]
pub enum Data {
    A = 5,
    B = 17,
    C = 39,
}

fn main() {}
