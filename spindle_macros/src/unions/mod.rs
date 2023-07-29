use proc_macro2::{Ident, Span};
use uuid::Uuid;

use crate::{primitives::_Primitive, case::{LowerSnakeIdent, UpperCamelIdent}};

pub mod db;
mod parse;

#[derive(Clone)]
pub struct _Union {
    uuid: String,   // Uuid,
    ident: UpperCamelIdent,
    fields: Vec<String>,    // Vec<Uuid>,
}

impl _Union {
    pub fn new(ident: Ident, fields: Vec<Ident>) -> Result<Self, crate::case::Error> {
        todo!()
    }

    pub fn ident(&self) -> &Ident {
        &self.ident.0
    }

    pub fn span(&self) -> Span {
        self.ident().span()
    }
}