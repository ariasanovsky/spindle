use proc_macro2::Ident;
use syn::{parse::Parse, punctuated::Punctuated, Token};

use crate::primitives::Primitive;

use super::_Union;

impl Parse for _Union {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let union_initializer = input.parse::<UnionInitializer>()?;
        let id = uuid::Uuid::new_v4();
        
        todo!()
    }
}

struct UnionInitializer(Ident, Option<Vec<Ident>>);

impl Parse for UnionInitializer {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        // `U = X | Y | Z` => (U, Some([X, Y, Z]))
        // `U` => (U, None)
        let ident = input.parse::<Ident>()?;
        let fields = if input.peek(syn::Token![=]) {
            input.parse::<syn::Token![=]>()?;
            let fields = Punctuated::<Ident, Token![|]>::parse_terminated(input)?;
            Some(fields.into_iter().collect())
        } else {
            None
        };
        Ok(UnionInitializer(ident, fields))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::{parse_quote, Ident};

    #[test]
    fn test_union_initializer() {
        let input: proc_macro2::TokenStream = "U = X | Y | Z".parse().unwrap();
        let parsed: UnionInitializer = syn::parse2(input).unwrap();

        assert_eq!(&parsed.0, &Ident::new("U", proc_macro2::Span::call_site()));
        assert_eq!(
            parsed.1,
            Some(vec![
                Ident::new("X", proc_macro2::Span::call_site()),
                Ident::new("Y", proc_macro2::Span::call_site()),
                Ident::new("Z", proc_macro2::Span::call_site()),
            ])
        );
        // assert_eq!(&parsed.1.unwrap()[1], &Ident::new("Y", proc_macro2::Span::call_site()));
    }

    #[test]
    fn test_union_initializer_no_fields() {
        let input: proc_macro2::TokenStream = "V".parse().unwrap();
        let parsed: UnionInitializer = syn::parse2(input).unwrap();

        assert_eq!(parsed.0, Ident::new("V", proc_macro2::Span::call_site()));
        assert!(parsed.1.is_none());
    }
}