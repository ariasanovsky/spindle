use std::fmt::Debug;

use quote::ToTokens;
use syn::ItemFn;

use crate::{tag::CrateTag, primitives::_Primitive};

mod parse;
#[cfg(test)]
mod test;
mod tokens;

#[derive(Debug, PartialEq)]
pub struct Attrs {
    pub tags: Vec<CrateTag>,
}

pub struct InputInitFn {
    pub item_fn: ItemFn,
    pub input_type: _Primitive,
    pub output_type: _Primitive,
}

impl Debug for InputInitFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("InputInitFn")
        .field("item_fn", &self.item_fn.to_token_stream().to_string())
        .field("input_type", &self.input_type)
        .field("output_type", &self.output_type)
        .finish()
    }
}

impl PartialEq for InputInitFn {
    fn eq(&self, other: &Self) -> bool {
        let Self { item_fn, input_type, output_type } = self;
        let Self { item_fn: other_item_fn, input_type: other_input_type, output_type: other_output_type } = other;
        item_fn.into_token_stream().to_string() == other_item_fn.into_token_stream().to_string()
        && input_type == other_input_type && output_type == other_output_type
    }
}

#[derive(Debug, PartialEq)]
pub struct OutputInitFn;

pub fn init(attrs: Attrs, init_map: InputInitFn) -> OutputInitFn {
    todo!()
}
