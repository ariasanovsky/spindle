use crate::{TypeDb, DbResult};

use super::DbItemFn;

impl TypeDb {
    pub fn get_tags_on_item_fn(&self, db_item_fn: &DbItemFn) -> DbResult<Vec<String>> {
        let mut stmt = self.conn.prepare("SELECT tag FROM tagged_item_fns WHERE item_fn_uuid = ?")?;
        let rows = stmt.query_map([&db_item_fn.uuid], |row| row.get(0))?;
        rows.collect()
    }

    pub fn get_item_fns_with_tag(&self, tag: &str) -> DbResult<Vec<DbItemFn>> {
        let mut stmt = self.conn.prepare("SELECT item_fn_uuid FROM tagged_item_fns WHERE tag = ?")?;
        let rows = stmt.query_map([&tag], |row| {
            let uuid: String = row.get(0)?;
            self.get_item_fn_by_uuid(uuid)
        })?;
        rows.collect()
    }

    pub(crate) fn get_item_fn_by_uuid(&self, uuid: String) -> DbResult<DbItemFn> {
        let mut stmt = self.conn.prepare("SELECT ident, content FROM item_fns WHERE uuid = ?")?;
        let (ident, content) = stmt.query_row([&uuid], |row| {
            let ident: String = row.get(0)?;
            let content: String = row.get(1)?;
            Ok((ident, content))
        })?;
        Ok(DbItemFn { uuid, ident, content })
    }
}