#![cfg(test)]

#[macro_use]
extern crate sval_derive;

#[macro_use]
extern crate serde_derive;

use serde_test::assert_ser_tokens;

use std::collections::BTreeMap;

type Map = BTreeMap<&'static str, i32>;

type Seq = Vec<i32>;

#[derive(Value, Serialize)]
struct MapStruct {
    field_0: i32,
    field_1: bool,
    field_2: &'static str,
}

#[derive(Value, Serialize)]
struct SeqStruct(i32, bool, &'static str);

#[derive(Value, Serialize)]
struct Tagged(i32);

#[derive(Value, Serialize)]
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

fn serialize_case(v: (impl sval::Value + serde::Serialize), tokens: &[serde_test::Token]) {
    assert_ser_tokens(&sval_serde::to_serialize(&v), tokens);
    assert_ser_tokens(&v, tokens);
}

#[test]
fn map_to_serialize() {
    serialize_case(
        {
            let mut map = Map::new();

            map.insert("a", 1);
            map.insert("b", 2);

            map
        },
        {
            use serde_test::Token::*;

            &[
                Map {
                    len: Option::Some(2),
                },
                Str("a"),
                I32(1),
                Str("b"),
                I32(2),
                MapEnd,
            ]
        },
    );
}

#[test]
fn seq_to_serialize() {
    serialize_case(
        {
            let mut seq = Seq::new();

            seq.push(1);
            seq.push(2);

            seq
        },
        {
            use serde_test::Token::*;

            &[
                Seq {
                    len: Option::Some(2),
                },
                I32(1),
                I32(2),
                SeqEnd,
            ]
        },
    );
}

#[test]
fn map_struct_to_serialize() {
    serialize_case(
        MapStruct {
            field_0: 1,
            field_1: true,
            field_2: "a",
        },
        {
            use serde_test::Token::*;

            &[
                Struct {
                    name: "MapStruct",
                    len: 3,
                },
                Str("field_0"),
                I32(1),
                Str("field_1"),
                Bool(true),
                Str("field_2"),
                Str("a"),
                StructEnd,
            ]
        },
    );
}

#[test]
fn seq_struct_named_to_serialize() {
    serialize_case((1, true, "a"), {
        use serde_test::Token::*;

        &[
            TupleStruct {
                name: "SeqStruct",
                len: 3,
            },
            I32(1),
            Bool(true),
            Str("a"),
            TupleStructEnd,
        ]
    });
}

#[test]
fn seq_struct_unnamed_to_serialize() {
    serialize_case((1, true, "a"), {
        use serde_test::Token::*;

        &[Tuple { len: 3 }, I32(1), Bool(true), Str("a"), TupleEnd]
    });
}
