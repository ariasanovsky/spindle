use proc_macro2::Ident;

use crate::case::{UpperCamelIdent, PrimitiveIdent, LowerSnakeIdent};

mod tokens;
#[cfg(test)]
mod test;

#[derive(Debug)]
pub(crate) enum RawSpinInput {
    UnionInScope(UnionInScope),
    NewUnion(NewUnion),
    MapFnInScope(MapFnInScope),
}

#[derive(Debug)]
pub(crate) struct UnionInScope(pub UpperCamelIdent);

#[derive(Debug)]
pub(crate) struct NewUnion(pub UpperCamelIdent, pub Vec<PrimitiveIdent>);

#[derive(Debug)]
pub(crate) struct MapFnInScope(pub LowerSnakeIdent);

impl RawSpinInput {
    pub(crate) fn ident(&self) -> &Ident {
        match self {
            Self::UnionInScope(ident) => &ident.0.0,
            Self::NewUnion(ident) => &ident.0.0,
            Self::MapFnInScope(ident) => &ident.0.0,
        }
    }

    pub(crate) fn new_union(&self) -> Option<&NewUnion> {
        match self {
            Self::NewUnion(new_union) => Some(new_union),
            _ => None,
        }
    }

    pub(crate) fn union_in_scope(&self) -> Option<&UnionInScope> {
        match self {
            Self::UnionInScope(union_in_scope) => Some(union_in_scope),
            _ => None,
        }
    }

    pub(crate) fn map_fn_in_scope(&self) -> Option<&MapFnInScope> {
        match self {
            Self::MapFnInScope(map_fn_in_scope) => Some(map_fn_in_scope),
            _ => None,
        }
    }
}
