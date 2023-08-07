use crate::primitives::_Primitive;

#[derive(Debug, Clone)]
pub(crate) struct InOut {
    pub(crate) input: Option<_Primitive>,
    pub(crate) output: Option<_Primitive>,
}
