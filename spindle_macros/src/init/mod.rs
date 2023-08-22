use std::fmt::Debug;

use crate::{tag::CrateTag, dev_item_fn::{DevFnIdent, DevFnArg, DevReturnType}};

mod db;
mod parse;
#[cfg(test)]
mod test;
mod tokens;
mod try_from;

#[derive(Debug, PartialEq)]
pub struct Attrs {
    pub tags: Vec<CrateTag>,
}

pub struct DevInitFn {
    pub vis: syn::Visibility,
    pub sig: DevInitSignature,
    pub block: syn::Block,
}

pub struct DevInitSignature {
    // pub constness: Option<Const>,
    // pub asyncness: Option<Async>,
    // pub unsafety: Option<Unsafe>,
    // pub abi: Option<Abi>,
    pub fn_token: syn::Token![fn],
    pub ident: DevFnIdent,
    // pub generics: Generics,
    pub paren_token: syn::token::Paren,
    pub input: DevFnArg,
    pub comma: Option<syn::token::Comma>,
    // pub variadic: Option<Variadic>,
    pub output: DevReturnType,
}

impl Debug for DevInitFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
        // f.debug_struct("InputInitFn")
        // .field("item_fn", &self.item_fn.to_token_stream().to_string())
        // .field("input_type", &self.input_type)
        // .field("output_type", &self.output_type)
        // .finish()
    }
}

impl PartialEq for DevInitFn {
    fn eq(&self, other: &Self) -> bool {
        todo!()
        // let Self { item_fn, input_type, output_type } = self;
        // let Self { item_fn: other_item_fn, input_type: other_input_type, output_type: other_output_type } = other;
        // item_fn.into_token_stream().to_string() == other_item_fn.into_token_stream().to_string()
        // && input_type == other_input_type && output_type == other_output_type
    }
}

#[derive(Debug, PartialEq)]
pub struct OutputInitFn;

pub fn init(attrs: Attrs, init_map: DevInitFn) -> OutputInitFn {
    todo!()
}
