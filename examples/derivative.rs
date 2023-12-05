#![allow(dead_code)]

use derivative::Derivative;
use just_convert::JustConvert;

#[derive(JustConvert, Derivative)]
#[derivative(Default)]
#[convert(from(B, default))]
struct A {
    id: i64,
    #[derivative(Default(value = "C(0)"))]
    #[convert(skip)]
    c: C,
}

#[derive(Debug, PartialEq)]
struct B {
    id: i64,
}

struct C(i64);

fn main() {}
