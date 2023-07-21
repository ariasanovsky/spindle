use quote::ToTokens;
use syn::{
    parse::{Parse, ParseStream},
    ItemFn, Result,
};
use proc_macro2::TokenStream;

use crate::{MapFn, BasicRangeAttrs};

static NO_ATTRIBUTES: &str = "no attributes";
static NO_GENERICS: &str = "no generics";
static NOT_A_METHOD: &str = "may not be a method";
static NO_WHERE_CLAUSE: &str = "no where clauses";
static EXACTLY_ONE_INPUT: &str = "exactly one (integer) input";
static ONLY_PRIMITIVE_INPUTS: &str = "only primitive number inputs (i32, usize, f32, etc.)";
static NO_RETURN: &str = "missing return type";
static ONLY_PRIMITIVE_RETURNS: &str =
    "only returns primitive numbers (i32, usize, f32, etc.)";

impl Parse for BasicRangeAttrs {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.is_empty() {
            Ok(Self)
        } else {
            Err(input.error(NO_ATTRIBUTES))
        }
    }
}
    
    
impl Parse for MapFn {
    fn parse(input: ParseStream) -> Result<Self> {
        let range_fn: ItemFn = input.parse()?;
        if !range_fn.attrs.is_empty() {
            return Err(input.error(NO_ATTRIBUTES));
        }
        if !range_fn.sig.generics.params.is_empty() {
            return Err(input.error(NO_GENERICS));
        }
        if range_fn.sig.generics.where_clause.is_some() {
            return Err(input.error(NO_WHERE_CLAUSE));
        }
        if range_fn.sig.inputs.is_empty() {
            return Err(input.error(EXACTLY_ONE_INPUT));
        }
        let mut inputs = range_fn.sig.inputs.iter();
        let arg = inputs.next();
        let arg = match (arg, inputs.next()) {
            (None, _) | (Some(_), Some(_)) => return Err(input.error(EXACTLY_ONE_INPUT)),
            (Some(arg), None) => arg,
        };
        let arg = match arg {
            syn::FnArg::Receiver(_) => return Err(input.error(NOT_A_METHOD)),
            syn::FnArg::Typed(arg) => arg,
        };
        if !arg.attrs.is_empty() {
            return Err(input.error(NO_ATTRIBUTES));
        }
        let int_type = match arg.ty.as_ref() {
            syn::Type::Path(path) => path,
            _ => return Err(input.error(ONLY_PRIMITIVE_INPUTS)),
        };
        if int_type.qself.is_some() {
            return Err(input.error(ONLY_PRIMITIVE_INPUTS));
        }
        let int_type = match int_type.path.segments.len() {
            1 => &int_type.path.segments[0],
            _ => return Err(input.error(ONLY_PRIMITIVE_INPUTS)),
        };
        if !int_type.arguments.is_empty() {
            return Err(input.error(ONLY_PRIMITIVE_INPUTS));
        }
        let int_type = int_type.ident.to_string();
        if ![
            "isize", "usize", "f32", "f64", "i8", "u8", "i16", "u16", "i32", "u32", "i64", "u64",
        ]
        .contains(&int_type.as_str())
        {
            return Err(input.error(ONLY_PRIMITIVE_INPUTS));
        }
        let output = match &range_fn.sig.output {
            syn::ReturnType::Default => return Err(input.error(NO_RETURN)),
            syn::ReturnType::Type(_, output) => *output.clone(),
        };
        let output = match output {
            syn::Type::Path(path) => path,
            _ => return Err(input.error(ONLY_PRIMITIVE_RETURNS)),
        };
        if output.qself.is_some() {
            return Err(input.error(ONLY_PRIMITIVE_RETURNS));
        }
        let output_type = match output.path.segments.len() {
            1 => &output.path.segments[0],
            _ => return Err(input.error(ONLY_PRIMITIVE_RETURNS)),
        };
        if !output_type.arguments.is_empty() {
            return Err(input.error(ONLY_PRIMITIVE_RETURNS));
        }
        let output_type = output_type.ident.to_string();
        if ![
            "isize", "usize", "f32", "f64", "i8", "u8", "i16", "u16", "i32", "u32", "i64", "u64",
        ]
        .contains(&output_type.as_str())
        {
            return Err(input.error(ONLY_PRIMITIVE_RETURNS));
        }

        Ok(Self(range_fn))
    }
}

impl ToTokens for MapFn {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.0.to_tokens(tokens);
    }
}
