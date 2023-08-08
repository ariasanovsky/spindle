use proc_macro2::{Ident, TokenStream};
use quote::ToTokens;
use syn::{
    parse::{Parse, ParseStream},
    ItemFn, PatType, Result, Signature,
};

use crate::{
    case::LowerSnakeIdent,
    map::in_out::InOut,
    primitives::_Primitive,
    regulate::{
        item_fn::RegulateItemFn, pat_type::RegulatePatTypes, return_type::RegulateReturnType,
        signature::RegulateSignature, EXPECTED_INPUT_ONE, EXPECTED_ONE_INPUT_PRIMITIVE,
        EXPECTED_RETURN_PRIMITIVE, UNEXPECTED_ATTRIBUTES,
    },
    MapAttrs,
};

use super::MapFn;

impl Parse for MapAttrs {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.is_empty() {
            Ok(Self)
        } else {
            Err(input.error(UNEXPECTED_ATTRIBUTES))
        }
    }
}

// todo! bleh, custom error types? better spans?
impl MapFn {
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

impl Parse for MapFn {
    fn parse(input: ParseStream) -> Result<Self> {
        let item_fn: ItemFn = input.parse()?;
        let ItemFn {
            attrs: _attrs,
            vis: _vis,
            sig,
            block: _block,
        } = Self::_parse_item_fn(item_fn.clone()).map_err(|e| input.error(e))?;

        let (input_pat_types, return_ident) =
            Self::_parse_signature(sig).map_err(|e| input.error(e))?;

        // todo! only one input -- see roadmap...
        let input_ident = Self::_parse_pat_types(input_pat_types).map_err(|e| input.error(e))?;

        const PRIMITIVES: &[&str] = &[
            "bool", "isize", "usize", "f32", "f64", "i8", "u8", "i16", "u16", "i32", "u32", "i64",
            "u64",
        ];

        if !PRIMITIVES.contains(&input_ident.to_string().as_str()) {
            return Err(input.error(EXPECTED_ONE_INPUT_PRIMITIVE));
        }

        if !PRIMITIVES.contains(&return_ident.to_string().as_str()) {
            return Err(input.error(EXPECTED_RETURN_PRIMITIVE));
        }

        // todo! hacks abound, ugh
        let input = _Primitive {
            ident: LowerSnakeIdent(input_ident),
        };
        let output = _Primitive {
            ident: LowerSnakeIdent(return_ident),
        };

        // let input_ident = Self::_parse_ident_as_primitive(input_idents)
        //     .map_err(|e| input.error(e))?;
        // let return_ident = Self::_parse_ident_as_primitive(return_ident)
        //     .map_err(|e| input.error(e))?;
        Ok(Self {
            item_fn,
            in_outs: vec![InOut {
                input: Some(input),
                output: Some(output),
            }],
        })
        // todo!()
    }
}

impl ToTokens for MapFn {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.item_fn.to_tokens(tokens);
    }
}

// finally, we assert that the input & return types are primitives
// todo!()
// if !map_fn.attrs.is_empty() {
//     return Err(input.error(UNEXPECTED_ATTRIBUTES));
// }
// if !map_fn.sig.generics.params.is_empty() {
//     return Err(input.error(UNEXPECTED_GENERICS));
// }
// if map_fn.sig.generics.where_clause.is_some() {
//     return Err(input.error(UNEXPECTED_WHERE_CLAUSE));
// }
// if map_fn.sig.inputs.is_empty() {
//     return Err(input.error(EXPECTED_INPUT_ONE));
// }
// let mut inputs = map_fn.sig.inputs.iter();
// let arg = inputs.next();
// let arg = match (arg, inputs.next()) {
//     (None, _) | (Some(_), Some(_)) => return Err(input.error(EXPECTED_INPUT_ONE)),
//     (Some(arg), None) => arg,
// };
// let arg = match arg {
//     syn::FnArg::Receiver(_) => return Err(input.error(UNEXPECTED_SELF)),
//     syn::FnArg::Typed(arg) => arg,
// };
// if !arg.attrs.is_empty() {
//     return Err(input.error(UNEXPECTED_ATTRIBUTES));
// }
// let int_type = match arg.ty.as_ref() {
//     syn::Type::Path(path) => path,
//     _ => return Err(input.error(EXPECTED_ONE_INPUT_PRIMITIVE)),
// };
// if int_type.qself.is_some() {
//     return Err(input.error(EXPECTED_ONE_INPUT_PRIMITIVE));
// }
// let int_type = match int_type.path.segments.len() {
//     1 => &int_type.path.segments[0],
//     _ => return Err(input.error(EXPECTED_ONE_INPUT_PRIMITIVE)),
// };
// if !int_type.arguments.is_empty() {
//     return Err(input.error(EXPECTED_ONE_INPUT_PRIMITIVE));
// }
// let int_type = int_type.ident.to_string();
// if ![
//     "isize", "usize", "f32", "f64", "i8", "u8", "i16", "u16", "i32", "u32", "i64", "u64",
// ]
// .contains(&int_type.as_str())
// {
//     return Err(input.error(EXPECTED_ONE_INPUT_PRIMITIVE));
// }
// let output = match &map_fn.sig.output {
//     syn::ReturnType::Default => return Err(input.error(ONLY_PRIMITIVE_INPUTS)),
//     syn::ReturnType::Type(_, output) => *output.clone(),
// };
// let output = match output {
//     syn::Type::Path(path) => path,
//     _ => return Err(input.error(ONLY_PRIMITIVE_RETURNS)),
// };
// if output.qself.is_some() {
//     return Err(input.error(ONLY_PRIMITIVE_RETURNS));
// }
// let output_type = match output.path.segments.len() {
//     1 => &output.path.segments[0],
//     _ => return Err(input.error(ONLY_PRIMITIVE_RETURNS)),
// };
// if !output_type.arguments.is_empty() {
//     return Err(input.error(ONLY_PRIMITIVE_RETURNS));
// }
// let output_type = output_type.ident.to_string();
// if ![
//     "isize", "usize", "f32", "f64", "i8", "u8", "i16", "u16", "i32", "u32", "i64", "u64",
// ]
// .contains(&output_type.as_str())
// {
//     return Err(input.error(ONLY_PRIMITIVE_RETURNS));
// }

// Ok(Self(map_fn))
