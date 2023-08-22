use rusqlite::OptionalExtension;

use crate::{TypeDb, tag::AsDbTag, DbResult};

use super::{AsDbItemFn, DbItemFn};

impl TypeDb {
    pub fn get_or_insert_item_fn<F: AsDbItemFn, T: AsDbTag>(
        &self,
        item_fn: &F,
        tags: &[T],
    ) -> DbResult<DbItemFn> {
        let db_item_fn = self.get_item_fn(item_fn)
        .transpose()
        .unwrap_or_else(|| self.insert_item_fn(item_fn))?;
        tags.iter().try_for_each(|tag| {
            self.tag_or_ignore_item_fn(&db_item_fn, tag)
        })?;
        Ok(db_item_fn)
    }

    pub(crate) fn get_item_fn<F: AsDbItemFn>(&self, item_fn: &F) -> DbResult<Option<DbItemFn>> {
        // get all item_fns with the same content
        let mut stmt = self.conn.prepare("SELECT uuid FROM item_fns WHERE content = ?")?;
        let content = item_fn.db_item_content();
        // if there are no item_fns with the same content, return None
        // if there are item_fns with the same content, return the first one
        let uuid: Option<String> = stmt.query_row([&content], |row| row.get(0)).optional()?;
        Ok(uuid.map(|uuid| {
            DbItemFn {
                uuid,
                ident: item_fn.db_item_ident(),
                content,
            }
        }))
    }

    pub(crate) fn insert_item_fn<F: AsDbItemFn>(&self, item_fn: &F) -> DbResult<DbItemFn> {
        let mut stmt = self.conn.prepare("INSERT INTO item_fns (uuid, ident, content) VALUES (?, ?, ?)")?;
        let db_item_fn = DbItemFn {
            uuid: TypeDb::new_uuid(),
            ident: item_fn.db_item_ident(),
            content: item_fn.db_item_content(),
        };
        stmt.execute([&db_item_fn.uuid, &db_item_fn.ident, &db_item_fn.content])?;
        Ok(db_item_fn)
    }

    pub(crate) fn tag_or_ignore_item_fn<T: AsDbTag>(
        &self,
        item_fn: &DbItemFn,
        tag: &T,
    ) -> DbResult<()> {
        // get all DbItemFn's with this ident and tag
        let mut stmt = self.conn.prepare("SELECT item_fn_uuid FROM tagged_item_fns WHERE tag = ?")?;
        let tag = tag.db_tag();
        let similar_item_fns: Vec<(String, String)> = stmt.query_map([&tag], |row| {
            let uuid: String = row.get(0)?;
            // get the content of the DbItemFn with this uuid
            let mut stmt = self.conn.prepare("SELECT content FROM item_fns WHERE uuid = ?")?;
            let content: String = stmt.query_row([&uuid], |row| row.get(0))?;
            Ok((uuid, content))
        })?.collect::<DbResult<_>>()?;
        match &similar_item_fns[..] {
            [] => {
                // if there are no similar item_fns, insert the tag
                let mut stmt = self.conn.prepare("INSERT INTO tagged_item_fns (item_fn_uuid, tag) VALUES (?, ?)")?;
                let _: usize = stmt.execute([&item_fn.uuid, &tag])?;
                Ok(())
            }
            similar_item_fns => {
                similar_item_fns.iter().try_for_each(|similar_item| {
                    // if there is one similar item_fn, check if it has the same content
                    if similar_item.1 != item_fn.content {
                        // if they have different content, fatal error, give the user a panic
                        // todo! ?handle error better
                        panic!(
                            "Tag `{}` is on two distinct `{}` functions.\nRemove the duplicates and run `cargo spindle clean`.",
                            tag,
                            item_fn.ident,
                        )
                    } else {
                        // if it has the same content, ignore the tag
                        Ok(())
                    }
                })
            },
        }
    }
}
