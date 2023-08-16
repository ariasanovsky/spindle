use std::path::PathBuf;

use quote::ToTokens;
use serde::{Deserialize, Serialize};

use crate::{error::NaivelyTokenize, TokenResult, case::LowerSnakeIdent};

use in_out::InOut;

mod display;
pub(crate) mod in_out;
mod parse;
mod tokens;
#[cfg(test)]
mod test;

static MAP_PATH: &str = "target/spindle/map/";

#[derive(Clone)]
pub(crate) struct MapFn {
    pub(crate) item_fn: syn::ItemFn,
    pub(crate) in_outs: Vec<InOut>,
}

#[derive(Clone, Debug)]
pub(crate) struct MapAttrs {
    pub _tags: Vec<CrateTag>,
}

#[derive(Clone, Debug)]
pub(crate) struct CrateTag(pub LowerSnakeIdent);

impl PartialEq for MapFn {
    fn eq(&self, other: &Self) -> bool {
        // todo! feels hacky
        self.item_fn.to_token_stream().to_string()
        == other.item_fn.to_token_stream().to_string()
    }
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
