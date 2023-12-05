use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{Path, Result};

use crate::{map::parse_map_expr, FieldParams, FieldValue, Fields, Params, PathParams};

impl<T: Clone> FieldValue<T> {
    fn get_from(&self, path: &Path) -> Option<T> {
        self.from
            .get(path)
            .cloned()
            .or_else(|| self.common_from.clone())
            .or_else(|| self.common.clone())
    }

    fn get_into(&self, path: &Path) -> Option<T> {
        self.into
            .get(path)
            .cloned()
            .or_else(|| self.common_into.clone())
            .or_else(|| self.common.clone())
    }
}

impl Params {
    pub(crate) fn build(mut self) -> Result<TokenStream> {
        let from_impl = self.build_from()?;
        let into_impl = self.build_into()?;

        Ok(quote! {
            #from_impl
            #into_impl
        })
    }

    fn build_from(&mut self) -> Result<TokenStream> {
        let mut items = vec![];

        for from in &self.from {
            let current = self.name.clone();
            let from_path = &from.path;
            let assigns = build_from_assigns(from, self.fields.clone())?;

            let default_expr = if from.default {
                quote! { ..Default::default() }
            } else {
                quote!()
            };

            let item = quote! {
                impl std::convert::From<#from_path> for #current {
                    fn from(this: #from_path) -> Self {
                        #[allow(clippy::needless_update)]
                        #current {
                            #(#assigns)*
                            #default_expr
                        }
                    }
                }
            };
            items.push(item);
        }

        Ok(quote! {
            #(#items)*
        })
    }

    fn build_into(&mut self) -> Result<TokenStream> {
        let mut items = vec![];

        for into in &self.into {
            let current = self.name.clone();
            let into_path = &into.path;
            let assigns = build_into_assigns(into_path, self.fields.clone())?;

            let default_expr = if into.default {
                quote! { ..Default::default() }
            } else {
                quote!()
            };

            let item = quote! {
                impl std::convert::Into<#into_path> for #current {
                    fn into(self) -> #into_path {
                        let this = self;
                        #[allow(clippy::needless_update)]
                        #into_path {
                            #(#assigns)*
                            #default_expr
                        }
                    }
                }
            };
            items.push(item);
        }

        Ok(quote! {
            #(#items)*
        })
    }
}

fn build_from_assigns(target: &PathParams, fields: Fields) -> Result<Vec<TokenStream>> {
    let mut items = vec![];
    for (field, params) in fields {
        items.push(build_from_assign_item(field, params, target)?);
    }
    Ok(items)
}

fn build_from_assign_item(
    left_field: Ident,
    params: FieldParams,
    PathParams {
        path: target,
        default: target_default,
    }: &PathParams,
) -> syn::Result<TokenStream> {
    if params.skip.get_from(target).unwrap_or_default() {
        if *target_default {
            return Ok(quote!());
        }
        return Ok(quote! {
            #left_field: Default::default(),
        });
    }

    if let Some(map) = params.map.get_from(target) {
        let map_expr = parse_map_expr(left_field.clone(), map)?;
        return Ok(quote! {
            #left_field: #map_expr,
        });
    }

    let right_field = match params.rename.get_from(target) {
        Some(n) => n,
        None => left_field.clone(),
    };

    if params.unwrap.get_from(target).unwrap_or_default() {
        return Ok(quote! {
            #left_field: this.#right_field.unwrap_or_default(),
        });
    }

    if params.a_type.is_option() && params.wrap.get_from(target).unwrap_or_default() {
        return Ok(quote! {
            #left_field: Some(this.#right_field),
        });
    }

    if params.a_type.is_option() {
        return Ok(quote! {
            #left_field: this.#right_field.map(Into::into),
        });
    }

    if params.a_type.is_option_vec() {
        return Ok(quote! {
            #left_field: this.#right_field.map(|x| x.into_iter().map(Into::into).collect()),
        });
    }

    if params.a_type.is_vec() {
        return Ok(quote! {
            #left_field: this.#right_field.into_iter().map(Into::into).collect(),
        });
    }

    if params.a_type.is_vec_option() {
        return Ok(quote! {
            #left_field: this.#right_field.into_iter().map(|x| x.map(Into::into)).collect(),
        });
    }

    Ok(quote! {
        #left_field: this.#right_field.into(),
    })
}

fn build_into_assigns(target: &Path, fields: Fields) -> Result<Vec<TokenStream>> {
    let mut items = vec![];
    for (field, params) in fields {
        items.push(build_into_assign_item(field, params, target)?);
    }
    Ok(items)
}

fn build_into_assign_item(
    right_field: Ident,
    params: FieldParams,
    target: &Path,
) -> syn::Result<TokenStream> {
    if params.skip.get_into(target).unwrap_or_default() {
        return Ok(quote! {});
    }

    let left_field = match params.rename.get_into(target) {
        Some(n) => n,
        None => right_field.clone(),
    };

    if let Some(map) = params.map.get_into(target) {
        let map_expr = parse_map_expr(right_field, map)?;
        return Ok(quote! {
            #left_field: #map_expr,
        });
    }

    if params.unwrap.get_into(target).unwrap_or_default() {
        return Ok(quote! {
            #left_field: this.#right_field.unwrap_or_default(),
        });
    }

    if params.a_type.is_option() {
        return Ok(quote! {
            #left_field: this.#right_field.map(Into::into),
        });
    }

    if params.a_type.is_option_vec() {
        return Ok(quote! {
            #left_field: this.#right_field.map(|x| x.into_iter().map(Into::into).collect()),
        });
    }

    if params.a_type.is_vec() {
        return Ok(quote! {
            #left_field: this.#right_field.into_iter().map(Into::into).collect(),
        });
    }

    if params.a_type.is_vec_option() {
        return Ok(quote! {
            #left_field: this.#right_field.into_iter().map(|x| x.map(Into::into)).collect(),
        });
    }

    Ok(quote! {
        #left_field: this.#right_field.into(),
    })
}
