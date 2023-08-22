use quote::ToTokens;
use spindle_db::item_fn::{DbItemFn, AsDbItemFn};

use crate::dev_item_fn::DevSignature;

use super::{DevItemFn, DevFnIdent};

impl TryFrom<DbItemFn> for DevItemFn {
    type Error = syn::Error;

    fn try_from(value: DbItemFn) -> Result<Self, Self::Error> {
        let DbItemFn { uuid: _, ident: _, content } = value;
        syn::parse_str(&content)
    }
}

impl AsDbItemFn for DevItemFn {
    fn db_item_ident(&self) -> String {
        let DevItemFn { vis: _, sig, block: _ } = self;
        let DevSignature { fn_token: _, ident, paren_token: _, inputs: _, output: _ } = sig;
        let DevFnIdent(ident) = ident;
        ident.to_string()
    }

    fn db_item_content(&self) -> String {
        self.to_token_stream().to_string()
    }
}