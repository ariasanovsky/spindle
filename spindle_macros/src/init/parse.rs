use syn::{parse::Parse, Token};

use crate::{tag::CrateTag, dev_item_fn::DevItemFn};

use super::{Attrs, DevInitFn};

impl Parse for Attrs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        // e.g., `#foo, #bar, #baz` -> `vec![foo, bar, baz]`
        let tag: CrateTag = input.parse()?;
        let mut tags: Vec<CrateTag> = vec![tag];
        // we expect a comma-separated list of tags
        while !input.is_empty() {
            let _: Token![,] = input.parse()?;
            // allow one trailing comma
            if input.is_empty() {
                break;
            }
            let tag: CrateTag = input.parse()?;
            tags.push(tag);
        }
        let attrs = Attrs { tags };
        Ok(attrs)
        // let err = input.error(format!("make Attrs from {input:#?}"));
        // Err(err)
    }
}

impl Parse for DevInitFn {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let map_fn: DevItemFn = input.parse()?;
        map_fn.try_into()
    }
}
