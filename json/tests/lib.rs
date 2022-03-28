#[macro_use]
extern crate sval_derive;

#[macro_use]
extern crate serde_derive;

#[derive(Value, Serialize)]
pub enum Enum {
    Constant,
    Tagged(i32),
    Map { a: i32, b: bool },
    Seq(i32, bool),
}

#[derive(Value, Serialize)]
pub struct Map {
    a: i32,
    b: bool,
}

#[derive(Value, Serialize)]
pub struct Seq(i32, bool);

#[test]
fn enum_consistency() {
    for e in [
        Enum::Constant,
        Enum::Tagged(42),
        Enum::Map { a: 42, b: true },
        Enum::Seq(42, true),
    ] {
        assert_eq!(
            serde_json::to_string(&e).unwrap(),
            sval_json::to_string(&e).unwrap(),
        );
    }
}

#[test]
fn struct_consistency() {
    assert_eq!(
        serde_json::to_string(&Map { a: 42, b: true }).unwrap(),
        sval_json::to_string(&Map { a: 42, b: true }).unwrap(),
    );
}

#[test]
fn tuple_consistency() {
    assert_eq!(
        serde_json::to_string(&Seq(42, true)).unwrap(),
        sval_json::to_string(&Seq(42, true)).unwrap(),
    );
}

#[test]
fn enum_roundtrip() {
    for e in [
        Enum::Constant,
        Enum::Tagged(42),
        Enum::Map { a: 42, b: true },
        Enum::Seq(42, true),
    ] {
        let json = sval_json::to_string(&e).unwrap();

        let roundtrip = sval_json::to_string(&sval_json::slice(&json)).unwrap();

        assert_eq!(json, roundtrip);
    }
}

#[test]
fn struct_roundtrip() {
    let json = sval_json::to_string(&Map { a: 42, b: true }).unwrap();

    let roundtrip = sval_json::to_string(&sval_json::slice(&json)).unwrap();

    assert_eq!(json, roundtrip);
}

#[test]
fn tuple_roundtrip() {
    let json = sval_json::to_string(&Seq(42, true)).unwrap();

    let roundtrip = sval_json::to_string(&sval_json::slice(&json)).unwrap();

    assert_eq!(json, roundtrip);
}

#[test]
fn slice_convert() {
    use sval::Value;

    assert_eq!(Some(true), sval_json::slice("true").to_bool());
    assert_eq!(Some("a string"), sval_json::slice("\"a string\"").to_str());
}
