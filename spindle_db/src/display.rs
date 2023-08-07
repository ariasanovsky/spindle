use std::fmt::Debug;

use crate::TypeDb;

impl Debug for TypeDb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // todo! table sizes/contents
        f.debug_struct("TypeDb")
        .field("conn", &self.conn)
        .field("tables", &self.table_names())
        .finish()
    }
}