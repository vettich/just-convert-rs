#![allow(dead_code)]

use just_convert::JustConvert;

#[derive(JustConvert)]
#[convert(into(B))]
struct A {
    #[convert(rename = bid)]
    id: i64,
    #[convert(map = ".to_string()")]
    num: i64,
    #[convert(unwrap)]
    name: Option<String>,
}

#[derive(Debug, PartialEq)]
struct B {
    bid: i64,
    num: String,
    name: String,
}

fn main() {}

#[test]
fn test_basic() {
    let a = A {
        id: 2,
        num: 1,
        name: Some("Jack".to_string()),
    };
    let b: B = a.into();
    debug_assert_eq!(
        B {
            num: "1".to_string(),
            bid: 2,
            name: "Jack".to_string(),
        },
        b
    );
}
