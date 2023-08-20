use syn::parse::Parse;

use super::{Attrs, InputInitFn};

impl Parse for Attrs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        todo!("parse the attrs")
    }
}

impl Parse for InputInitFn {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        todo!("parse the input init fn")
    }
}
