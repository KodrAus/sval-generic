#![cfg(test)]
#![feature(test)]

extern crate test;

use sval_json_tests_twitter::Twitter;

fn input_json() -> String {
    std::fs::read_to_string("../../tests/twitter/twitter.json").unwrap()
}

fn input_struct() -> Twitter {
    let j = input_json();
    serde_json::from_str(&j).unwrap()
}

#[bench]
fn primitive_miniserde(b: &mut test::Bencher) {
    b.iter(|| miniserde::json::to_string(&42));
}

#[bench]
fn primitive_serde(b: &mut test::Bencher) {
    b.iter(|| serde_json::to_string(&42).unwrap());
}

#[bench]
fn primitive_erased_serde(b: &mut test::Bencher) {
    let s: Box<dyn erased_serde::Serialize> = Box::new(42);

    b.iter(|| serde_json::to_string(&s).unwrap());
}

#[bench]
fn primitive_sval(b: &mut test::Bencher) {
    b.iter(|| sval_json::to_string(&42).unwrap());
}

#[bench]
fn primitive_sval_dynamic(b: &mut test::Bencher) {
    use sval_dynamic as dynamic;

    let s = 42;
    let s = &s as &dyn dynamic::Value;

    b.iter(|| sval_json::to_string(s).unwrap());
}

#[bench]
fn primitive_valuable(b: &mut test::Bencher) {
    b.iter(|| valuable_json::to_string(&42).unwrap());
}

#[bench]
fn twitter_miniserde(b: &mut test::Bencher) {
    let s = input_struct();
    b.iter(|| miniserde::json::to_string(&s));
}

#[bench]
fn twitter_serde(b: &mut test::Bencher) {
    let s = input_struct();
    b.iter(|| serde_json::to_string(&s).unwrap());
}

#[bench]
fn twitter_erased_serde(b: &mut test::Bencher) {
    let s = input_struct();
    let s: &dyn erased_serde::Serialize = &s;

    b.iter(|| serde_json::to_string(&s).unwrap());
}

#[bench]
fn twitter_sval(b: &mut test::Bencher) {
    let s = input_struct();
    b.iter(|| sval_json::to_string(&s).unwrap());
}

#[bench]
fn twitter_sval_dynamic(b: &mut test::Bencher) {
    use sval_dynamic as dynamic;

    let s = input_struct();
    let s = &s as &dyn dynamic::Value;

    b.iter(|| sval_json::to_string(&s).unwrap());
}

#[bench]
fn twitter_valuable(b: &mut test::Bencher) {
    let s = input_struct();
    b.iter(|| valuable_json::to_string(&s).unwrap());
}

struct Fmt<T>(T);

impl<T: std::fmt::Debug> std::fmt::Display for Fmt<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[bench]
fn twitter_std_fmt(b: &mut test::Bencher) {
    let s = input_struct();
    let s = Fmt(&s);

    b.iter(|| s.to_string());
}

#[bench]
fn twitter_scan_sval(b: &mut test::Bencher) {
    let json = input_json();

    b.iter(|| {
        use sval::Value;

        let json = sval_json::JsonSlice::new(&json);

        json.stream(EmptyStream).unwrap()
    });
}

struct EmptyStream;

impl<'a> sval::Stream<'a> for EmptyStream {
    #[inline(never)]
    fn dynamic_begin(&mut self) -> sval::Result {
        Ok(())
    }

    #[inline(never)]
    fn dynamic_end(&mut self) -> sval::Result {
        Ok(())
    }

    #[inline(never)]
    fn unit(&mut self) -> sval::Result {
        Ok(())
    }

    #[inline(never)]
    fn null(&mut self) -> sval::Result {
        Ok(())
    }

    #[inline(never)]
    fn text_begin(&mut self, _: Option<usize>) -> sval::Result {
        Ok(())
    }

    #[inline(never)]
    fn text_fragment_computed(&mut self, _: &str) -> sval::Result {
        Ok(())
    }

    #[inline(never)]
    fn text_end(&mut self) -> sval::Result {
        Ok(())
    }

    #[inline(never)]
    fn binary_begin(&mut self, _: Option<usize>) -> sval::Result {
        Ok(())
    }

    #[inline(never)]
    fn binary_fragment_computed(&mut self, _: &[u8]) -> sval::Result {
        Ok(())
    }

    #[inline(never)]
    fn binary_end(&mut self) -> sval::Result {
        Ok(())
    }

    #[inline(never)]
    fn map_begin(&mut self, _: Option<usize>) -> sval::Result {
        Ok(())
    }

    #[inline(never)]
    fn map_key_begin(&mut self) -> sval::Result {
        Ok(())
    }

    #[inline(never)]
    fn map_key_end(&mut self) -> sval::Result {
        Ok(())
    }

    #[inline(never)]
    fn map_value_begin(&mut self) -> sval::Result {
        Ok(())
    }

    #[inline(never)]
    fn map_value_end(&mut self) -> sval::Result {
        Ok(())
    }

    #[inline(never)]
    fn map_end(&mut self) -> sval::Result {
        Ok(())
    }

    #[inline(never)]
    fn seq_begin(&mut self, _: Option<usize>) -> sval::Result {
        Ok(())
    }

    #[inline(never)]
    fn seq_value_begin(&mut self) -> sval::Result {
        Ok(())
    }

    #[inline(never)]
    fn seq_value_end(&mut self) -> sval::Result {
        Ok(())
    }

    #[inline(never)]
    fn seq_end(&mut self) -> sval::Result {
        Ok(())
    }
}
