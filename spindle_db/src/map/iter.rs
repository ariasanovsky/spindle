use crate::{TypeDb, DbResult};

use super::DbMap;

// iter over all DbMaps in the db
impl<'a> TypeDb {
    // fallible function to iterate over all DbMaps
    pub fn map_iter(&'a self) -> DbResult<MapIter<'a>> {
        let stmt = self.conn.prepare("SELECT uuid FROM maps")?;
        Ok(MapIter { db: self, stmt })
    }
}

// iterator over all DbMaps in the db
pub struct MapIter<'a> {
    db: &'a TypeDb,
    stmt: rusqlite::Statement<'a>,
}

impl<'a> Iterator for MapIter<'a> {
    type Item = DbResult<DbMap>;

    fn next(&mut self) -> Option<Self::Item> {
        let uuid: String = match self.stmt.query_row([], |row| row.get(0)) {
            Ok(uuid) => uuid,
            Err(rusqlite::Error::QueryReturnedNoRows) => return None,
            Err(e) => return Some(Err(e.into())),
        };
        Some(self.db.get_map_from_uuid(uuid))
    }
}
