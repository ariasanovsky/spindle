use std::path::PathBuf;

use rusqlite::{Connection, Result};

pub mod map;
pub mod primitive;
pub mod spindle_crate;
pub mod tables;
pub mod union;

pub(crate) const DEFAULT_HOME : &str = "target/spindle/db/";
pub(crate) const TEST: &str = "tests";
pub(crate) const DB: &str = "db";
pub(crate) const IN_OUTS: &str = "in_outs";
pub(crate) const CRATES: &str = "crates";
pub(crate) const CRATE_UNIONS: &str = "crate_unions";
pub(crate) const LIFT_CRATES: &str = "lift_crates";
pub(crate) const LIFTS: &str = "lifts";
pub(crate) const LIFT_ENTRIES: &str = "lift_entries";
pub(crate) const MAPS: &str = "maps";
pub(crate) const PRIMITIVES: &str = "primitives";
pub(crate) const UNIONS: &str = "unions";
pub(crate) const UNION_FIELDS: &str = "union_fields"; // todo! ?leaving space for non-primitive fields
const PROJECT: &str = "types";
const TABLES: &str = "SELECT name FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%'";

pub struct TypeDb {
    pub(crate) conn: Connection,
}

pub type DbResult<T> = Result<T>;

impl TypeDb {
    pub fn open_or_create<P: std::convert::AsRef<std::ffi::OsStr>>(name: &str, home: P) -> DbResult<Self> {
        dbg!();
        Self::open(name, &home).unwrap_or(Self::create(&home, name))
    }

    pub fn new<P: std::convert::AsRef<std::ffi::OsStr>>(name: &str, home: P) -> DbResult<Self> {
        // connect if it exists and drop all tables, else create
        let db = Self::open_or_create(name, home)?;
        db.drop_tables()?;
        Ok(db)
    }

    // make a uuid string for entries
    pub(crate) fn new_uuid() -> String {
        uuid::Uuid::new_v4().to_string()
    }

    pub(crate) fn drop_tables(&self) -> DbResult<()> {
        let mut statement = self.conn.prepare(TABLES)?;
        let mut rows = statement.query([])?;
        while let Some(row) = rows.next()? {
            let name: String = row.get(0)?;
            let sql = format!("DROP TABLE {name}");
            let _: usize = self.conn.execute(&sql, [])?;
        }
        Ok(())
    }

    pub(crate) fn open<P: std::convert::AsRef<std::ffi::OsStr>>(name: &str, home: &P) -> Option<DbResult<Self>> {
        // if the home exists, open the db
        let home = PathBuf::from(home);
        dbg!(&home);
        if home.exists() {
            let path = home.join(name).with_extension(DB);
            dbg!(&path);
            Some(Connection::open(path).map(|conn| Self { conn }))
        } else {
            None
        }
    }

    pub(crate) fn create<P: std::convert::AsRef<std::ffi::OsStr>>(home: &P, name: &str) -> DbResult<Self> {
        dbg!();
        // create the home directory
        let home = PathBuf::from(home);
        std::fs::create_dir_all(&home).expect("could not create home directory");
        let db = PathBuf::from(home).join(name).with_extension(DB);
        // create an empty file
        std::fs::File::create(&db).expect("could not create db file");
        dbg!(&db);
        Connection::open(db).map(|conn| Self { conn })
    }
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

    pub(crate) fn new_test_db(test_name: &str) -> DbResult<Self> {
        dbg!();
        let path = PathBuf::from(DEFAULT_HOME).join(TEST);
        dbg!(&path);
        TypeDb::open_or_create(test_name, path)
    }    
}
