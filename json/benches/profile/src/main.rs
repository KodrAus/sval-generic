#![feature(test)]

extern crate test;

use sval_generic_api::generator::GeneratorValue;
use sval_generic_api_json_twitter_tests::Twitter;

fn main() {
    let s = input_struct();
    let s = s.as_value_iter();

    println!("{}", std::mem::size_of::<Twitter>());
    println!(
        "{}",
        std::mem::size_of::<
            sval_generic_api::generator::Generator<
                '_,
                sval_generic_api_json::Formatter<&mut String>,
                Twitter,
            >,
        >()
    );

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
