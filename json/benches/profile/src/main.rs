#![feature(test)]

extern crate test;

use sval_generic_api::receiver::{self, Receiver};
use sval_generic_api_json_twitter_tests::Twitter;
use sval_generic_api_serde as serde;

fn main() {
    let s = input_struct();
    let s = serde::value(s);

    for _ in 0..5_000 {
        let _ = test::black_box(serde_json::to_string(&s));
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
    fn display<D: receiver::Display>(&mut self, _: D) -> receiver::Result {
        Ok(())
    }

    fn none(&mut self) -> receiver::Result {
        Ok(())
    }

    fn str<'v: 'a, V: receiver::ValueSource<'v, str>>(&mut self, _: V) -> receiver::Result {
        Ok(())
    }

    fn map_begin(&mut self, _: Option<usize>) -> receiver::Result {
        Ok(())
    }

    fn map_end(&mut self) -> receiver::Result {
        Ok(())
    }

    fn map_key_begin(&mut self) -> receiver::Result {
        Ok(())
    }

    fn map_key_end(&mut self) -> receiver::Result {
        Ok(())
    }

    fn map_value_begin(&mut self) -> receiver::Result {
        Ok(())
    }

    fn map_value_end(&mut self) -> receiver::Result {
        Ok(())
    }

    fn seq_begin(&mut self, _: Option<usize>) -> receiver::Result {
        Ok(())
    }

    fn seq_end(&mut self) -> receiver::Result {
        Ok(())
    }

    fn seq_elem_begin(&mut self) -> receiver::Result {
        Ok(())
    }

    fn seq_elem_end(&mut self) -> receiver::Result {
        Ok(())
    }
}
