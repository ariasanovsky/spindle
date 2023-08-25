use crate::dev_item_fn::{DevItemFn, DevSignature, DevReturnType, DevFnArg, DevArgType};

use super::DevMapFn;

impl TryFrom<DevItemFn> for DevMapFn {
    type Error = syn::Error;

    fn try_from(value: DevItemFn) -> Result<Self, Self::Error> {
        let DevItemFn { vis, sig, block } = value;
        let DevSignature { fn_token: _, ident: _, paren_token: _, inputs, output } = &sig;
        match output {
            DevReturnType::Default => unreachable!("DevReturnType::Default"),
            DevReturnType::Type(_arrow, out_type) => {
                let mut inputs = inputs.pairs();
                let (first, second) = (inputs.next(), inputs.next());
                match (first, second) {
                    (None, _) => {
                        Err(syn::Error::new_spanned(
                            out_type,
                            "Expected at least one input parameter",
                        ))
                    }
                    (Some(first), None) => {
                        let arg = match first {
                            syn::punctuated::Pair::Punctuated(arg, _) => arg,
                            syn::punctuated::Pair::End(arg) => arg,
                        };
                        let DevFnArg { pat: _, colon_token: _, ty } = arg;
                        match ty {
                            DevArgType::DeviceType(_ty) => Ok({
                                Self { vis, sig, block }
                            }),
                            DevArgType::SpindleAny(any) => Err(syn::Error::new_spanned(
                                any,
                                "Expected a device type, not Any.",
                            )),
                            DevArgType::SpindleAnyFullPath(any_path) => Err(syn::Error::new_spanned(
                                any_path,
                                "Expected a device type, not spindle::zst::Any.",
                            )),
                        }
                    }
                    (_, Some(second)) => {
                        Err(syn::Error::new_spanned(
                            second.into_value(),
                            "Expected only one input parameter.",
                        ))
                    }
                }
            }
            DevReturnType::Tuple(_arrow, _) => todo!("DevReturnType::Tuple"),
        }
    }
}