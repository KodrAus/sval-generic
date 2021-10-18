#![cfg(test)]
#![feature(test)]

extern crate test;

use sval_generic_api_json_twitter_tests::Twitter;

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
fn primitive_sval_generic_api(b: &mut test::Bencher) {
    b.iter(|| sval_generic_api_json::to_string(&42).unwrap());
}

#[bench]
fn primitive_erased_sval_generic_api(b: &mut test::Bencher) {
    use sval_generic_api_erased as erased;

    let s = 42;
    let s = erased::value(&s);

    b.iter(|| sval_generic_api_json::to_string(&s).unwrap());
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
fn twitter_sval_generic_api_generator(b: &mut test::Bencher) {
    use sval_generic_api::generator::GeneratorValue;

    let s = input_struct();
    let s = s.as_value_iter();

    b.iter(|| sval_generic_api_json::to_string(&s).unwrap());
}

#[bench]
fn twitter_sval_generic_api_coroutine(b: &mut test::Bencher) {
    use sval_generic_api::coroutine::CoroutineValue;

    let s = input_struct();
    let s = s.as_value_iter();

    b.iter(|| sval_generic_api_json::to_string(&s).unwrap());
}

#[bench]
fn twitter_sval_generic_api(b: &mut test::Bencher) {
    let s = input_struct();
    b.iter(|| sval_generic_api_json::to_string(&s).unwrap());
}

#[bench]
fn twitter_erased_sval_generic_api(b: &mut test::Bencher) {
    use sval_generic_api_erased as erased;

    let s = input_struct();
    let s = erased::value(&s);

    b.iter(|| sval_generic_api_json::to_string(&s).unwrap());
}

#[bench]
fn twitter_sval_generic_api_to_serde(b: &mut test::Bencher) {
    use sval_generic_api_serde as serde;

    let s = input_struct();
    let s = serde::value(s);

    b.iter(|| serde_json::to_string(&s).unwrap());
}

#[bench]
fn twitter_sval_generic_api_to_valuable(b: &mut test::Bencher) {
    use sval_generic_api_valuable as valuable;

    let s = input_struct();
    let s = valuable::value(&s);

    b.iter(|| valuable_json::to_string(&s).unwrap());
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
fn twitter_sval_generic_api_fmt(b: &mut test::Bencher) {
    use sval_generic_api_fmt as fmt;

    let s = input_struct();
    let s = Fmt(fmt::value(s));

    b.iter(|| s.to_string());
}
