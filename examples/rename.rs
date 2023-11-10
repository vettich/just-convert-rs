#![allow(dead_code)]

use just_convert::JustConvert;

#[derive(JustConvert)]
#[convert(from(B))]
#[convert(from_into(C))]
struct A {
    #[convert(rename(from_into(C, c_id)))]
    #[convert(rename(from = user_id))]
    id: String,
    #[convert(skip)]
    age: i64,
}

struct B {
    user_id: String,
}

struct C {
    c_id: String,
}

fn main() {}
