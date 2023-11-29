#![allow(dead_code)]

use just_convert::JustConvert;

#[derive(JustConvert)]
#[convert(from(B))]
struct A {
    #[convert(map = "to_str_id(.)")]
    id: String,
}

struct B {
    id: i32,
}

fn to_str_id(id: i32) -> String {
    id.to_string()
}

fn main() {}
