#![cfg(test)]

#[macro_use]
extern crate sval_derive;

#[macro_use]
extern crate serde_derive;

use std::collections::BTreeMap;

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

#[test]
fn complex_map_to_serialize() {
    use serde_test::{assert_ser_tokens, Token::*};

    let map = {
        let mut map = BTreeMap::new();

        map.insert("a", vec![1, 2, 3]);
        map.insert("b", vec![4, 5, 6]);

        map
    };

    let tokens = &[
        Map {
            len: Option::Some(2),
        },
        Str("a"),
        Seq {
            len: Option::Some(3),
        },
        I32(1),
        I32(2),
        I32(3),
        SeqEnd,
        Str("b"),
        Seq {
            len: Option::Some(3),
        },
        I32(4),
        I32(5),
        I32(6),
        SeqEnd,
        MapEnd,
    ];

    assert_ser_tokens(&map, tokens);
    assert_ser_tokens(&sval_serde::to_serialize(&map), tokens);
}
