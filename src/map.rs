use proc_macro2::{Delimiter, Group, Literal, Punct, Spacing, TokenStream, TokenTree};
use syn::{
    braced, bracketed, parenthesized,
    parse::{Parse, ParseStream},
    token, Expr, Ident, Token,
};

struct MapExpression(Expr);

pub(crate) fn parse_map_expr(field_name: Ident, input: Literal) -> syn::Result<Expr> {
    let expr_str = format!("{input}");
    let expr_str: String = expr_str.chars().skip(1).take(expr_str.len() - 2).collect();
    let map_expr = match syn::parse_str::<MapExpression>(&format!("{};{}", field_name, expr_str)) {
        Ok(expr) => expr,
        Err(err) => return Err(syn::Error::new(input.span(), err.to_string())),
    };
    Ok(map_expr.0)
}

impl Parse for MapExpression {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let field_name = input.parse()?;
        input.parse::<Token![;]>()?;
        let tt = parse_custom_fn_expr(field_name, input)?;
        let expr = syn::parse2(tt)?;
        Ok(Self(expr))
    }
}

fn parse_custom_fn_expr(field_name: Ident, input: ParseStream) -> syn::Result<TokenStream> {
    let mut begin_expr = true;
    let mut tokens = Vec::new();
    while !input.is_empty() {
        if begin_expr
            && input.peek(Token![.])
            && (input.peek2(syn::Ident) || input.peek2(Token![as]))
        {
            input.parse::<Token![.]>()?;
            tokens.push(TokenTree::Ident(Ident::new("this", field_name.span())));
            tokens.push(TokenTree::Punct(Punct::new('.', Spacing::Alone)));
            tokens.push(TokenTree::Ident(field_name.clone()));
            if !input.peek(Token![as]) {
                tokens.push(TokenTree::Punct(Punct::new('.', Spacing::Alone)));
            }
            begin_expr = false;
            continue;
        }

        begin_expr = input.peek(Token![break])
            || input.peek(Token![continue])
            || input.peek(Token![if])
            || input.peek(Token![in])
            || input.peek(Token![match])
            || input.peek(Token![mut])
            || input.peek(Token![return])
            || input.peek(Token![while])
            || input.peek(Token![+])
            || input.peek(Token![&])
            || input.peek(Token![!])
            || input.peek(Token![^])
            || input.peek(Token![,])
            || input.peek(Token![/])
            || input.peek(Token![=])
            || input.peek(Token![>])
            || input.peek(Token![<])
            || input.peek(Token![|])
            || input.peek(Token![%])
            || input.peek(Token![;])
            || input.peek(Token![*])
            || input.peek(Token![-]);

        let token: TokenTree = if input.peek(token::Paren) {
            let content;
            let delimiter = parenthesized!(content in input);
            let nested = parse_custom_fn_expr(field_name.clone(), &content)?;
            let mut group = Group::new(Delimiter::Parenthesis, nested);
            group.set_span(delimiter.span.join());
            TokenTree::Group(group)
        } else if input.peek(token::Brace) {
            let content;
            let delimiter = braced!(content in input);
            let nested = parse_custom_fn_expr(field_name.clone(), &content)?;
            let mut group = Group::new(Delimiter::Brace, nested);
            group.set_span(delimiter.span.join());
            TokenTree::Group(group)
        } else if input.peek(token::Bracket) {
            let content;
            let delimiter = bracketed!(content in input);
            let nested = parse_custom_fn_expr(field_name.clone(), &content)?;
            let mut group = Group::new(Delimiter::Bracket, nested);
            group.set_span(delimiter.span.join());
            TokenTree::Group(group)
        } else {
            input.parse()?
        };
        tokens.push(token);
    }
    Ok(TokenStream::from_iter(tokens))
}
