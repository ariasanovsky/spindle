use proc_macro2::Ident;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::fields::Field;

#[derive(Clone)]
pub struct _Union {
    id: Uuid,
    ident: Ident,
    fields: Vec<Field>,
}

#[derive(Serialize, Deserialize)]
struct MockUnion {
    id: Uuid,
    ident: String,
    fields: Vec<Field>,
}

impl Serialize for _Union {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        let mock_union = MockUnion {
            id: self.id,
            ident: self.ident.to_string(),
            fields: self.fields.clone(),
        };
        mock_union.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for _Union {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de> {
        let mock_union = MockUnion::deserialize(deserializer)?;
        Ok(_Union {
            id: mock_union.id,
            ident: syn::parse_str(&mock_union.ident).map_err(serde::de::Error::custom)?,
            fields: mock_union.fields,
        })
    }
}
