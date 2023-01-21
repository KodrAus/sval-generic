#![cfg(test)]

#[macro_use]
extern crate sval_derive;

#[macro_use]
extern crate serde_derive;

use serde_test::assert_ser_tokens;

use std::collections::BTreeMap;

type Map = BTreeMap<&'static str, i32>;

type Seq = Vec<i32>;

#[derive(derive_value, Serialize)]
struct MapStruct {
    field_0: i32,
    field_1: bool,
    field_2: &'static str,
}

#[derive(derive_value, Serialize)]
struct SeqStruct(i32, bool, &'static str);

#[derive(derive_value, Serialize)]
struct Tagged(i32);

#[derive(derive_value, Serialize)]
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
    assert_ser_tokens(
        &sval_serde::to_serialize(sval_buffer::stream_to_value(&v).unwrap()),
        tokens,
    );
    assert_ser_tokens(
        &sval_serde::to_serialize(&v as &dyn sval_dynamic::Value),
        tokens,
    );
    assert_ser_tokens(&v, tokens);
}

#[test]
fn unit_to_serialize() {
    serialize_case((), {
        use serde_test::Token::*;

        &[Unit]
    })
}

#[test]
fn option_some_to_serialize() {
    serialize_case(Some(1i32), {
        use serde_test::Token::*;

        &[Some, I32(1)]
    })
}

#[test]
fn option_none_to_serialize() {
    serialize_case(None::<i32>, {
        use serde_test::Token::*;

        &[None]
    })
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
    serialize_case(SeqStruct(1, true, "a"), {
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

#[test]
fn tagged_struct_to_serialize() {
    serialize_case(Tagged(1), {
        use serde_test::Token::*;

        &[NewtypeStruct { name: "Tagged" }, I32(1)]
    })
}

#[test]
fn enum_tag_to_serialize() {
    serialize_case(Enum::Constant, {
        use serde_test::Token::*;

        &[UnitVariant {
            name: "Enum",
            variant: "Constant",
        }]
    });
}

#[test]
fn enum_tagged_to_serialize() {
    serialize_case(Enum::Tagged(1), {
        use serde_test::Token::*;

        &[
            NewtypeVariant {
                name: "Enum",
                variant: "Tagged",
            },
            I32(1),
        ]
    });
}

#[test]
fn enum_record_to_serialize() {
    serialize_case(
        Enum::MapStruct {
            field_0: 1,
            field_1: true,
            field_2: "a",
        },
        {
            use serde_test::Token::*;

            &[
                StructVariant {
                    name: "Enum",
                    variant: "MapStruct",
                    len: 3,
                },
                Str("field_0"),
                I32(1),
                Str("field_1"),
                Bool(true),
                Str("field_2"),
                Str("a"),
                StructVariantEnd,
            ]
        },
    );
}

#[test]
fn enum_tuple_to_serialize() {
    serialize_case(Enum::SeqStruct(1, true, "a"), {
        use serde_test::Token::*;

        &[
            TupleVariant {
                name: "Enum",
                variant: "SeqStruct",
                len: 3,
            },
            I32(1),
            Bool(true),
            Str("a"),
            TupleVariantEnd,
        ]
    });
}
