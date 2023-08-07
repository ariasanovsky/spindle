use std::path::PathBuf;

use quote::ToTokens;
use serde::{Deserialize, Serialize};

use crate::{error::NaivelyTokenize, MapAttrs, TokenResult};

use in_out::InOut;

mod display;
mod in_out;
mod parse;
#[cfg(test)]
mod test;

static MAP_PATH: &str = "target/spindle/map/";

#[derive(Clone)]
pub(crate) struct MapFn {
    pub(crate) item_fn: syn::ItemFn,
    pub(crate) in_outs: Vec<InOut>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct MapFnStrings(pub(crate) String, pub(crate) String);

pub(crate) fn serialize_map(_attr: MapAttrs, item: MapFn) -> TokenResult {
    let map_dir = PathBuf::from(MAP_PATH);
    std::fs::create_dir_all(&map_dir).map_err(NaivelyTokenize::naively_tokenize)?;
    let map_fn_strings = MapFnStrings("".into(), item.item_fn.to_token_stream().to_string());
    let map_fn_strings =
        serde_json::to_string(&map_fn_strings).map_err(NaivelyTokenize::naively_tokenize)?;
    let map_path = map_dir
        .join(item.item_fn.sig.ident.to_string())
        .with_extension("json");
    std::fs::write(map_path, map_fn_strings).map_err(NaivelyTokenize::naively_tokenize)?;
    Ok(quote::quote! {
        // #_attr todo! traits for MapAttrs (currently requiried to be empty)
        #item
    })
}
