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

fn main() {
    let v = input_struct();
    let mut w = String::new();

    for _ in 0..50_000 {
        w.clear();
        sval_json::to_fmt(&mut w, &v).unwrap();

        test::black_box(&mut w);
    }
}
