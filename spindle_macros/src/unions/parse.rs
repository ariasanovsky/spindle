use proc_macro2::Ident;
use syn::{parse::Parse, punctuated::Punctuated, Token};

use crate::{primitives::_Primitive, case::{UpperCamelIdent, LowerSnakeIdent}};

use super::_Union;

impl Parse for _Union {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        // the first token is an UpperCamelIdent
        let ident = input.parse::<Ident>()?;
        let upper_ident: UpperCamelIdent = ident.try_into().map_err(|_| {
            syn::Error::new(input.span(), "expected UpperCamelCase")
        })?;
        // there are 2 cases: `U = X | Y | ...` or `U`
        // the next token is either `=` or `,`
        let uuids = if input.peek(Token![=]) {
            // `U = X | Y | ...`
            // `U` may or may not be in the database
            // consume the `=`
            input.parse::<Token![=]>()?;
            // the next tokens alternate between field and `|`
            let fields = Punctuated::<_Primitive, Token![|]>::parse_terminated(input)?;
            // since we parse them as a field, we just made sure they are in the database
            let uuids: Vec<String> = fields.into_iter().map(|field| field.uuid).collect();
            // then, update the database
            Some(uuids)
        } else if input.peek(Token![,]) {
            // `U`
            // `U` is already in the database
            // consume the `,`
            input.parse::<Token![,]>()?;
            None
        } else {
            // unexpected token
            return Err(syn::Error::new(input.span(), "expected `=` or `,`"))
        };
        todo!("f (?deprecated)")
        // let db = crate::db::TypeDb::connect().map_err(|err| {
        //     syn::Error::new(input.span(), err)
        // })?;
        // db.add_union_if_not_exists(upper_ident, uuids)
        // .map_err(|err| {
        //     syn::Error::new(input.span(), err)
        // })
    }
}

// struct UnionInitializer(Ident, Option<Vec<Ident>>);

// impl Parse for UnionInitializer {
//     fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
//         // `U = X | Y | Z` => (U, Some([X, Y, Z]))
//         // `U` => (U, None)
//         let ident = input.parse::<Ident>()?;
//         let fields = if input.peek(syn::Token![=]) {
//             input.parse::<syn::Token![=]>()?;
//             let fields = Punctuated::<Ident, Token![|]>::parse_terminated(input)?;
//             Some(fields.into_iter().collect())
//         } else {
//             None
//         };
//         Ok(UnionInitializer(ident, fields))
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use syn::{parse_quote, Ident};

//     #[test]
//     fn test_union_initializer() {
//         let input: proc_macro2::TokenStream = "U = X | Y | Z".parse().unwrap();
//         let parsed: UnionInitializer = syn::parse2(input).unwrap();

//         assert_eq!(&parsed.0, &Ident::new("U", proc_macro2::Span::call_site()));
//         assert_eq!(
//             parsed.1,
//             Some(vec![
//                 Ident::new("X", proc_macro2::Span::call_site()),
//                 Ident::new("Y", proc_macro2::Span::call_site()),
//                 Ident::new("Z", proc_macro2::Span::call_site()),
//             ])
//         );
//         // assert_eq!(&parsed.1.unwrap()[1], &Ident::new("Y", proc_macro2::Span::call_site()));
//     }

//     #[test]
//     fn test_union_initializer_no_fields() {
//         let input: proc_macro2::TokenStream = "V".parse().unwrap();
//         let parsed: UnionInitializer = syn::parse2(input).unwrap();

//         assert_eq!(parsed.0, Ident::new("V", proc_macro2::Span::call_site()));
//         assert!(parsed.1.is_none());
//     }
// }