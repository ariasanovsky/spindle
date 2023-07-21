use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Ident, Token, parse::Parse, parse::ParseStream};

pub(crate) struct SpinInput {
    pub(crate) union_name: Ident,
    pub(crate) types: Vec<Ident>,
}

impl Parse for SpinInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let union_name: Ident = input.parse()?;
        input.parse::<Token![,]>()?;
        let types: syn::punctuated::Punctuated<Ident, Token![,]> = input.parse_terminated(Ident::parse, Token![,])?;
        Ok(SpinInput {
            union_name,
            types: types.into_iter().collect(),
        })
    }
}
// my favorite emojis are: 🦀 and also 💔
