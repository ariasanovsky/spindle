use quote::ToTokens;

use super::{DevFnIdent, DevFnArg, DevArgType, DevType, DevItemFn, DevSignature};

impl ToTokens for DevItemFn {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let DevItemFn { vis, sig, block } = self;
        vis.to_tokens(tokens);
        sig.to_tokens(tokens);
        block.to_tokens(tokens);
    }
}

impl ToTokens for DevSignature {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let DevSignature { fn_token, ident, paren_token, inputs, output } = self;
        fn_token.to_tokens(tokens);
        ident.to_tokens(tokens);
        paren_token.surround(tokens, |tokens| {
            inputs.to_tokens(tokens);
        });
        output.to_tokens(tokens);
    }
}

impl ToTokens for DevFnIdent {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let DevFnIdent(ident) = self;
        ident.to_tokens(tokens);
    }
}

impl ToTokens for DevFnArg {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let DevFnArg { pat, colon_token, ty } = self;
        pat.to_tokens(tokens);
        colon_token.to_tokens(tokens);
        ty.to_tokens(tokens);
    }
}

impl ToTokens for DevArgType {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            Self::DeviceType(ty) => ty.to_tokens(tokens),
            Self::SpindleAny(ident) => ident.to_tokens(tokens),
            Self::SpindleAnyFullPath(path) => path.to_tokens(tokens),
        }
    }
}

impl ToTokens for DevType {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        use DevType::*;
        match self {
            Float(ident) | Bool(ident) | SizedInteger(ident)
                => ident.to_tokens(tokens),
        }
    }
}