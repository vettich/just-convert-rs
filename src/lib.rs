//! just-convert make easier to convert between structs.
//!
//! This crate provides JustConvert derive macro.
//!
//! Example of use:
//!
//! ```rust
//! # use just_convert::JustConvert;
//! #
//! // Allow convert A struct into B struct
//! #[derive(JustConvert)]
//! #[convert(into(B))]
//! struct A {
//!     // field can be renamed
//!     #[convert(rename = bid)]
//!     id: i64,
//!
//!     // field can execute any expression
//!     #[convert(map = ".to_string()")]
//!     num: i64,
//!
//!     // unwrap Option value for B::name
//!     #[convert(unwrap)]
//!     name: Option<String>,
//! }
//!
//! struct B {
//!     bid: i64,
//!     num: String,
//!     name: String,
//! }
//! ```
//!
//! See more [examples](https://github.com/vettich/just-convert-rs/tree/main/examples)

use std::collections::HashMap;

use parse::parse_params;
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, Ident, Path};

mod build;
mod map;
mod parse;

#[proc_macro_derive(JustConvert, attributes(convert))]
pub fn just_convert_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    build_impl(input).into()
}

fn build_impl(input: DeriveInput) -> proc_macro2::TokenStream {
    let params = match parse_params(&input) {
        Ok(p) => p,
        Err(err) => return err.to_compile_error(),
    };

    params
        .build()
        .unwrap_or_else(syn::Error::into_compile_error)
}

#[derive(Debug)]
struct Params {
    name: Ident,
    from: Vec<PathParams>,
    into: Vec<PathParams>,
    fields: Fields,
}

#[derive(Debug)]
struct PathParams {
    path: Path,
    default: bool,
    wrap_option: bool,
}

#[derive(Debug, Clone)]
struct FieldParams {
    map: FieldValue<proc_macro2::Literal>,
    rename: FieldValue<Ident>,
    wrap: FieldValue<bool>,
    unwrap: FieldValue<bool>,
    skip: FieldValue<bool>,
    a_type: AdditionalType,
}

impl FieldParams {
    fn new() -> Self {
        Self {
            map: FieldValue::new(),
            rename: FieldValue::new(),
            wrap: FieldValue::new(),
            unwrap: FieldValue::new(),
            skip: FieldValue::new(),
            a_type: AdditionalType::None,
        }
    }
}

#[derive(Debug, Default, Clone)]
struct FieldValue<T> {
    common: Option<T>,
    common_from: Option<T>,
    common_into: Option<T>,
    from: HashMap<Path, T>,
    into: HashMap<Path, T>,
}

impl<T> FieldValue<T> {
    fn new() -> Self {
        Self {
            common: None,
            common_from: None,
            common_into: None,
            from: [].into(),
            into: [].into(),
        }
    }

    fn set_from(&mut self, path: Option<Path>, value: T) {
        if let Some(path) = path {
            self.from.insert(path, value);
        } else {
            self.common_from = Some(value);
        }
    }

    fn set_into(&mut self, path: Option<Path>, value: T) {
        if let Some(path) = path {
            self.into.insert(path, value);
        } else {
            self.common_into = Some(value);
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
enum AdditionalType {
    #[default]
    None,
    /// Option<T>
    Option,
    /// Option<Vec<T>>
    OptionVec,
    /// Vec<T>
    Vec,
    /// Vec<Option<T>>
    VecOption,
}

impl AdditionalType {
    fn is_option(self) -> bool {
        matches!(self, Self::Option)
    }

    fn is_option_vec(self) -> bool {
        matches!(self, Self::OptionVec)
    }

    fn is_vec(self) -> bool {
        matches!(self, Self::Vec)
    }

    fn is_vec_option(self) -> bool {
        matches!(self, Self::VecOption)
    }
}

type Fields = HashMap<Ident, FieldParams>;
