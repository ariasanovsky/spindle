use crate::case::{UpperCamelIdent, PrimitiveIdent};

mod tokens;
#[cfg(test)]
mod test;

#[derive(Debug)]
pub(crate) enum RawUnionInput {
    UnionInScope(UpperCamelIdent),
    NewUnion(UpperCamelIdent, Vec<PrimitiveIdent>),
}

impl RawUnionInput {
    pub(crate) fn ident(&self) -> &UpperCamelIdent {
        match self {
            Self::UnionInScope(ident) => ident,
            Self::NewUnion(ident, _) => ident,
        }
    }

    pub(crate) fn fields(&self) -> Option<&Vec<PrimitiveIdent>> {
        match self {
            Self::UnionInScope(_) => None,
            Self::NewUnion(_, fields) => Some(fields),
        }
    }
}
