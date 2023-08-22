use crate::{dev_item_fn::{DevItemFn, DevSignature}, init::DevInitSignature};

use super::DevInitFn;

impl TryFrom<DevItemFn> for DevInitFn {
    type Error = syn::Error;

    fn try_from(value: DevItemFn) -> Result<Self, Self::Error> {
        let DevItemFn { vis, sig, block } = value;
        let sig: DevInitSignature = sig.try_into()?;
        Ok(DevInitFn { vis, sig, block })
    }
}

impl TryFrom<DevSignature> for DevInitSignature {
    type Error = syn::Error;

    fn try_from(value: DevSignature) -> Result<Self, Self::Error> {
        let DevSignature { fn_token, ident, paren_token, inputs, output } = value;
        let mut inputs = inputs.into_pairs();
        let (first, second) = (inputs.next(), inputs.next());
        match (first, second) {
            (None, _) => // todo! somewhat lazy
                Err(syn::Error::new_spanned(ident, "expected exactly one argument")),
            (Some(first), None) => {
                let (input, comma) = match first {
                    syn::punctuated::Pair::Punctuated(arg, comma) => (arg, Some(comma)),
                    syn::punctuated::Pair::End(arg) => (arg, None)
                };
                Ok(DevInitSignature { fn_token, ident, paren_token, input, comma, output })
            }
            (Some(_), Some(second)) => Err(syn::Error::new_spanned(second, "expected exactly one argument")),
        }
    }
}
