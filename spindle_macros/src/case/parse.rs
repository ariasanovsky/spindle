use syn::parse::Parse;

use crate::camel_word;

use super::{LowerSnakeIdent, PrimitiveIdent, UpperCamelIdent};

impl Parse for UpperCamelIdent {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        // first we parse as an `Ident`
        let ident: syn::Ident = input.parse()?;
        // then we check that it's UpperCamelCase
        let s: String = ident.to_string();
        let camel = camel_word(&s);
        if s != camel {
            return Err(syn::Error::new_spanned(
                ident,
                format!("expected UpperCamelCase, found {s}"),
            ));
        }
        Ok(UpperCamelIdent(ident))
    }
}

impl Parse for PrimitiveIdent {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        // first we parse as an `Ident`
        let ident: syn::Ident = input.parse()?;
        // then we check that it's lower_snake_case
        let s: String = ident.to_string();
        const PRIMITIVES: &[&str] = &[
            "bool", "f32", "f64", "i8", "i16", "i32", "i64", "i128", "str", "u8", "u16", "u32",
            "u64", "u128", // "usize" , "isize", "char",
        ];
        if !PRIMITIVES.contains(&s.as_str()) {
            return Err(syn::Error::new_spanned(
                ident,
                format!("expected one of {PRIMITIVES:?}, found {s}"),
            ));
        }
        Ok(PrimitiveIdent(ident))
    }
}

impl Parse for LowerSnakeIdent {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        // first we parse as an `Ident`
        let ident: syn::Ident = input.parse()?;
        // then we check that it's lower_snake_case
        let s: String = ident.to_string();
        let snake = s.to_lowercase();
        if s != snake {
            return Err(syn::Error::new_spanned(
                ident,
                format!("expected lower_snake_case, found {s}"),
            ));
        }
        Ok(LowerSnakeIdent(ident))
    }
}
