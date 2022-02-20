#![feature(test)]

extern crate test;

use sval_json_twitter_tests::Twitter;

fn input_struct() -> Twitter {
    serde_json::from_str(include_str!("../../tests/twitter/twitter.json")).unwrap()
}

fn main() {
    let data = input_struct();

    let mut s = String::new();
    for _ in 0..10_000 {
        s.clear();
        test::black_box(sval_json::to_fmt(&mut s, &data).unwrap());
    }
}
