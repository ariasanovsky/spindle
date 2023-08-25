use std::fmt::Debug;

use proc_macro2::{Ident, Span};
use quote::ToTokens;
use spindle_db::TypeDb;

use crate::{tag::CrateTag, dev_item_fn::{DevFnIdent, DevFnArg, DevReturnType, DevArgType}};

mod db;
mod device;
mod kernel;
mod launch;
mod parse;
#[cfg(test)]
mod test;
mod tokens;
mod try_from;

#[derive(Debug, PartialEq)]
pub struct Attrs {
    pub tags: Vec<CrateTag>,
}

pub struct DevInitFn {
    pub vis: syn::Visibility,
    pub sig: DevInitSignature,
    pub block: syn::Block,
}

impl DevInitFn {
    pub fn ident(&self) -> &Ident {
        let Self { vis: _, sig, block: _ } = self;
        let DevInitSignature { fn_token: _, ident, paren_token: _, input: _, comma: _, output: _ } = sig;
        let DevFnIdent(ident) = ident;
        ident
    }

    pub fn input_type(&self) -> &DevArgType {
        let Self { vis: _, sig, block: _ } = self;
        let DevInitSignature { fn_token: _, ident: _, paren_token: _, input, comma: _, output: _ } = sig;
        let DevFnArg { pat: _, colon_token: _, ty } = input;
        ty
    }

    pub fn output(&self) -> &DevReturnType {
        let Self { vis: _, sig, block: _ } = self;
        let DevInitSignature { fn_token: _, ident: _, paren_token: _, input: _, comma: _, output } = sig;
        output
    }
}

pub struct DevInitSignature {
    // pub constness: Option<Const>,
    // pub asyncness: Option<Async>,
    // pub unsafety: Option<Unsafe>,
    // pub abi: Option<Abi>,
    pub fn_token: syn::Token![fn],
    pub ident: DevFnIdent,
    // pub generics: Generics,
    pub paren_token: syn::token::Paren,
    pub input: DevFnArg,
    pub comma: Option<syn::token::Comma>,
    // pub variadic: Option<Variadic>,
    pub output: DevReturnType,
}

pub fn init(attrs: Attrs, init_fn: DevInitFn) -> proc_macro2::TokenStream {
    let Attrs { tags } = attrs;
    // todo! unwraps
    let db = TypeDb::open_or_create_default().unwrap();
    db.create_or_ignore_tables_for_tagged_item_fns().unwrap();
    let _db_item_fn = db.get_or_insert_item_fn(&init_fn, &tags).unwrap();
    let trait_tokens = init_fn.launch_trait();
    todo!("init::init: write crate, compile");
    quote::quote_spanned! { Span::mixed_site() => 
        #init_fn
        #trait_tokens
    }
}
