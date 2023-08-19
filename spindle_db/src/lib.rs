use std::path::PathBuf;

use rusqlite::{Connection, Result};

pub mod display;
pub mod map;
pub mod primitive;
pub mod spindle_crate;
pub mod tables;
pub mod tag;
pub mod union;

pub(crate) const _TEST: &str = "tests";
pub(crate) const DB: &str = "db";
pub(crate) const _IN_OUTS: &str = "in_outs";
pub(crate) const _CRATES: &str = "crates";
pub(crate) const _CRATE_UNIONS: &str = "crate_unions";
pub(crate) const _LIFT_CRATES: &str = "lift_crates";
pub(crate) const _LIFTS: &str = "lifts";
pub(crate) const _LIFT_ENTRIES: &str = "lift_entries";
pub(crate) const _MAPS: &str = "maps";
pub(crate) const _PRIMITIVES: &str = "primitives";
pub(crate) const _TAGS: &str = "tags";
pub(crate) const _UNION_TAGS: &str = "union_tags";
pub(crate) const _MAP_TAGS: &str = "map_tags";
pub(crate) const _UNIONS: &str = "unions";
pub(crate) const _UNION_FIELDS: &str = "union_fields"; // todo! ?leaving space for non-primitive fields
                                                       // const PROJECT: &str = "types";

pub struct TypeDb {
    pub(crate) conn: Connection,
}

pub type Error = rusqlite::Error;
pub type DbResult<T> = Result<T>;

impl TypeDb {
    pub fn open_or_create<P: std::convert::AsRef<std::ffi::OsStr>>(
        name: &str,
        home: P,
    ) -> DbResult<Self> {
        Self::open(name, &home).unwrap_or_else(|| Self::create(&home, name))
    }

    pub fn new<P: std::convert::AsRef<std::ffi::OsStr>>(name: &str, home: P) -> DbResult<Self> {
        let db = Self::open(name, &home).transpose()?;
        if let Some(db) = db {
            db.drop_tables()?;
            db.create_tables()?;
            Ok(db)
        } else {
            Self::create(&home, name)
        }
    }

    // make a uuid string for entries
    pub(crate) fn new_uuid() -> String {
        uuid::Uuid::new_v4().to_string()
    }

    pub(crate) fn open<P: std::convert::AsRef<std::ffi::OsStr>>(
        name: &str,
        home: &P,
    ) -> Option<DbResult<Self>> {
        let home = PathBuf::from(home);
        if home.exists() {
            let path = home.join(name).with_extension(DB);
            Some(Connection::open(path).map(|conn| Self { conn }))
        } else {
            None
        }
    }

    pub(crate) fn create<P: std::convert::AsRef<std::ffi::OsStr>>(
        home: &P,
        name: &str,
    ) -> DbResult<Self> {
        // create the home directory
        let home = PathBuf::from(home);
        std::fs::create_dir_all(&home).expect("could not create home directory");
        let db = home.join(name).with_extension(DB);
        // create an empty file
        std::fs::File::create(&db).expect("could not create db file");
        let db = Connection::open(db).map(|conn| Self { conn })?;
        // create the tables
        db.create_tables()?;
        Ok(db)
    }
}

impl TypeDb {
    pub(crate) fn _new_test_db(test_name: &str) -> DbResult<Self> {
        let db_home = std::env::var("CARGO_TARGET_DIR").unwrap_or_else(|_| "target".to_string());
        let path = PathBuf::from(db_home).join(_TEST);
        TypeDb::open_or_create(test_name, path)
    }
}
