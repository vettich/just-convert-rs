#![allow(dead_code)]

use just_convert::JustConvert;

// To convert between A and B structs use #[convert(from_into(Struct))]
#[derive(JustConvert)]
#[convert(from_into(B))]
struct A {
    #[convert(rename = bid)]
    id: i64,
    name: String,
    #[convert(wrap, unwrap(into))]
    age: Option<i64>,
}

struct B {
    bid: i64,
    name: String,
    age: i64,
}

fn main() {}
