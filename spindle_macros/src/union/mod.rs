use crate::case::{UpperCamelIdent, PrimitiveIdent};

mod tokens;
#[cfg(test)]
mod test;

pub(crate) enum RawUnionInput {
    OldUnion(UpperCamelIdent),
    NewUnion(UpperCamelIdent, Vec<PrimitiveIdent>),
}

impl RawUnionInput {
    pub(crate) fn ident(&self) -> &UpperCamelIdent {
        match self {
            Self::OldUnion(ident) => ident,
            Self::NewUnion(ident, _) => ident,
        }
    }

    pub(crate) fn fields(&self) -> Option<&Vec<PrimitiveIdent>> {
        match self {
            Self::OldUnion(_) => None,
            Self::NewUnion(_, fields) => Some(fields),
        }
    }
}
