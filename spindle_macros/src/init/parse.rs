use syn::{parse::Parse, Token};

use crate::{tag::CrateTag, map::{MapFn, in_out::InOut}};

use super::{Attrs, InputInitFn};

impl Parse for Attrs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        // e.g., `#foo, #bar, #baz` -> `vec![foo, bar, baz]`
        dbg!(&input);
        let tag: CrateTag = input.parse()?;
        dbg!(&tag);
        dbg!(&input);
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
        dbg!(&tags);
        let attrs = Attrs { tags };
        dbg!(&attrs);
        Ok(attrs)
        // let err = input.error(format!("make Attrs from {input:#?}"));
        // Err(err)
    }
}

impl Parse for InputInitFn {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        // e.g., `fn foo(i: i32) { i as f64 }` -> f64`
        // exactly 1 primitive integer argument
        // exactly 1 primitive output
        dbg!(&input);
        // first parse as a MapFn
        let map_fn: MapFn = input.parse()?;
        dbg!(&map_fn);
        // then restrict the input to a single primitive integer
        // and the output to a single primitive
        let MapFn { item_fn, in_outs } = map_fn;
        match &in_outs[..] {
            [x] => {
                let InOut { input: _, output: _ } = &x;
                let input_type = x.input.as_ref().ok_or(input.error("expected exactly 1 input"))?;
                if !input_type.is_integer() {
                    return Err(input.error("expected exactly 1 primitive input"));
                }
                dbg!(&input_type);
                let output_type = x.output.as_ref().ok_or(input.error("expected exactly 1 output"))?;
                dbg!(&output_type);
                Ok(InputInitFn { item_fn, input_type: input_type.clone(), output_type: output_type.clone() })
            },
            _ => return Err(input.error("expected exactly 1 input and 1 output")),
        }
        // let err = input.error(format!("make InputInitFn from {input:#?}"));
        // Err(err)
    }
}
