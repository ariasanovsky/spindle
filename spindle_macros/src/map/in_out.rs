use crate::primitives::_Primitive;

// todo! ?refactor to use `PrimitiveIdent` instead of this useless struct
#[derive(Debug, Clone)]
pub struct InOut {
    pub input: Option<_Primitive>,
    pub output: Option<_Primitive>,
}
