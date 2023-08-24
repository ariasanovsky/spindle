use crate::{TypeDb, DbResult};

const CREATE_TAGS: &str = "
    CREATE TABLE tags (
    tag TEXT NOT NULL PRIMARY KEY
)";

impl TypeDb {
    pub fn create_tags_table(&self) -> DbResult<()> {
        let _: usize = self.conn.execute(CREATE_TAGS, [])?;
        Ok(())
    }
}