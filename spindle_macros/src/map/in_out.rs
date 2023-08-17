use crate::primitives::_Primitive;

#[derive(Debug, Clone)]
pub struct InOut {
    pub input: Option<_Primitive>,
    pub output: Option<_Primitive>,
}
