use proc_macro2::{Ident, Span};
// todo! remove uuid
use uuid::Uuid;
// use serde::{Deserialize, Serialize};
// use uuid::Uuid;

use crate::case::LowerSnakeIdent;

// pub mod db;
pub mod parse;

#[derive(Debug, Clone)]
pub struct _Primitive {
    pub ident: LowerSnakeIdent,
}

impl _Primitive {
    pub fn ident(&self) -> &Ident {
        &self.ident.0
    }

    pub fn span(&self) -> Span {
        self.ident().span()
    }

}
