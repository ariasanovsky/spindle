use quote::ToTokens;
use spindle_db::item_fn::{AsDbItemFn, DbItemFn};

use crate::dev_item_fn::DevFnIdent;

use super::{DevInitFn, DevInitSignature};

impl AsDbItemFn for DevInitFn {
    fn db_item_ident(&self) -> String {
        let Self { vis: _, sig, block: _ } = self;
        let DevInitSignature { fn_token: _, ident, paren_token: _, input: _, comma: _, output: _ } = sig;
        let DevFnIdent(ident) = ident;
        ident.to_string()
    }

    fn db_item_content(&self) -> String {
        self.to_token_stream().to_string()
    }
}

impl TryFrom<DbItemFn> for DevInitFn {
    type Error = syn::Error;

    fn try_from(value: DbItemFn) -> Result<Self, Self::Error> {
        let DbItemFn { uuid: _, ident: _, content } = value;
        syn::parse_str(&content)
    }
}