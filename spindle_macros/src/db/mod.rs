use spindle_db::primitive::AsDbPrimitive;

use crate::case::PrimitiveIdent;

// impl AsDbUnion for NewUnion {
//     type Primitive = PrimitiveIdent;

//     fn db_ident(&self) -> String {
//         self.0.0.to_string()
//     }

//     fn db_fields(&self) -> Vec<Self::Primitive> {
//         self.1.clone()
//     }
// }

impl AsDbPrimitive for PrimitiveIdent {
    fn db_ident(&self) -> String {
        self.0.to_string()
    }
}
