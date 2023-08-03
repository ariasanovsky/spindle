use proc_macro2::{Ident, Span};
use uuid::Uuid;
// use serde::{Deserialize, Serialize};
// use uuid::Uuid;

use crate::case::LowerSnakeIdent;

// pub mod db;
pub mod parse;

#[derive(Debug, Clone)]
pub struct _Primitive {
    pub uuid: String,   // Uuid,
    pub ident: LowerSnakeIdent,
}

impl _Primitive {
    pub fn ident(&self) -> &Ident {
        &self.ident.0
    }

    pub fn span(&self) -> Span {
        self.ident().span()
    }

    pub fn uuid(&self) -> &str {
        &self.uuid
    }
}
