use proc_macro2::Span;
use quote::ToTokens;
use spindle_db::TypeDb;

use crate::tag::CrateTag;

use in_out::InOut;

use self::tokens::MapTokens;

mod display;
pub(crate) mod in_out;
mod parse;
mod tokens;
#[cfg(test)]
mod test;

const MAP_PATH: &str = "target/spindle/db/";
const DB_NAME: &str = "map";

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
        self.item_fn.to_token_stream().to_string()
        == other.item_fn.to_token_stream().to_string()
    }
}


pub(crate) fn map(attrs: MapAttrs, map_fn: MapFn, db_name: &str)
-> syn::Result<proc_macro2::TokenStream> {
    // add map to database
    // tag in database with #example_01
    // emit map & map trait
    let db = TypeDb::open_or_create(db_name, MAP_PATH).unwrap();
    let _map = db.get_or_insert_map(&map_fn, &attrs._tags).unwrap();
    let map_trait = map_fn.map_trait();
    Ok(quote::quote_spanned! { Span::mixed_site() =>
        #map_fn
        #map_trait
    })
}
