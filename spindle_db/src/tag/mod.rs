use crate::{TypeDb, DbResult, map::DbMap};

impl TypeDb {
    pub fn get_maps_from_tag<T: AsDbTag>(&self, tag: T) -> DbResult<Vec<DbMap>> {
        todo!()
    }
}

pub trait AsDbTag {
    fn db_tag(&self) -> String;
}

impl AsDbTag for &str {
    fn db_tag(&self) -> String {
        self.to_string()
    }
}
