#![cfg(test)]

#[macro_use]
extern crate sval_derive;

use std::fmt;

fn assert_debug(v: impl sval::Value + fmt::Debug) {
    let expected = format!("{:?}", v);
    let actual = format!("{:?}", sval_fmt::debug(v));

    assert_eq!(expected, actual);
}

#[derive(Value, Debug)]
struct MyData {
    field_0: i32,
    field_1: bool,
    field_2: &'static str,
}

#[test]
fn debug_primitive() {
    assert_debug(42i32);
}

#[test]
fn debug_struct() {
    assert_debug(MyData {
        field_0: 42,
        field_1: true,
        field_2: "Hello",
    });
}
