use proc_macro2::Ident;
// use serde::{Deserialize, Serialize};
// use uuid::Uuid;

use crate::case::LowerSnakeIdent;

pub mod db;
pub mod parse;

#[derive(Debug, Clone)]
pub struct Primitive {
    // id: Uuid,
    kind: FieldKind,    
}

impl Primitive {
    pub fn new_primitive(ident: LowerSnakeIdent) -> Self {
        Self {
            // id: Uuid::new_v4(),
            kind: FieldKind::Primitive(ident),
        }
    }

    fn ident(&self) -> &Ident {
        match &self.kind {
            FieldKind::Primitive(ident) => &ident.0,
        }
    }
}

#[derive(Debug, Clone)]
enum FieldKind {
    Primitive(LowerSnakeIdent),
}

// #[derive(Serialize, Deserialize)]
struct MockField {
    // id: Uuid,
    ident: String,
}

// impl Serialize for Field {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where S: serde::Serializer {
//         let mock_field = MockField {
//             // id: self.id,
//             ident: self.ident().to_string(),
//         };
//         mock_field.serialize(serializer)
//     }
// }

// impl<'de> Deserialize<'de> for Field {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where D: serde::Deserializer<'de> {
//         let mock_field = MockField::deserialize(deserializer)?;
//         let ident = Ident::new(&mock_field.ident, proc_macro2::Span::call_site());
//         // todo! assumes primitive
//         let ident = LowerSnakeIdent(ident);
//         Ok(Field {
//             // id: mock_field.id,
//             kind: FieldKind::Primitive(ident),
//         })
//     }
// }
