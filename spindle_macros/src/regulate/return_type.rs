use proc_macro2::Ident;
use syn::{ReturnType, Type};

use crate::regulate::EXPECTED_INPUTS_INDENT;

use super::{ARRAYS_SOON, REFERENCES_SOON, TUPLES_SOON, UNSUPPORTED_RETURN, EXPECTED_RETURN_IDENT, UNEXPECTED_RETURN};

pub(crate) trait RegulateReturnType: Sized {
    fn ident_return(&self) -> Result<Ident, &'static str>;
    fn no_return(self) -> Result<Self, &'static str>;
}

impl RegulateReturnType for ReturnType {
    fn ident_return(&self) -> Result<Ident, &'static str> {
        let ty = match self {
            syn::ReturnType::Default => return Err(EXPECTED_RETURN_IDENT),
            syn::ReturnType::Type(_, ty) => ty.clone(),
        };
        match *ty {
            Type::Array(_) => Err(ARRAYS_SOON),
            Type::Path(type_path) => {
                // The explicit Self type in a qualified path: the T in <T as Display>::fmt.
                if type_path.qself.is_some() {
                    return Err(EXPECTED_INPUTS_INDENT);
                }
                // A path like std::slice::Iter, optionally qualified with a self-type as in <Vec<T> as SomeTrait>::Associated.
                type_path.path.get_ident().cloned().ok_or(EXPECTED_INPUTS_INDENT)
            }
            Type::Reference(_) => Err(REFERENCES_SOON),
            Type::Tuple(_) => Err(TUPLES_SOON),
            _ => Err(UNSUPPORTED_RETURN),
        }

    }

    fn no_return(self) -> Result<Self, &'static str> {
        match self {
            syn::ReturnType::Default => Ok(self),
            _ => Err(UNEXPECTED_RETURN),
        }
    }
}
