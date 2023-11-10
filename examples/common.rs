#![allow(dead_code)]

use just_convert::JustConvert;

/// Convert between Dog and Cat, Dog and Mouse
///
/// Using map, skip, unwrap field helpers
#[derive(JustConvert, Default)]
#[convert(from_into(other::Mouse))]
#[convert(from_into(Cat, default))]
struct Dog {
    #[convert(unwrap(from(Cat)))]
    name: String,
    #[convert(skip(from(other::Mouse)))]
    #[convert(map(from(Cat, ". as i64"), into(Cat, ". as u64")))]
    age: i64,

    // inner of Option must be autoconvert
    #[convert(skip(from_into(other::Mouse)))]
    error: Option<DogError>,

    // inners of Vec must be autoconvert
    #[convert(skip(from_into(other::Mouse)))]
    messages: Vec<DogError>,

    // inners of Option of Vec must be autoconvert
    #[convert(skip(from_into(other::Mouse)))]
    items: Option<Vec<DogError>>,
}

#[derive(JustConvert, Debug, Default, PartialEq)]
struct Cat {
    name: Option<String>,
    age: u64,
    // inner of Option must be autoconvert
    error: Option<CatError>,
    // inners of Vec must be autoconvert
    messages: Vec<CatError>,
    // inners of Option of Vec must be autoconvert
    items: Option<Vec<CatError>>,
}

mod other {
    pub struct Mouse {
        pub name: String,
    }
}

#[derive(Clone, Debug)]
pub struct DogError;
#[derive(Clone, Debug, PartialEq)]
pub struct CatError;

impl From<DogError> for CatError {
    fn from(_value: DogError) -> Self {
        Self
    }
}

impl From<CatError> for DogError {
    fn from(_value: CatError) -> Self {
        Self
    }
}

fn main() {}

#[test]
fn test_dog_into_cat() {
    let a = Dog {
        name: "John".into(),
        age: 12,
        error: Some(DogError),
        messages: vec![DogError],
        items: Some(vec![DogError]),
    };
    let b: Cat = a.into();
    debug_assert_eq!(
        Cat {
            name: Some("John".into()),
            age: 12,
            error: Some(CatError),
            messages: vec![CatError],
            items: Some(vec![CatError]),
        },
        b
    );
}
