#![feature(test)]

extern crate test;

use sval_generic_api_json_twitter_tests::Twitter;

fn main() {
    let s = input_struct();

    for _ in 0..30_000 {
        let r = sval_generic_api_json::to_string(&s).unwrap();
        test::black_box(r);
    }
}

fn input_json() -> String {
    std::fs::read_to_string("./twitter.json").unwrap()
}

fn input_struct() -> Twitter {
    let j = input_json();
    serde_json::from_str(&j).unwrap()
}
