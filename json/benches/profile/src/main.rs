#![feature(test)]

extern crate test;

use sval_generic_api::{
    Receiver,
    Result,
    source::ValueSource,
    generator::GeneratorValue,
    receiver::Display,
};
use sval_generic_api_json_twitter_tests::Twitter;

fn main() {
    let s = input_struct();

    for _ in 0..30_000 {
        let r = s.stream_iter(Empty);
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

struct Empty;

impl<'a> Receiver<'a> for Empty {
    fn display<D: Display>(&mut self, _: D) -> Result {
        Ok(())
    }

    fn none(&mut self) -> Result {
        Ok(())
    }

    fn str<'v: 'a, V: ValueSource<'v, str>>(&mut self, _: V) -> Result {
        Ok(())
    }

    fn map_begin(&mut self, _: Option<usize>) -> Result {
        Ok(())
    }

    fn map_end(&mut self) -> Result {
        Ok(())
    }

    fn map_key_begin(&mut self) -> Result {
        Ok(())
    }

    fn map_key_end(&mut self) -> Result {
        Ok(())
    }

    fn map_value_begin(&mut self) -> Result {
        Ok(())
    }

    fn map_value_end(&mut self) -> Result {
        Ok(())
    }

    fn seq_begin(&mut self, _: Option<usize>) -> Result {
        Ok(())
    }

    fn seq_end(&mut self) -> Result {
        Ok(())
    }

    fn seq_elem_begin(&mut self) -> Result {
        Ok(())
    }

    fn seq_elem_end(&mut self) -> Result {
        Ok(())
    }
}
