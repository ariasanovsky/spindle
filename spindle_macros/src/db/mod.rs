use quote::ToTokens;
use spindle_db::{
    map::{AsDbInOut, AsDbMap},
    primitive::AsDbPrimitive,
    union::AsDbUnion, item_fn::AsDbItemFn,
};

use crate::{
    case::PrimitiveIdent,
    map_fn::{in_out::InOut, DevMapFn},
    union::NewUnion, dev_item_fn::{DevSignature, DevFnIdent},
};

impl AsDbUnion for NewUnion {
    type Primitive = PrimitiveIdent;

    fn db_ident(&self) -> String {
        self.0.0.to_string()
    }

    fn db_fields(&self) -> Vec<Self::Primitive> {
        self.1.clone()
    }
}

impl AsDbPrimitive for PrimitiveIdent {
    fn db_ident(&self) -> String {
        self.0.to_string()
    }
}
