use crate::{TypeDb, DbResult};

const CREATE_ITEM_FNS: &str = "
    CREATE TABLE item_fns (
    uuid TEXT PRIMARY KEY,
    ident TEXT NOT NULL,        -- unique when paired with a tag
    content UNIQUE NOT NULL     -- unique identifier
)";

const CREATE_ITEM_FN_TAGS: &str = "
    CREATE TABLE tagged_item_fns (
    item_fn_uuid TEXT NOT NULL,
    tag TEXT NOT NULL -- ,
    -- todo! ?manage this myself FOREIGN KEY (item_fn_uuid) REFERENCES item_fns (uuid),
    -- todo! ?manage this myself FOREIGN KEY (tag) REFERENCES tags (tag),
    -- PRIMARY KEY (item_fn_uuid, tag)
)";

impl TypeDb {
    pub fn create_new_item_fn_table(&self) -> DbResult<()> {
        let _: usize = self.conn.execute(CREATE_ITEM_FNS, [])?;
        Ok(())
    }

    pub fn create_new_tagged_item_fn_table(&self) -> DbResult<()> {
        let _: usize = self.conn.execute(CREATE_ITEM_FN_TAGS, [])?;
        Ok(())
    }
}