use syn::{
    meta::ParseNestedMeta, parenthesized, token, Data, DeriveInput, FieldsNamed, Ident, Path,
    PathSegment, Result, Token,
};

use crate::{AdditionalType, FieldParams, FieldValue, Fields, Params, PathParams};

pub(crate) fn parse_params(input: &DeriveInput) -> Result<Params> {
    let (from, into) = parse_attributes(input)?;

    let fields = parse_fields(&input.data)?;

    let params = Params {
        name: input.ident.clone(),
        from,
        into,
        fields,
    };

    Ok(params)
}

fn parse_attributes(input: &DeriveInput) -> Result<(Vec<PathParams>, Vec<PathParams>)> {
    let mut into = vec![];
    let mut from = vec![];

    for attr in &input.attrs {
        if !attr.path().is_ident("convert") {
            continue;
        }

        attr.parse_nested_meta(|meta| {
            // if need to convert both for `from` and `into`
            let is_both = meta.path.is_ident("from_into") || meta.path.is_ident("into_from");

            let is_from = is_both || meta.path.is_ident("from");
            let is_into = is_both || meta.path.is_ident("into");

            if !is_from && !is_into {
                return Err(meta.error("unrecognized convert"));
            }

            // parse for path
            let content;
            parenthesized!(content in meta.input);
            let path: Path = content.parse()?;

            let mut default = false;
            if content.peek(Token![,]) && content.peek2(Token![default]) {
                content.parse::<Token![,]>()?;
                content.parse::<Token![default]>()?;
                default = true;
            }

            let mut wrap_option = false;
            if content.peek(Token![,]) && content.peek2(Ident) {
                content.parse::<Token![,]>()?;
                let attr = content.parse::<Ident>()?.to_string();
                if attr == "wrap_option" {
                    wrap_option = true;
                } else {
                    return Err(meta.error("unknown value"));
                }
            }

            if is_from {
                from.push(PathParams {
                    path: path.clone(),
                    default,
                    wrap_option,
                });
            }

            if is_into {
                into.push(PathParams {
                    path,
                    default,
                    wrap_option,
                });
            }

            Ok(())
        })?;
    }

    Ok((from, into))
}

fn parse_fields(data: &Data) -> Result<Fields> {
    match data {
        Data::Struct(s) => match &s.fields {
            syn::Fields::Named(d) => parse_named_struct_fields(d),
            syn::Fields::Unnamed(_) => Err(syn::Error::new(
                s.struct_token.span,
                "unnamed struct is not currently supported",
            )),
            syn::Fields::Unit => Err(syn::Error::new(
                s.struct_token.span,
                "unit is not currently supported",
            )),
        },
        Data::Enum(d) => Err(syn::Error::new(
            d.enum_token.span,
            "enum is not currently supported",
        )),
        Data::Union(d) => Err(syn::Error::new(
            d.union_token.span,
            "union is not currently supported",
        )),
    }
}

fn parse_named_struct_fields(d: &FieldsNamed) -> Result<Fields> {
    let mut fields: Fields = [].into();

    for field in &d.named {
        let Some(name) = field.ident.clone() else {
            continue;
        };

        let mut field_params = FieldParams::new();
        field_params.a_type = detect_type_for_serviced(&field.ty, 0);

        for attr in &field.attrs {
            if !attr.path().is_ident("convert") {
                continue;
            }

            attr.parse_nested_meta(|meta| {
                if parse_field_value("rename", &meta, &mut field_params.rename)? {
                    return Ok(());
                }

                if parse_field_value_bool("wrap", &meta, &mut field_params.wrap)? {
                    return Ok(());
                }

                if parse_field_value_bool("unwrap", &meta, &mut field_params.unwrap)? {
                    return Ok(());
                }

                if parse_field_value_bool("skip", &meta, &mut field_params.skip)? {
                    return Ok(());
                }

                if parse_field_value("map", &meta, &mut field_params.map)? {
                    return Ok(());
                }

                Err(meta.error("unknown field"))
            })?;
        }

        fields.insert(name, field_params);
    }

    Ok(fields)
}

/// Try parse value as `rename = "value"`,
/// `rename(into = "value")` or `rename(into(Path, "value"))`
fn parse_field_value<T>(
    name: &'static str,
    meta: &ParseNestedMeta<'_>,
    field_value: &mut FieldValue<T>,
) -> Result<bool>
where
    T: syn::parse::Parse + Clone,
{
    if !meta.path.is_ident(name) {
        return Ok(false);
    }

    if meta.input.peek(token::Paren) {
        meta.parse_nested_meta(|meta| {
            if let Some((path, v)) = parse_field_value_for("from", &meta)? {
                field_value.set_from(path, v);
                return Ok(());
            }

            if let Some((path, v)) = parse_field_value_for("into", &meta)? {
                field_value.set_into(path, v);
                return Ok(());
            }

            if let Some((path, v)) = parse_field_value_for::<T>("from_into", &meta)? {
                field_value.set_from(path.clone(), v.clone());
                field_value.set_into(path, v);
                return Ok(());
            }

            Err(meta.error("unknown field"))
        })?;
    } else {
        let value: T = meta.value()?.parse()?;
        field_value.common = Some(value);
    }

    Ok(true)
}

/// Try parse value as `from(Path, "value")` or `from = "value"`
fn parse_field_value_for<T: syn::parse::Parse>(
    name: &'static str,
    meta: &ParseNestedMeta<'_>,
) -> Result<Option<(Option<Path>, T)>> {
    if !meta.path.is_ident(name) {
        return Ok(None);
    }

    if meta.input.peek(token::Paren) {
        let content;
        parenthesized!(content in meta.input);

        let path: Path = content.parse()?;
        content.parse::<Token![,]>()?;
        let v: T = content.parse()?;
        return Ok(Some((Some(path), v)));
    }

    let v: T = meta.value()?.parse()?;
    Ok(Some((None, v)))
}

fn parse_field_value_bool(
    name: &'static str,
    meta: &ParseNestedMeta<'_>,
    field_value: &mut FieldValue<bool>,
) -> Result<bool> {
    if !meta.path.is_ident(name) {
        return Ok(false);
    }

    if meta.input.peek(token::Paren) {
        meta.parse_nested_meta(|meta| {
            let mut found = false;

            if let Some(path) = parse_field_value_for_bool("from", &meta)? {
                found = true;
                field_value.set_from(path, true);
            }

            if let Some(path) = parse_field_value_for_bool("into", &meta)? {
                found = true;
                field_value.set_into(path, true);
            }

            if !found {
                return Err(meta.error("unknown field"));
            }
            Ok(())
        })?;
    } else {
        field_value.common = Some(true);
    }

    Ok(true)
}

fn parse_field_value_for_bool(
    name: &'static str,
    meta: &ParseNestedMeta<'_>,
) -> Result<Option<Option<Path>>> {
    if !meta.path.is_ident(name)
        && !meta.path.is_ident("from_into")
        && !meta.path.is_ident("into_from")
    {
        return Ok(None);
    }

    if meta.input.peek(token::Paren) {
        let content;
        parenthesized!(content in meta.input);

        let path: Path = content.parse()?;
        return Ok(Some(Some(path)));
    }

    Ok(Some(None))
}

fn detect_type_for_serviced(ty: &syn::Type, recurse: u64) -> AdditionalType {
    let syn::Type::Path(syn::TypePath {
        path: Path { segments, .. },
        ..
    }) = &ty
    else {
        return AdditionalType::None;
    };

    match segments.iter().next() {
        Some(seg) if seg.ident == "Option" => match is_support_type_for_serviced(seg, recurse) {
            (true, AdditionalType::None) => return AdditionalType::Option,
            (true, AdditionalType::Vec) => return AdditionalType::OptionVec,
            _ => (),
        },
        Some(seg) if seg.ident == "Vec" => match is_support_type_for_serviced(seg, recurse) {
            (true, AdditionalType::None) => return AdditionalType::Vec,
            (true, AdditionalType::Option) => return AdditionalType::VecOption,
            _ => (),
        },
        _ => (),
    }

    AdditionalType::None
}

fn is_support_type_for_serviced(seg: &PathSegment, recurse: u64) -> (bool, AdditionalType) {
    if let syn::PathArguments::AngleBracketed(a) = &seg.arguments {
        let a_next = a.args.iter().next();
        if let Some(syn::GenericArgument::Type(ty)) = a_next {
            if recurse < 1 {
                return (true, detect_type_for_serviced(ty, 1));
            }
            return (true, AdditionalType::None);
        }
    };
    (false, AdditionalType::None)
}
