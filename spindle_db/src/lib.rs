use std::path::PathBuf;

use rusqlite::{Connection, Result};

pub mod map;
pub mod primitive;
pub mod spindle_crate;
pub mod union;

pub(crate) const HOME : &str = ".spindle";
pub(crate) const TEST: &str = "tests";
pub(crate) const DB: &str = "db";
pub(crate) const IN_OUTS: &str = "in_outs";
pub(crate) const MAPS: &str = "maps";
pub(crate) const PRIMITIVES: &str = "primitives";
pub(crate) const UNIONS: &str = "unions";
pub(crate) const UNION_FIELDS: &str = "union_fields"; // todo! ?leaving space for non-primitive fields
const PROJECT: &str = "types";
const TABLES: &str = "SELECT name FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%'";
const DROP_TABLE: &str = "DROP TABLE ?";

pub struct TypeDb {
    pub(crate) conn: Connection,
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
        // create the directory if it doesn't exist
        if !path.exists() {
            // todo! handle error w/ https://docs.rs/rusqlite/latest/rusqlite/enum.Error.html
            std::fs::create_dir_all(&path.parent().unwrap()).unwrap();
        }
        // if it exists, delete it
        if path.exists() {
            // todo! handle error
            std::fs::remove_file(&path).unwrap();
        }
        let db = Self::open(path)?;
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

impl TypeDb {
    pub(crate) fn table_names(&self) -> DbResult<Vec<String>> {
        let mut statement = self.conn.prepare(TABLES)?;
        let mut rows = statement.query([])?;
        let mut names = Vec::new();
        while let Some(row) = rows.next()? {
            let name: String = row.get(0)?;
            names.push(name);
        }
        Ok(names)
    }

    pub(crate) fn new_test_db(test_name: &str) -> DbResult<TypeDb> {
        let path = PathBuf::from(HOME)
            .join(TEST)
            .join(test_name)
            .with_extension(DB);
        let db = TypeDb::new(path)?;
        Ok(db)
    }    
}
