use crate::{TypeDb, DbResult, map::DbMap};

impl TypeDb {
    pub fn get_maps_from_tag<T: AsDbTag>(&self, tag: &T) -> DbResult<Vec<DbMap>> {
        let mut stmt = self.conn.prepare("SELECT map_uuid FROM map_tags WHERE tag = ?")?;
        let uuids = stmt.query_map([tag.db_tag()], |row| row.get(0))?;
        uuids.map(|uuid| self.get_map_from_uuid(uuid?)).collect::<DbResult<_>>()
    }

    pub fn tag_map<T: AsDbTag>(&self, map: &DbMap, tags: &[T]) -> DbResult<()> {
        for tag in tags {
            // first, insert the tag if it does not exist
            self.insert_or_ignore_tag(tag)?;
            // first, we collect all maps with this tag and ident
            let mut maps = self.get_maps_from_tag(tag)?;
            maps.retain(|m| m.ident == map.ident);
            const INSERT: &str = "INSERT INTO map_tags (map_uuid, tag) VALUES (?, ?)";
            match &maps[..] {
                [] => {
                    // if there are no maps with this tag and ident, we insert a new map
                    let mut stmt = self.conn.prepare(INSERT)?;
                    let _: usize = stmt.execute([map.uuid.clone(), tag.db_tag()])?;
                },
                [old_map] => {
                    // if the uuid does not match, the data is inconsistent, fatal error
                    assert_eq!(
                        old_map.uuid,
                        map.uuid,
                        "tag {} is on two maps with the same ident {} but different uuids:\n{:#?}\n{:#?}",
                        tag.db_tag(),
                        map.ident,
                        map,
                        old_map,
                    );
                    // dbg!(tag.db_tag(), &map, &old_map);
                },
                _ => {
                    // if there are multiple maps with this tag and ident, the data is inconsistent, fatal error
                    // todo! handle this case
                    panic!(
                        "tag {} is on multiple maps with the same ident {}:\n{:#?}",
                        tag.db_tag(),
                        map.ident,
                        maps
                    );
                },
            }
        }
        Ok(())
    }

    pub(crate) fn insert_or_ignore_tag<T: AsDbTag>(&self, tag: &T) -> DbResult<()> {
        // insert the tag if it does not exist
        let mut stmt = self.conn.prepare("INSERT OR IGNORE INTO tags (tag) VALUES (?)")?;
        let _: usize = stmt.execute([tag.db_tag()])?;
        Ok(())
    }

    pub fn get_tags(&self) -> DbResult<Vec<String>> {
        let mut stmt = self.conn.prepare("SELECT tag FROM tags")?;
        let tags = stmt.query_map([], |row| row.get(0))?;
        tags.map(|tag| tag).collect::<DbResult<_>>()
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
