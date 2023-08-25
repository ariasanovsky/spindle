use std::fmt::Debug;

use quote::ToTokens;

use super::{DevItemFn, DevSignature, DevReturnType, DevType, DevTypeTuple, DevFnArg, DevArgType};

impl Debug for DevItemFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DeviceItemFn")
        // .field("vis", &self.vis)
        .field("sig", &self.sig)
        .field("block", &self.block.clone().into_token_stream().to_string())
        .finish()
    }
}

impl Debug for DevSignature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DevSignature")
        // .field("fn_token", &self.fn_token)
        .field("ident", &self.ident.0)
        // .field("paren_token", &self.paren_token)
        .field("inputs", &self.inputs.iter().collect::<Vec<_>>())
        .field("output", &self.output)
        .finish()
    }
}

impl Debug for DevReturnType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Default => write!(f, "Default"),
            Self::Type(_arg0, arg1) => f.debug_tuple("Type").field(arg1).finish(),
            Self::Tuple(_arg0, arg1) => f.debug_tuple("Tuple").field(arg1).finish(),
        }
    }
}

impl Debug for DevType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Float(arg0) => f.debug_tuple("Float").field(arg0).finish(),
            Self::Bool(arg0) => f.debug_tuple("Bool").field(arg0).finish(),
            Self::SizedInteger(arg0) => f.debug_tuple("SizedInteger").field(arg0).finish(),
        }
    }
}

impl Debug for DevTypeTuple {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DevTypeTuple")
        // .field("paren_token", &self.paren_token)
        .field("elems", &self.elems.iter().collect::<Vec<_>>())
        .finish()
    }
}

impl Debug for DevFnArg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DevFnArg")
        .field("pat", &self.pat.to_token_stream().to_string())
        // .field("colon_token", &self.colon_token)
        .field("ty", &self.ty).finish()
    }
}

impl Debug for DevArgType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DeviceType(arg0) => f.debug_tuple("DeviceType").field(arg0).finish(),
            Self::SpindleAny(arg0) => f.debug_tuple("SpindleAny").field(arg0).finish(),
            Self::SpindleAnyFullPath(_arg0) => todo!("SpindleAnyFullPath"),
                // f.debug_tuple("SpindleAnyFullPath")
                // .field(arg0)
                // .finish(),
        }
    }
}
