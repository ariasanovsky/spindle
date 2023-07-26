use proc_macro2::Ident;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone)]
pub struct Field {
    id: Uuid,
    ident: Ident,    
}

#[derive(Serialize, Deserialize)]
struct MockField {
    id: Uuid,
    ident: String,
}

impl Serialize for Field {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        let mock_field = MockField {
            id: self.id,
            ident: self.ident.to_string(),
        };
        mock_field.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Field {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de> {
        let mock_field = MockField::deserialize(deserializer)?;
        Ok(Field {
            id: mock_field.id,
            ident: syn::parse_str(&mock_field.ident).map_err(serde::de::Error::custom)?,
        })
    }
}
