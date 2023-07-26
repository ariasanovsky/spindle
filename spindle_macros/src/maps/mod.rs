use proc_macro2::Ident;
use quote::ToTokens;
use serde::{Deserialize, Serialize};
use syn::ItemFn;
use uuid::Uuid;

use self::attrs::MapAttrs;

pub mod attrs;
pub mod in_out;

#[derive(Clone)]
pub struct Map {
    id: Uuid,
    ident: Ident,
    inout_pairs: Vec<(Uuid, Uuid)>,
    attrs: MapAttrs,
    item: ItemFn,
}

#[derive(Serialize, Deserialize)]
    struct MockMap {
        id: Uuid,
        ident: String,
        inout_pairs: Vec<(Uuid, Uuid)>,
        attrs: MapAttrs,
        item: String,
    }

impl Serialize for Map {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        let mock_map = MockMap {
            id: self.id,
            ident: self.ident.to_string(),
            inout_pairs: self.inout_pairs.clone(),
            attrs: self.attrs.clone(),
            item: format!("{}", self.item.to_token_stream()),
        };
        mock_map.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Map {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de> {
        let mock_map = MockMap::deserialize(deserializer)?;
        let item = syn::parse_str(&mock_map.item).map_err(serde::de::Error::custom)?;
        Ok(Map {
            id: mock_map.id,
            ident: syn::parse_str(&mock_map.ident).map_err(serde::de::Error::custom)?,
            inout_pairs: mock_map.inout_pairs,
            attrs: mock_map.attrs,
            item,
        })
    }
}
