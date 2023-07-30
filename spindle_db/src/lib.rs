use std::path::PathBuf;

use rusqlite::{Connection, Result};

pub mod primitive;
pub mod test;
// pub mod union;

pub(crate) const HOME : &str = ".spindle";
pub(crate) const DB: &str = "db";
const PROJECT: &str = "types";
const TABLES: &str = "SELECT name FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%'";

pub struct TypeDb {
    pub conn: Connection,
}

pub type DbResult<T> = Result<T>;

impl TypeDb {
    // make a uuid string for entries
    pub fn new_uuid() -> String {
        uuid::Uuid::new_v4().to_string()
    }

    // (create or) open a database
    pub fn open(path: PathBuf) -> DbResult<TypeDb> {
        Connection::open(path).map(|conn| TypeDb { conn })
    }

    pub(crate) fn new(path: PathBuf) -> DbResult<TypeDb> {
        let db = Self::open(path)?;
        db.drop_all()?;
        Ok(db)
    }

    // wipe all tables
    pub(crate) fn drop_all(&self) -> DbResult<()> {
        // get table names
        let mut statement = self.conn.prepare(TABLES)?;
        let mut rows = statement.query([])?;
        while let Some(row) = rows.next()? {
            let table: String = row.get(0)?;
            // drop tables
            let _: usize = self.conn.execute("DROP TABLE ?", [table])?;
        }
        Ok(())
    }
}
