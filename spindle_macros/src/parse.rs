use syn::{parse::{Parse, ParseStream}, ItemFn, Result};

use crate::{RangeAttributes, RangeFn};

static NO_ATTRIBUTES: &str = "attributes are not yet supported";
static NO_GENERICS: &str = "generic functions are not yet supported";
static NOT_A_METHOD: &str = "range functions are methods";
static NO_WHERE_CLAUSE: &str = "where clauses are not supported";
static EXACTLY_ONE_INPUT: &str = "range functions have exactly one integer input";
static ONLY_INTEGERS: &str = "range functions take integer types (isize, usize, i32, u32, etc.)";
static NO_RETURN: &str = "range functions have a return type";
static ONLY_PRIMITIVE_RETURNS: &str = "range functions currently return primitive number types (i32, usize, f32, etc.)";
static ONLY_I32: &str = "range functions currently only admit i32";

impl Parse for RangeAttributes {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.is_empty() {
            Ok(Self)
        } else {
            Err(input.error(NO_ATTRIBUTES))
        }
    }
}

impl Parse for RangeFn {
    fn parse(input: ParseStream) -> Result<Self> {
        let range_fn: ItemFn = input.parse()?;
        if !range_fn.attrs.is_empty() {
            return Err(input.error(NO_ATTRIBUTES));
        }
        if !range_fn.sig.generics.params.is_empty() {
            return Err(input.error(NO_GENERICS));
        }
        if !range_fn.sig.generics.where_clause.is_none() {
            return Err(input.error(NO_WHERE_CLAUSE));
        }
        if range_fn.sig.inputs.is_empty() {
            return Err(input.error(EXACTLY_ONE_INPUT));
        }
        let mut inputs = range_fn.sig.inputs.iter();
        let arg = inputs.next();
        let arg = match (arg, inputs.next()) {
            (None, _) | (Some(_), Some(_)) =>
                return Err(input.error(EXACTLY_ONE_INPUT)),
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
            _ => return Err(input.error(ONLY_INTEGERS)),
        };
        if int_type.qself.is_some() {
            return Err(input.error(ONLY_INTEGERS));
        }
        let int_type = match int_type.path.segments.len() {
            1 => &int_type.path.segments[0],
            _ => return Err(input.error(ONLY_INTEGERS)),
        };
        if !int_type.arguments.is_empty() {
            return Err(input.error(ONLY_INTEGERS));
        }
        let int_type = int_type.ident.to_string();
        if !["isize", "usize", "i8", "u8", "i16", "u16", "i32", "u32", "i64", "u64"].contains(&int_type.as_str()) {
            return Err(input.error(ONLY_INTEGERS));
        }
        if int_type.ne("i32") {
            return Err(input.error(ONLY_I32));
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
        if !["isize", "usize", "f32", "f64", "i8", "u8", "i16", "u16", "i32", "u32", "i64", "u64"].contains(&output_type.as_str()) {
            return Err(input.error(ONLY_PRIMITIVE_RETURNS));
        }

        Ok(Self(range_fn))
    }
}
