use quote::ToTokens;
use spindle_db::{
    map::{AsDbInOut, AsDbMap},
    primitive::AsDbPrimitive,
    union::AsDbUnion,
};

use crate::{
    case::PrimitiveIdent,
    map_fn::{in_out::InOut, MapFn},
    union::NewUnion,
};

// impl AsDbPrimitive for _Primitive {
//     fn db_ident(&self) -> String {
//         self.ident.0.to_string()
//     }
// }

impl AsDbInOut for InOut {
    type Primitive = PrimitiveIdent;

    fn db_inout(&self) -> (Option<Self::Primitive>, Option<Self::Primitive>) {
        (self.input.clone(), self.output.clone())
    }
}

impl AsDbMap for MapFn {
    type InOut = InOut;

    fn db_ident(&self) -> String {
        self.item_fn.sig.ident.to_string()
    }

    fn db_content(&self) -> String {
        self.item_fn.clone().to_token_stream().to_string()
    }

    fn db_inouts(&self) -> Vec<Self::InOut> {
        self.in_outs.clone()
    }

    fn range_type(&self) -> Option<String> {
        None
    }
}

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
