#![allow(dead_code)]

use just_convert::JustConvert;

#[derive(JustConvert)]
#[convert(from(B))]
struct A {
    #[convert(map = "if . { Some(42) } else { None }")]
    param: Option<i32>,
}

struct B {
    param: bool,
}

fn main() {}
