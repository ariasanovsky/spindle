use proc_macro2::Span;
use syn::{ItemFn, Signature, Error};

use super::{DeviceItemFn, DevSignature, DevFnIdent, DevReturnType};

impl TryFrom<ItemFn> for DeviceItemFn {
    type Error = Error;

    fn try_from(value: ItemFn) -> Result<Self, Self::Error> {
        let ItemFn {
            attrs,
            vis,
            sig,
            block,
        } = value;
        let attrs = attrs.into_iter();
        if attrs.len() != 0 {
            let attrs = quote::quote_spanned! { Span::mixed_site() => 
                #(#attrs)*
            };
            return Err(Error::new_spanned(attrs, "Device functions attributes are not supported."));
        }
        Ok(Self {
            vis,
            sig: sig.try_into()?,
            block: *block,
        })
    }
}

impl TryFrom<Signature> for DevSignature {
    type Error = Error;

    fn try_from(value: Signature) -> Result<Self, Self::Error> {
        let Signature {
            constness,
            asyncness,
            unsafety,
            abi,
            fn_token,
            ident,
            generics,
            paren_token,
            inputs,
            variadic,
            output,
        } = value;
        if let Some(constness) = constness {
            return Err(Error::new_spanned(constness, "Device functions cannot be const."));
        }
        if let Some(asyncness) = asyncness {
            return Err(Error::new_spanned(asyncness, "Device functions cannot be async."));
        }
        if let Some(unsafety) = unsafety {
            return Err(Error::new_spanned(unsafety, "Device functions cannot be unsafe."));
        }
        if let Some(abi) = abi {
            return Err(Error::new_spanned(abi, "Device functions cannot have an abi."));
        }
        if let Some(variadic) = variadic {
            return Err(Error::new_spanned(variadic, "Device functions cannot be variadic."));
        }
        let ident = ident.try_into()?;
        let inputs = todo!();
        let output = output.try_into()?;
        Ok(Self {
            fn_token,
            ident,
            paren_token,
            inputs,
            output,
        })
    }
}

impl TryFrom<syn::Ident> for DevFnIdent {
    type Error = Error;

    fn try_from(value: syn::Ident) -> Result<Self, Self::Error> {
        todo!();
    }
}

impl TryFrom<syn::ReturnType> for DevReturnType {
    type Error = Error;

    fn try_from(value: syn::ReturnType) -> Result<Self, Self::Error> {
        todo!();
    }
}
