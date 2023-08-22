use syn::parse::Parse;

use super::DevItemFn;

impl Parse for DevItemFn {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let item_fn: syn::ItemFn = input.parse()?;
        item_fn.try_into()
    }
}
