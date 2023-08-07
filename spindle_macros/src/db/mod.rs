use quote::ToTokens;
use spindle_db::{map::AsDbMap, primitive::AsDbPrimitive};

use crate::{map::MapFn, primitives::_Primitive};

impl AsDbPrimitive for _Primitive {
    fn db_ident(&self) -> String {
        self.ident.0.to_string()
    }
}

impl AsDbMap for MapFn {
    type Primitive = _Primitive;

    fn db_content(&self) -> String {
        self.item_fn.clone().into_token_stream().to_string()
    }

    fn db_inout_pairs(&self) -> Vec<(Option<Self::Primitive>, Option<Self::Primitive>)> {
        let foo = &self.in_outs;
        todo!()
    }
}
