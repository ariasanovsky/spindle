use proc_macro2::Ident;

use crate::case::{LowerSnakeIdent, PrimitiveIdent, UpperCamelIdent};

#[cfg(test)]
mod test;
mod tokens;

#[derive(Debug)]
pub(crate) enum RawSpinInput {
    UnionInScope(UnionInScope),
    NewUnion(NewUnion),
    MapFnInScope(MapFnInScope),
}

#[derive(Debug)]
pub struct UnionInScope(pub UpperCamelIdent);

#[derive(Debug)]
pub struct NewUnion(pub UpperCamelIdent, pub Vec<PrimitiveIdent>);

#[derive(Debug)]
pub(crate) struct MapFnInScope(pub LowerSnakeIdent);

impl RawSpinInput {
    pub(crate) fn _ident(&self) -> &Ident {
        match self {
            Self::UnionInScope(ident) => &ident.0 .0,
            Self::NewUnion(ident) => &ident.0 .0,
            Self::MapFnInScope(ident) => &ident.0 .0,
        }
    }

    pub(crate) fn _new_union(&self) -> Option<&NewUnion> {
        match self {
            Self::NewUnion(new_union) => Some(new_union),
            _ => None,
        }
    }

    pub(crate) fn _union_in_scope(&self) -> Option<&UnionInScope> {
        match self {
            Self::UnionInScope(union_in_scope) => Some(union_in_scope),
            _ => None,
        }
    }

    pub(crate) fn _map_fn_in_scope(&self) -> Option<&MapFnInScope> {
        match self {
            Self::MapFnInScope(map_fn_in_scope) => Some(map_fn_in_scope),
            _ => None,
        }
    }
}
