use std::path::PathBuf;

use rusqlite::{Connection, Result};

pub mod primitive;
#[allow(dead_code)]
pub mod test;
// pub mod union;

pub(crate) const HOME : &str = ".spindle";
pub(crate) const DB: &str = "db";
const PROJECT: &str = "types";
const TABLES: &str = "SELECT name FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%'";
const DROP_TABLE: &str = "DROP TABLE ?";

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
        dbg!(&path);
        // create the directory if it doesn't exist
        if !path.exists() {
            // todo! handle error w/ https://docs.rs/rusqlite/latest/rusqlite/enum.Error.html
            dbg!();
            std::fs::create_dir_all(&path.parent().unwrap()).unwrap();
        }
        // if it exists, delete it
        if path.exists() {
            dbg!();
            // todo! handle error
            std::fs::remove_file(&path).unwrap();
        }
        dbg!();
        let db = Self::open(path)?;
        dbg!();
        // db.drop_all()?;
        // dbg!();
        Ok(db)
    }

    // wipe all tables
    // pub(crate) fn drop_all(&self) -> DbResult<()> {
    //     // get table names
    //     let mut statement = self.conn.prepare(TABLES)?;
    //     let mut rows = statement.query([])?;
    //     while let Some(row) = rows.next()? {
    //         let table: String = row.get(0)?;
    //         dbg!(&table);
    //         // drop tables
    //         let _: usize = self.conn.execute(format!("DROP TABLE {}", table).as_str(), [])?;
    //         dbg!();
    //     }
    //     Ok(())
    // }
}
