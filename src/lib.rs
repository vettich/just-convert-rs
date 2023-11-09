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
}

#[derive(Debug, Clone)]
struct FieldParams {
    map: FieldValue<proc_macro2::Literal>,
    rename: FieldValue<Ident>,
    wrap: FieldValue<bool>,
    unwrap: FieldValue<bool>,
    skip: FieldValue<bool>,
}

impl FieldParams {
    fn new() -> Self {
        Self {
            map: FieldValue::new(),
            rename: FieldValue::new(),
            wrap: FieldValue::new(),
            unwrap: FieldValue::new(),
            skip: FieldValue::new(),
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

type Fields = HashMap<Ident, FieldParams>;
