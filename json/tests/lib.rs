#[macro_use]
extern crate sval_derive;

#[macro_use]
extern crate serde_derive;

use sval::Tag;

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
fn option_roundtrip() {
    let json = sval_json::to_string(&Some(42)).unwrap();

    let roundtrip = sval_json::to_string(&sval_json::slice(&json)).unwrap();

    assert_eq!(json, roundtrip);
}

#[test]
fn anonymous_enum() {
    pub enum AnonymousEnum {
        I32(i32),
        Bool(bool),
    }

    impl sval::Value for AnonymousEnum {
        fn stream<'sval, S: sval::Stream<'sval> + ?Sized>(
            &'sval self,
            stream: &mut S,
        ) -> sval::Result {
            stream.enum_begin(Tag::Structural(None))?;
            stream.tagged_begin(Tag::Structural(None))?;

            match self {
                AnonymousEnum::I32(v) => sval::stream(&mut *stream, v)?,
                AnonymousEnum::Bool(v) => sval::stream(&mut *stream, v)?,
            }

            stream.tagged_end(Tag::Structural(None))?;
            stream.enum_end(Tag::Structural(None))
        }
    }

    assert_eq!(
        "true",
        sval_json::to_string(&AnonymousEnum::Bool(true)).unwrap(),
    );
    assert_eq!("42", sval_json::to_string(&AnonymousEnum::I32(42)).unwrap());
}

#[test]
fn slice_convert() {
    use sval::Value;

    assert_eq!(Some(true), sval_json::slice("true").to_bool());
    assert_eq!(Some("a string"), sval_json::slice("\"a string\"").to_text());
}

struct Printer;

impl<'a> sval::Stream<'a> for Printer {
    fn null(&mut self) -> sval::Result {
        println!("null");
        Ok(())
    }

    fn text_begin(&mut self, _: Option<usize>) -> sval::Result {
        println!("text_begin");
        Ok(())
    }

    fn text_fragment_computed(&mut self, fragment: &str) -> sval::Result {
        println!("text_fragment_computed: {:?}", fragment);
        Ok(())
    }

    fn text_end(&mut self) -> sval::Result {
        println!("text_end");
        Ok(())
    }

    fn binary_begin(&mut self, _: Option<usize>) -> sval::Result {
        println!("binary_begin");
        Ok(())
    }

    fn binary_fragment_computed(&mut self, fragment: &[u8]) -> sval::Result {
        println!("binary_fragment_computed: {:?}", fragment);
        Ok(())
    }

    fn binary_end(&mut self) -> sval::Result {
        println!("binary_end");
        Ok(())
    }

    fn map_begin(&mut self, _: Option<usize>) -> sval::Result {
        println!("map_begin");
        Ok(())
    }

    fn map_key_begin(&mut self) -> sval::Result {
        println!("map_key_begin");
        Ok(())
    }

    fn map_key_end(&mut self) -> sval::Result {
        println!("map_key_end");
        Ok(())
    }

    fn map_value_begin(&mut self) -> sval::Result {
        println!("map_value_begin");
        Ok(())
    }

    fn map_value_end(&mut self) -> sval::Result {
        println!("map_value_end");
        Ok(())
    }

    fn map_end(&mut self) -> sval::Result {
        println!("map_end");
        Ok(())
    }

    fn seq_begin(&mut self, _: Option<usize>) -> sval::Result {
        println!("seq_begin");
        Ok(())
    }

    fn seq_value_begin(&mut self) -> sval::Result {
        println!("seq_value_begin");
        Ok(())
    }

    fn seq_value_end(&mut self) -> sval::Result {
        println!("seq_value_end");
        Ok(())
    }

    fn seq_end(&mut self) -> sval::Result {
        println!("seq_end");
        Ok(())
    }
}

#[test]
fn co() {
    let json = "\"Hello\\nWorld\"";

    let mut reader = sval_json::JsonSliceCoReader::begin(json.as_bytes(), Printer);

    while reader.resume().unwrap() {
        println!("suspending");
    }
}

struct Number<'a>(&'a [&'a str]);

impl<'a> sval::Value for Number<'a> {
    fn stream<'b, S: sval::Stream<'b> + ?Sized>(&'b self, stream: &mut S) -> sval::Result {
        stream.number_begin()?;
        stream.text_begin(None)?;

        for fragment in self.0 {
            stream.text_fragment(fragment)?;
        }

        stream.text_end()?;
        stream.number_end()
    }
}

#[test]
fn float_normal_contiguous() {
    for (case, expected) in [
        ("0", "0"),
        ("1", "1"),
        ("3454475", "3454475"),
        ("728725.788864389", "728725.788864389"),
        ("00000000", "0"),
        ("000000001", "1"),
        ("-0", "0"),
        ("-1", "-1"),
        ("-87235.54387", "-87235.54387"),
        ("3.7587238e10", "3.7587238e10"),
        ("3.7587238e+10", "3.7587238e+10"),
        ("3.7587238e-10", "3.7587238e-10"),
        ("-3.7587238e10", "-3.7587238e10"),
        ("-3.7587238e+10", "-3.7587238e+10"),
        ("-3.7587238e-10", "-3.7587238e-10"),
    ] {
        assert_eq!(expected, sval_json::to_string(&Number(&[case])).unwrap());
    }
}

#[test]
fn float_normal_chunked() {
    for (case, expected) in [
        (&["345", "4475"] as &[&str], "3454475"),
        (&["72", "8725.", "78886", "4389"], "728725.788864389"),
        (&["0", "000", "0000"], "0"),
        (&["0", "000", "00001"], "1"),
        (&["-", "0"], "0"),
        (&["-", "1"], "-1"),
        (&["-8", "7235.54387"], "-87235.54387"),
        (&["3.", "7587238e10"], "3.7587238e10"),
        (&["3", ".", "7587238", "e+10"], "3.7587238e+10"),
        (
            &["-", "8", "7", "2", "3", "5", ".", "5", "4", "3", "8", "7"],
            "-87235.54387",
        ),
    ] {
        assert_eq!(expected, sval_json::to_string(&Number(case)).unwrap());
    }
}

#[test]
fn float_leading_plus_contiguous() {
    for (case, expected) in [
        ("+0", "0"),
        ("+12432.7593", "12432.7593"),
        ("+1.7593e7", "1.7593e7"),
    ] {
        assert_eq!(expected, sval_json::to_string(&Number(&[case])).unwrap());
    }
}

#[test]
fn float_leading_plus_chunked() {
    for (case, expected) in [
        (&["+", "0"] as &[&str], "0"),
        (&["+1", "2432.7593"], "12432.7593"),
    ] {
        assert_eq!(expected, sval_json::to_string(&Number(case)).unwrap());
    }
}

#[test]
fn float_nan_inf_contiguous() {
    for case in ["nan", "NaN", "inf", "INF", "+inf", "+INF", "-inf", "-INF"] {
        assert_eq!("null", sval_json::to_string(&Number(&[case])).unwrap());
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
        assert_eq!("null", sval_json::to_string(&Number(case)).unwrap());
    }
}
