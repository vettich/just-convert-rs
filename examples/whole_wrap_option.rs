#![allow(dead_code)]

use just_convert::JustConvert;

#[derive(JustConvert)]
#[convert(from(B, wrap_option))]
struct A {
    id: Option<i64>,
    abc: Option<i64>,
    def: Option<i64>,
}

#[derive(Debug, PartialEq)]
struct B {
    id: i64,
    abc: i64,
    def: i64,
}

fn main() {}
