use proc_macro2::Span;
use quote::ToTokens;

use crate::dev_item_fn::{DevReturnType, DevTypeTuple};

use super::{DevInitFn, DevInitSignature};

impl ToTokens for DevInitFn {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Self { vis, sig, block } = self;
        tokens.extend(quote::quote! {
            #vis #sig #block
        });
    }
}

impl ToTokens for DevInitSignature {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Self { fn_token, ident, paren_token, input, comma, output } = self;
        tokens.extend(quote::quote_spanned! { Span::mixed_site() =>
            #fn_token #ident
        });
        paren_token.surround(tokens, |tokens| {
            input.to_tokens(tokens);
            comma.to_tokens(tokens);
        });
        output.to_tokens(tokens);
    }
}

impl ToTokens for DevReturnType {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            Self::Default => {},
            Self::Type(arrow, ty) => {
                arrow.to_tokens(tokens);
                ty.to_tokens(tokens);
            },
            Self::Tuple(arrow, tuple) => {
                arrow.to_tokens(tokens);
                tuple.to_tokens(tokens);
            }
        }
    }
}

impl ToTokens for DevTypeTuple {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        todo!("asdfasf")
    }
}
