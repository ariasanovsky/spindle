use heck::ToSnakeCase;
use proc_macro2::Span;
use syn::{ItemFn, Signature, Error, punctuated::{Pair, Punctuated}, PatType, Path, TypePath, spanned::Spanned, PathSegment};

use crate::dev_item_fn::DevType;

use super::{DevItemFn, DevSignature, DevFnIdent, DevReturnType, DevFnArg, DevArgType};

impl TryFrom<ItemFn> for DevItemFn {
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
        let syn::Generics {
            lt_token: _,
            params,
            gt_token: _,
            where_clause,
        } = generics;
        let params = params.iter();
        if params.len() != 0 {
            let params = quote::quote_spanned! { Span::mixed_site() => 
                #(#params)*
            };
            return Err(Error::new_spanned(params, "Device functions cannot have generic parameters."));
        }
        if let Some(where_clause) = where_clause {
            return Err(Error::new_spanned(where_clause, "Device functions cannot have a where clause."));
        }
        if let Some(variadic) = variadic {
            return Err(Error::new_spanned(variadic, "Device functions cannot be variadic."));
        }
        let ident = ident.try_into()?;
        let mut new_inputs: Punctuated<DevFnArg, syn::token::Comma> = Punctuated::new();
        inputs.clone().into_pairs().try_for_each(|p| {
            match p {
                Pair::Punctuated(arg, comma) => {
                    let arg = arg.try_into()?;
                    new_inputs.push_value(arg);
                    new_inputs.push_punct(comma);
                    Ok::<_, Error>(())
                },
                Pair::End(arg) => {
                    let arg = arg.try_into()?;
                    new_inputs.push_value(arg);
                    Ok::<_, Error>(())
                },
            }
        })?;
        let output = output.try_into()?;
        Ok(Self {
            fn_token,
            ident,
            paren_token,
            inputs: new_inputs,
            output,
        })
    }
}

impl TryFrom<syn::Ident> for DevFnIdent {
    type Error = Error;

    fn try_from(value: syn::Ident) -> Result<Self, Self::Error> {
        // require that it is in upper camel case
        // require that there are no leading or trailing underscores
        let ident_string: String = value.to_string();
        if ident_string.starts_with('_') {
            return Err(Error::new_spanned(value, "Leading underscores are not supported in device function names."));
        }
        if ident_string.ends_with('_') {
            return Err(Error::new_spanned(value, "Trailing underscores are not supported in device function names."));
        }
        let snake_name = ident_string.to_snake_case();
        if snake_name != ident_string {
            return Err(Error::new_spanned(value, "Device function names must be in upper camel case."));
        }
        Ok(Self(value))
    }
}

impl TryFrom<syn::ReturnType> for DevReturnType {
    type Error = Error;

    fn try_from(value: syn::ReturnType) -> Result<Self, Self::Error> {
        Ok(match value {
            syn::ReturnType::Default => Self::Default,
            syn::ReturnType::Type(arrow, ty) => match *ty {
                // syn::Type::Array(_) => todo!(),
                // syn::Type::BareFn(_) => todo!(),
                // syn::Type::Group(_) => todo!(),
                // syn::Type::ImplTrait(_) => todo!(),
                // syn::Type::Infer(_) => todo!(),
                // syn::Type::Macro(_) => todo!(),
                // syn::Type::Never(_) => todo!(),
                // syn::Type::Paren(_) => todo!(),
                syn::Type::Path(path) => {
                    let TypePath { qself, path } = path;
                    if let Some(qself) = qself {
                        return Err(Error::new(qself.span(), "Device function return types cannot have a qualified self."));
                    }
                    let Path { leading_colon, segments } = path;
                    if let Some(leading_colon) = leading_colon {
                        return Err(Error::new_spanned(leading_colon, "Device function return types cannot have a leading colon."));
                    }
                    let mut segments = segments.into_pairs();
                    let (first, second) = (segments.next(), segments.next());
                    match (first, second) {
                        (None, _) => unreachable!("A single segment is always present."),
                        (Some(pair), None) => {
                            // `e.g., `f64`
                            let segment = match pair {
                                Pair::Punctuated(seg, _sep) => seg,
                                Pair::End(seg) => seg,
                            };
                            // primitives, but not `Any`
                            let ident = path_segment_to_ident(segment)?;
                            let dev_type = DevType::try_from(ident)?;
                            Self::Type(arrow, dev_type)
                        },
                        (Some(pair), Some(_)) => {
                            // todo! ?make a spindle::Fuse type alias
                            // e.g., `spindle::Fuse<(f32, f32)>`, idk about syntax
                            return Err(Error::new_spanned(pair, "Device function return types cannot have a path with more than one segment."));
                        },
                    }
                },
                // syn::Type::Ptr(_) => todo!(),
                // syn::Type::Reference(_) => todo!(),
                // syn::Type::Slice(_) => todo!(),
                // syn::Type::TraitObject(_) => todo!(),
                syn::Type::Tuple(_) => todo!(),
                // syn::Type::Verbatim(_) => todo!(),
                _ => todo!(),
            }
        })
    }
}

impl TryFrom<syn::FnArg> for DevFnArg {
    type Error = Error;

    fn try_from(value: syn::FnArg) -> Result<Self, Self::Error> {
        // but we only allow `_: _` patterns, no `self`
        match value {
            syn::FnArg::Receiver(receiver) => {
                Err(Error::new_spanned(receiver, "Device functions cannot have a receiver."))
            },
            syn::FnArg::Typed(typed) => {
                let PatType {
                    attrs,
                    pat,
                    colon_token,
                    ty,
                } = typed;
                if attrs.len() != 0 {
                    let attrs = quote::quote_spanned! { Span::mixed_site() => 
                        #(#attrs)*
                    };
                    return Err(Error::new_spanned(attrs, "Device function argument attributes are not supported."));
                }
                // todo ?we don't care about the left-hand side
                let pat = *pat;
                let ty = *ty;
                let ty = ty.try_into()?;
                Ok(Self {
                    pat,
                    colon_token,
                    ty,
                })
            }
        }
    }
}

fn path_segment_to_ident(path_segment: syn::PathSegment) -> Result<syn::Ident, Error> {
    let PathSegment { ident, arguments } = path_segment;
    match arguments {
        syn::PathArguments::None => Ok(ident),
        syn::PathArguments::AngleBracketed(args) =>
            return Err(Error::new_spanned(args, "Device function arguments cannot have angle-bracketed arguments.")),
        syn::PathArguments::Parenthesized(args) =>
            return Err(Error::new_spanned(args, "Device function arguments cannot have parenthesized arguments.")),
    }
}


impl TryFrom<syn::Type> for DevArgType {
    type Error = Error;

    fn try_from(value: syn::Type) -> Result<Self, Self::Error> {
        match value {
            // syn::Type::Array(_) => todo!(),
            // syn::Type::BareFn(_) => todo!(),
            // syn::Type::Group(_) => todo!(),
            // syn::Type::ImplTrait(_) => todo!(),
            // syn::Type::Infer(_) => todo!(),
            // syn::Type::Macro(_) => todo!(),
            // syn::Type::Never(_) => todo!(),
            // syn::Type::Paren(_) => todo!(),
            syn::Type::Path(path) => {
                let TypePath {
                    qself,
                    path,
                } = path;
                if let Some(qself) = qself {
                    return Err(Error::new(qself.span(), "Device function arguments cannot have a qualified self."));
                }
                let Path {
                    leading_colon,
                    segments,
                } = path;
                if let Some(leading_colon) = leading_colon {
                    return Err(Error::new_spanned(leading_colon, "Device function arguments cannot have a leading colon."));
                }
                let mut segments = segments.into_pairs();
                let (first, second, third) = (segments.next(), segments.next(), segments.next());
                match (first, second, third) {
                    (Some(pair), None, _) => {
                        let segment = match pair {
                            Pair::Punctuated(seg, _sep) => seg,
                            Pair::End(seg) => seg,
                        };
                        // primitives or `Any`
                        let ident = path_segment_to_ident(segment)?;
                        // check for `Any`, `bool`, and `{i|u}size` first
                        if ident == "Any" {
                            return Ok(DevArgType::SpindleAny(ident));
                        }
                        let dev_type = DevType::try_from(ident)?;
                        return Ok(DevArgType::DeviceType(dev_type));
                    },
                    (
                        Some(Pair::Punctuated(spindle, seg_1)),
                        Some(Pair::Punctuated(zst, seg_2)),
                        Some(Pair::End(any)),
                    ) => {
                        // `spindle::zst::Any`
                        let spindle = path_segment_to_ident(spindle)?;
                        let zst = path_segment_to_ident(zst)?;
                        let any = path_segment_to_ident(any)?;
                        if spindle != "spindle" || zst != "zst" || any != "Any" {
                            return Err(Error::new_spanned(spindle, "The only allowed path is `spindle::zst::Any`."));
                        }
                        let mut segments = Punctuated::new();
                        segments.push_value(spindle);
                        segments.push_punct(seg_1);
                        segments.push_value(zst);
                        segments.push_punct(seg_2);
                        segments.push_value(any);
                        return Ok(DevArgType::SpindleAnyFullPath(segments));
                    },
                    (Some(first), _, _) =>
                        // unrecognized
                        return Err(Error::new_spanned(first, "Device function arguments may be `f{32|64}`, `bool`, `{i|u}{8|16|32|64}`, `{i|u}size`, `Any`, or `spindle::zst::Any`.")),
                    (None, _, _) => unreachable!("A single segment is always present."),
                }
            },
            // syn::Type::Ptr(_) => todo!(),
            // syn::Type::Reference(_) => todo!(),
            // syn::Type::Slice(_) => todo!(),
            // syn::Type::TraitObject(_) => todo!(),
            // syn::Type::Tuple(_) => todo!(),
            // syn::Type::Verbatim(_) => todo!(),
            _ => todo!(),
        };
        // with a type from the Path family
        // without qualification
    }
}

impl TryFrom<syn::Ident> for DevType {
    type Error = Error;

    fn try_from(ident: syn::Ident) -> Result<Self, Self::Error> {
        if ident == "bool" {
            return Ok(DevType::Bool(ident));
        }
        if ident == "isize" || ident == "usize" {
            return Ok(DevType::SizedInteger(ident));
        }
        // check for `f32` and `f64` next
        if ident == "f32" || ident == "f64" {
            return Ok(DevType::Float(ident));
        }
        // check for `{i|u}{8|16|32|64}` next
        let ident_string = ident.to_string();
        const SIZED_INTEGERS: &[&str] = &[
            "i8", "i16", "i32", "i64",
            "u8", "u16", "u32", "u64",
        ];
        if SIZED_INTEGERS.contains(&ident_string.as_str()) {
            return Ok(DevType::SizedInteger(ident));
        }
        // unrecognized
        return Err(Error::new_spanned(ident, "Device types are may be `f{32|64}`, `bool`, `{i|u}{8|16|32|64}`, and `{i|u}size`."));
    }
}