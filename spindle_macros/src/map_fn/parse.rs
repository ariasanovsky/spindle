use proc_macro2::{Ident, TokenStream};
use quote::ToTokens;
use syn::{
    parse::{Parse, ParseStream},
    ItemFn, PatType, Result, Signature,
};

use crate::{
    case::{LowerSnakeIdent, PrimitiveIdent},
    map_fn::in_out::InOut,
    regulate::{
        item_fn::RegulateItemFn, pat_type::RegulatePatTypes, return_type::RegulateReturnType,
        signature::RegulateSignature, EXPECTED_INPUT_ONE, EXPECTED_ONE_INPUT_PRIMITIVE,
        EXPECTED_RETURN_PRIMITIVE,
    }, dev_item_fn::DevItemFn,
};

use super::{CrateTag, MapAttrs, DevMapFn};

impl Parse for CrateTag {
    fn parse(input: ParseStream) -> Result<Self> {
        // # followed by a lower_snake_ident
        let _ = input.parse::<syn::Token![#]>()?;
        let ident: LowerSnakeIdent = input.parse()?;
        Ok(Self(ident))
    }
}

impl Parse for MapAttrs {
    fn parse(input: ParseStream) -> Result<Self> {
        // supported attrs:
        // - tags: `#example` -> CrateTag(LowerSnakeIdent)
        // for now, attrs are separated by commas
        // 0 attrs is allowed

        // recursively look for the # token followed by a lower snake ident
        let mut tags = Vec::new();
        if input.is_empty() {
            return Ok(Self { tags });
        }
        loop {
            let _ = input.parse::<syn::Token![#]>()?;
            let ident: LowerSnakeIdent = input.parse()?;
            tags.push(CrateTag(ident));
            // if the input is noempty, we expect a comma
            if !input.is_empty() {
                let _ = input.parse::<syn::Token![,]>()?;
            } else {
                break;
            }
        }
        Ok(Self { tags })
    }
}

// todo! bleh, custom error types? better spans?
impl DevMapFn {
    fn _parse_item_fn(item_fn: ItemFn) -> std::result::Result<ItemFn, &'static str> {
        item_fn.no_attributes()?.no_generics()?.no_where_clause()
    }

    fn _parse_signature(
        sig: Signature,
    ) -> std::result::Result<(Vec<PatType>, Ident), &'static str> {
        let sig = sig
            .no_const()?
            .no_async()?
            .no_abi()?
            .no_generics()?
            .no_variadic()?;
        let typed_inputs: Vec<_> = sig
            .only_typed_inputs()
            .map(|inputs| inputs.into_iter().cloned().collect())?;
        let return_type = sig.output.ident_return()?;
        Ok((typed_inputs, return_type))
    }

    fn _parse_pat_types(pat_types: Vec<PatType>) -> std::result::Result<Ident, &'static str> {
        pat_types
            .only_ident_inputs()?
            .first()
            .cloned()
            .ok_or(EXPECTED_INPUT_ONE)
    }
}

impl Parse for DevMapFn {
    fn parse(input: ParseStream) -> Result<Self> {
        let item_fn: DevItemFn = input.parse()?;
        todo!("MapFn::parse")
        // let item_fn: ItemFn = input.parse()?;
        // let ItemFn {
        //     attrs: _attrs,
        //     vis: _vis,
        //     sig,
        //     block: _block,
        // } = Self::_parse_item_fn(item_fn.clone()).map_err(|e| input.error(e))?;

        // let (input_pat_types, return_ident) =
        //     Self::_parse_signature(sig).map_err(|e| input.error(e))?;

        // // todo! only one input -- see roadmap...
        // let input_ident = Self::_parse_pat_types(input_pat_types).map_err(|e| input.error(e))?;

        // const PRIMITIVES: &[&str] = &[
        //     "bool", "isize", "usize", "f32", "f64", "i8", "u8", "i16", "u16", "i32", "u32", "i64",
        //     "u64",
        // ];

        // if !PRIMITIVES.contains(&input_ident.to_string().as_str()) {
        //     return Err(input.error(EXPECTED_ONE_INPUT_PRIMITIVE));
        // }

        // if !PRIMITIVES.contains(&return_ident.to_string().as_str()) {
        //     return Err(input.error(EXPECTED_RETURN_PRIMITIVE));
        // }

        // // todo! hacks abound, ugh
        // let input = PrimitiveIdent(input_ident);
        // let output = PrimitiveIdent(return_ident);

        // // let input_ident = Self::_parse_ident_as_primitive(input_idents)
        // //     .map_err(|e| input.error(e))?;
        // // let return_ident = Self::_parse_ident_as_primitive(return_ident)
        // //     .map_err(|e| input.error(e))?;
        // Ok(Self {
        //     item_fn,
        //     in_outs: vec![InOut {
        //         input: Some(input),
        //         output: Some(output),
        //     }],
        // })
    }
}

impl ToTokens for DevMapFn {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { vis, sig, block } = self;
        vis.to_tokens(tokens);
        sig.to_tokens(tokens);
        block.to_tokens(tokens);
    }
}
