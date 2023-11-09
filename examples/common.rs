#![allow(dead_code)]

use just_convert_rs::JustConvert;

#[derive(Clone, JustConvert, Default)]
#[convert(from_into(other::Mouse))]
#[convert(from_into(Cat, default))]
struct Dog {
    #[convert(unwrap(from(Cat)))]
    name: String,
    #[convert(skip(from(other::Mouse)))]
    #[convert(map(from(Cat, ". as i64"), into(Cat, ". as u64")))]
    age: i64,

    // TODO
    // inner of Option must be autoconvert
    #[convert(skip(from_into(other::Mouse)))]
    #[convert(map = ".map(Into::into)")]
    error: Option<DogError>,
}

#[derive(Clone, JustConvert, Default)]
struct Cat {
    name: Option<String>,
    age: u64,
    // inner of Option must be autoconvert
    error: Option<CatError>,
}

mod other {
    pub struct Mouse {
        pub name: String,
    }
}

#[derive(Clone)]
pub struct DogError;
#[derive(Clone)]
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
