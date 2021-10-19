#![feature(test)]

extern crate test;

use sval_generic_api::{
    coroutine::CoroutineValue, receiver::Display, source::ValueSource, Receiver, Result,
};
use sval_generic_api_json_twitter_tests::Twitter;

fn main() {
    let s = input_struct();

    for _ in 0..30_000 {
        let _ = test::black_box(s.stream_iter(Empty));
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
