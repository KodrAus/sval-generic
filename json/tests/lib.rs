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

struct BinFloat<'a>(&'a [&'a str]);

impl<'a> sval::Value for BinFloat<'a> {
    fn stream<'b, R: sval::Receiver<'b>>(&'b self, mut receiver: R) -> sval::Result {
        receiver.binfloat_begin()?;
        receiver.text_begin(None)?;

        for fragment in self.0 {
            receiver.text_fragment(fragment)?;
        }

        receiver.text_end()?;
        receiver.binfloat_end()
    }
}

#[test]
fn float_normal_contiguous() {
    for case in [
        "0",
        "1",
        "3454475",
        "728725.788864389",
        "-0",
        "-1",
        "-87235.54387",
        "3.7587238e10",
        "3.7587238e+10",
        "3.7587238e-10",
        "-3.7587238e10",
        "-3.7587238e+10",
        "-3.7587238e-10",
    ] {
        assert_eq!(case, sval_json::to_string(&BinFloat(&[case])).unwrap());
    }
}

#[test]
fn float_normal_chunked() {
    for case in [
        &["345", "4475"] as &[&str],
        &["72", "8725.", "78886", "4389"],
        &["-", "0"],
        &["-", "1"],
        &["-8", "7235.54387"],
        &["3.", "7587238e10"],
        &["3", ".", "7587238", "e+10"],
        &["-", "8", "7", "2", "3", "5", ".", "5", "4", "3", "8", "7"],
    ] {
        let expected = case.join("");
        assert_eq!(expected, sval_json::to_string(&BinFloat(case)).unwrap());
    }
}

#[test]
fn float_leading_plus_contiguous() {
    for (case, expected) in [
        ("+0", "0"),
        ("+12432.7593", "12432.7593"),
        ("+1.7593e7", "1.7593e7"),
    ] {
        assert_eq!(expected, sval_json::to_string(&BinFloat(&[case])).unwrap());
    }
}

#[test]
fn float_leading_plus_chunked() {
    for (case, expected) in [
        (&["+", "0"] as &[&str], "0"),
        (&["+1", "2432.7593"], "12432.7593"),
    ] {
        assert_eq!(expected, sval_json::to_string(&BinFloat(case)).unwrap());
    }
}

#[test]
fn float_nan_inf_contiguous() {
    for case in ["nan", "NaN", "inf", "INF", "+inf", "+INF", "-inf", "-INF"] {
        assert_eq!("null", sval_json::to_string(&BinFloat(&[case])).unwrap());
    }
}

#[test]
fn float_nan_inf_chunked() {
    for case in [
        &["n", "a", "n"] as &[&str],
        &["N", "a", "N"],
        &["i", "n", "f"],
        &["I", "N", "F"],
        &["+", "i", "n", "f"],
        &["+i", "nf"],
        &["-", "i", "n", "f"],
        &["-i", "nf"],
    ] {
        assert_eq!("null", sval_json::to_string(&BinFloat(case)).unwrap());
    }
}
