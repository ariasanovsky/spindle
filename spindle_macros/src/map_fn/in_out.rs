use crate::case::PrimitiveIdent;

#[derive(Debug, Clone)]
pub struct InOut {
    pub input: Option<PrimitiveIdent>,
    pub output: Option<PrimitiveIdent>,
}
