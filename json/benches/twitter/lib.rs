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
fn primitive_sval(b: &mut test::Bencher) {
    b.iter(|| sval_json::to_string(&42).unwrap());
}

#[bench]
fn primitive_sval_generic_api(b: &mut test::Bencher) {
    b.iter(|| sval_generic_api_json::to_string(&42).unwrap());
}

#[bench]
fn primitive_erased_sval_generic_api(b: &mut test::Bencher) {
    use sval_generic_api::Value;

    let s = 42;
    let s = s.erase();

    b.iter(|| sval_generic_api_json::to_string(&s).unwrap());
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
fn twitter_sval_generic_api(b: &mut test::Bencher) {
    let s = input_struct();
    b.iter(|| sval_generic_api_json::to_string(&s).unwrap());
}

#[bench]
fn twitter_erased_sval_generic_api(b: &mut test::Bencher) {
    use sval_generic_api::Value;

    let s = input_struct();
    let s = s.erase();

    b.iter(|| sval_generic_api_json::to_string(&s).unwrap());
}
