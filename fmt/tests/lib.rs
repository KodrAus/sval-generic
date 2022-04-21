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
struct MapStruct {
    field_0: i32,
    field_1: bool,
    field_2: &'static str,
}

#[derive(Value, Debug)]
struct SeqStruct(i32, bool, &'static str);

#[derive(Value, Debug)]
struct Tagged(i32);

#[derive(Value, Debug)]
enum Enum {
    Constant,
    Tagged(i32),
    MapStruct {
        field_0: i32,
        field_1: bool,
        field_2: &'static str,
    },
    SeqStruct(i32, bool, &'static str),
}

#[test]
fn debug_primitive() {
    assert_debug(42i32);
}

#[test]
fn debug_option() {
    assert_debug(Some(42i32));
    assert_debug(None::<i32>);
}

#[test]
fn debug_map_struct() {
    assert_debug(MapStruct {
        field_0: 42,
        field_1: true,
        field_2: "Hello",
    });
}

#[test]
fn debug_seq_struct() {
    assert_debug(SeqStruct(42, true, "Hello"));
    assert_debug((42, true, "Hello"));
}

#[test]
fn debug_tagged() {
    assert_debug(Tagged(42));
}

#[test]
fn debug_enum() {
    assert_debug(Enum::Constant);

    assert_debug(Enum::MapStruct {
        field_0: 42,
        field_1: true,
        field_2: "Hello",
    });

    assert_debug(Enum::SeqStruct(42, true, "Hello"));

    assert_debug(Enum::Tagged(42));
}
