use quote::ToTokens;
use spindle_db::{
    map::{AsDbInOut, AsDbMap},
    primitive::AsDbPrimitive, union::AsDbUnion,
};

use crate::{
    map::{in_out::InOut, MapFn},
    primitives::_Primitive, case::PrimitiveIdent, union::RawSpinInput,
};

impl AsDbPrimitive for _Primitive {
    fn db_ident(&self) -> String {
        self.ident.0.to_string()
    }
}

impl AsDbInOut for InOut {
    type Primitive = _Primitive;

    fn db_inout(&self) -> (Option<Self::Primitive>, Option<Self::Primitive>) {
        (self.input.clone(), self.output.clone())
    }
}

impl AsDbMap for MapFn {
    type InOut = InOut;

    fn db_content(&self) -> String {
        self.item_fn.clone().to_token_stream().to_string()
    }

    fn db_inout_pairs(&self) -> Vec<Self::InOut> {
        self.in_outs.clone()
    }
}

impl AsDbUnion for RawSpinInput {
    type Primitive = PrimitiveIdent;

    fn db_ident(&self) -> String {
        self.ident().0.to_string()
    }

    fn db_fields(&self) -> Vec<Self::Primitive> {
        self.fields().unwrap().clone()
    }
}

impl AsDbPrimitive for PrimitiveIdent {
    fn db_ident(&self) -> String {
        self.0.to_string()
    }
}