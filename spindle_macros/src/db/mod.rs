use spindle_db::{map::{AsDbMap, AsDbInOut}, primitive::AsDbPrimitive};

use crate::{map::{MapFn, in_out::InOut}, primitives::_Primitive};

impl AsDbPrimitive for _Primitive {
    fn db_ident(&self) -> String {
        self.ident.0.to_string()
    }
}

impl AsDbInOut for InOut {
    type Primitive = _Primitive;

    fn db_inout(&self) -> (Option<Self::Primitive>, Option<Self::Primitive>) {
        todo!()
    }
}

impl AsDbMap for MapFn {
    type InOut = InOut;

    fn db_content(&self) -> String {
        todo!()
    }

    fn db_inout_pairs(&self) -> Vec<Self::InOut> {
        todo!()
    }

    
}
