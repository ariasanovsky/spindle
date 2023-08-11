use crate::case::{UpperCamelIdent, PrimitiveIdent, LowerSnakeIdent};

mod tokens;
#[cfg(test)]
mod test;

#[derive(Debug)]
pub(crate) enum RawSpinInput {
    UnionInScope(UpperCamelIdent),
    NewUnion(UpperCamelIdent, Vec<PrimitiveIdent>),
    MapFnInScope(LowerSnakeIdent),
}

impl RawSpinInput {
    pub(crate) fn ident(&self) -> &UpperCamelIdent {
        match self {
            Self::UnionInScope(ident) => ident,
            Self::NewUnion(ident, _) => ident,
            Self::MapFnInScope(_) => todo!(),
        }
    }

    pub(crate) fn fields(&self) -> Option<&Vec<PrimitiveIdent>> {
        match self {
            Self::UnionInScope(_) => None,
            Self::NewUnion(_, fields) => Some(fields),
            Self::MapFnInScope(_) => todo!(),
        }
    }
}
