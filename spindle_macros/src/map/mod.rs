use std::path::PathBuf;

use proc_macro2::TokenStream;
use quote::ToTokens;
use serde::{Deserialize, Serialize};
use syn::parse_macro_input;

use crate::{MapAttrs, MapFn, TokenResult};

mod parse;

pub(crate) fn emit_slice_map_kernel(_attr: MapAttrs, item: MapFn) -> TokenResult {
    todo!()
}
