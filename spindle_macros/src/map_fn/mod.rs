use proc_macro2::Span;
use quote::ToTokens;
use spindle_db::TypeDb;

use crate::{tag::CrateTag, dev_item_fn::{DevSignature, DevFnIdent}};

use in_out::InOut;

use self::tokens::MapTokens;

mod db;
#[cfg(test)]
mod display;
pub(crate) mod in_out;
mod parse;
#[cfg(test)]
mod test;
mod tokens;

const _DB_NAME: &str = "map";

pub struct DevMapFn {
    pub vis: syn::Visibility,
    pub sig: DevSignature,
    pub block: syn::Block,
}

impl DevMapFn {
    pub fn ident(&self) -> &syn::Ident {
        let Self { vis: _, sig, block: _ } = self;
        let DevSignature { fn_token: _, ident, paren_token: _, inputs: _, output: _ } = sig;
        let DevFnIdent(ident) = ident;
        ident
    }
}

#[derive(Clone, Debug)]
pub struct MapAttrs {
    pub tags: Vec<CrateTag>,
}

impl PartialEq for DevMapFn {
    fn eq(&self, other: &Self) -> bool {
        // todo! feels hacky
        // self.item_fn.to_token_stream().to_string() == other.item_fn.to_token_stream().to_string()
        todo!("MapFn::eq")
    }
}

pub(crate) fn map(
    attrs: MapAttrs,
    map_fn: DevMapFn,
    db_name: &str,
) -> syn::Result<proc_macro2::TokenStream> {
    let MapAttrs { tags } = attrs;
    // todo! unwraps
    let db = TypeDb::open_or_create_default().unwrap();
    let _db_map = db.get_or_insert_item_fn(&map_fn, &tags).unwrap();
    let map_trait = map_fn.map_trait();
    todo!("map::map: write crate, compile");
    Ok(quote::quote_spanned! { Span::mixed_site() =>
        #map_fn
        #map_trait
    })
}
