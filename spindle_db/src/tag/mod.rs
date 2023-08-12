use crate::{TypeDb, DbResult, map::DbMap};

impl TypeDb {
    pub fn get_maps_from_tag<T: AsDbTag>(&self, tag: T) -> DbResult<Vec<DbMap>> {
        let mut stmt = self.conn.prepare("SELECT uuid FROM maps WHERE content = ?")?;
        let uuids = stmt.query_map([tag.db_tag()], |row| row.get(0))?;
        uuids.map(|uuid| {
            let uuid: String = uuid?;
            self.get_map_from_uuid(uuid)
        }).collect::<DbResult<_>>()
    }

    pub fn tag_map<T: AsDbTag>(&self, map: &DbMap, tags: &[T]) -> DbResult<()> {
        dbg!();
        let mut stmt = self.conn.prepare("INSERT INTO map_tags (map_uuid, map_ident, tag) VALUES (?, ?, ?)")?;
        for tag in tags {
            stmt.execute([map.uuid.as_str(), map.content.as_str(), tag.db_tag().as_str()])?;
        }
        Ok(())
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
