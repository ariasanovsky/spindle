use proc_macro2::Span;
use quote::ToTokens;
use spindle_db::TypeDb;

use crate::tag::CrateTag;

use in_out::InOut;

use self::tokens::MapTokens;

mod display;
pub(crate) mod in_out;
mod parse;
#[cfg(test)]
mod test;
mod tokens;

const _DB_NAME: &str = "map";

#[derive(Clone)]
pub struct MapFn {
    pub item_fn: syn::ItemFn,
    pub in_outs: Vec<InOut>,
}

#[derive(Clone, Debug)]
pub struct MapAttrs {
    pub _tags: Vec<CrateTag>,
}

impl PartialEq for MapFn {
    fn eq(&self, other: &Self) -> bool {
        // todo! feels hacky
        self.item_fn.to_token_stream().to_string() == other.item_fn.to_token_stream().to_string()
    }
}

pub(crate) fn map(
    attrs: MapAttrs,
    map_fn: MapFn,
    db_name: &str,
) -> syn::Result<proc_macro2::TokenStream> {
    // add map to database
    // tag in database with #example_01
    // emit map & map trait
    let target = std::env::var("CARGO_TARGET_DIR").unwrap_or_else(|_| "target".to_string());
    let map_path = format!("{target}/spindle/db/");
    let db = TypeDb::open_or_create(db_name, map_path).unwrap();
    let _map = db.get_or_insert_map(&map_fn, &attrs._tags).unwrap();
    let map_trait = map_fn.map_trait();
    Ok(quote::quote_spanned! { Span::mixed_site() =>
        #map_fn
        #map_trait
    })
}
