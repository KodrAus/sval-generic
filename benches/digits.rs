#![feature(test)]
extern crate test;

use sval::digits::Digits;

#[bench]
fn new_zero(b: &mut test::Bencher) {
    b.iter(|| Digits::new(&"0"));
}

#[bench]
fn new_u8_max(b: &mut test::Bencher) {
    let max = u8::MAX.to_string();

    b.iter(|| Digits::new(&max));
}

#[bench]
fn new_u16_max(b: &mut test::Bencher) {
    let max = u16::MAX.to_string();

    b.iter(|| Digits::new(&max));
}

#[bench]
fn new_u32_max(b: &mut test::Bencher) {
    let max = u32::MAX.to_string();

    b.iter(|| Digits::new(&max));
}

#[bench]
fn new_u64_max(b: &mut test::Bencher) {
    let max = u64::MAX.to_string();

    b.iter(|| Digits::new(&max));
}

#[bench]
fn new_u128_max(b: &mut test::Bencher) {
    let max = u128::MAX.to_string();

    b.iter(|| Digits::new(&max));
}

#[bench]
fn new_i8_min(b: &mut test::Bencher) {
    let min = i8::MIN.to_string();

    b.iter(|| Digits::new(&min));
}

#[bench]
fn new_i16_min(b: &mut test::Bencher) {
    let min = i16::MIN.to_string();

    b.iter(|| Digits::new(&min));
}

#[bench]
fn new_i32_min(b: &mut test::Bencher) {
    let min = i32::MIN.to_string();

    b.iter(|| Digits::new(&min));
}

#[bench]
fn new_i64_min(b: &mut test::Bencher) {
    let min = i64::MIN.to_string();

    b.iter(|| Digits::new(&min));
}

#[bench]
fn new_i128_min(b: &mut test::Bencher) {
    let min = i128::MIN.to_string();

    b.iter(|| Digits::new(&min));
}
