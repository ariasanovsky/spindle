use quote::ToTokens;
use spindle_db::item_fn::AsDbItemFn;

use crate::dev_item_fn::{DevSignature, DevFnIdent};

use super::DevMapFn;

impl AsDbItemFn for DevMapFn {
    fn db_item_ident(&self) -> String {
        let Self { vis: _, sig, block: _ } = self;
        let DevSignature { fn_token: _, ident, paren_token: _, inputs: _, output: _ } = sig;
        let DevFnIdent(ident) = ident;
        ident.to_string()
    }

    fn db_item_content(&self) -> String {
        self.to_token_stream().to_string()
    }
}
