use crate::{DbResult, TypeDb};

use super::DbMap;

// iter over all DbMaps in the db
impl TypeDb {
    // fallible function to iterate over all DbMaps
    pub fn map_iter(&self) -> DbResult<MapIter /* <'a> */> {
        let mut stmt = self.conn.prepare("SELECT uuid FROM maps")?;
        let maps: Vec<DbMap> = stmt
            .query_map([], |row| {
                let uuid: String = row.get(0)?;
                self.get_map_from_uuid(uuid)
            })?
            .collect::<DbResult<_>>()?;
        Ok(MapIter { maps })
    }
}

// iterator over all DbMaps in the db
// https://docs.rs/rusqlite/latest/rusqlite/struct.MappedRows.html#method.into_iter
// lazily we'll just collect a Vec when the iterator is created
// todo! make this lazy
// pub struct MapIter<'a> {
//     db: &'a TypeDb,
//     stmt: rusqlite::Statement<'a>,
// }
pub struct MapIter {
    maps: Vec<DbMap>,
}

// impl<'a> Iterator for MapIter<'a> {
impl Iterator for MapIter {
    type Item = DbMap;

    fn next(&mut self) -> Option<Self::Item> {
        self.maps.pop()
    }
}
