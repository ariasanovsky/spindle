use std::path::PathBuf;

use rusqlite::{Connection, Result};

pub mod display;
pub mod map;
pub mod primitive;
pub mod spindle_crate;
pub mod tables;
pub mod tag;
pub mod union;

pub(crate) const _DEFAULT_HOME: &str = "target/spindle/db/";
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
pub(crate) const _UNIONS: &str = "unions";
pub(crate) const _UNION_FIELDS: &str = "union_fields"; // todo! ?leaving space for non-primitive fields
                                                       // const PROJECT: &str = "types";

pub struct TypeDb {
    pub(crate) conn: Connection,
}

pub type DbResult<T> = Result<T>;

impl TypeDb {
    pub fn open_or_create<P: std::convert::AsRef<std::ffi::OsStr>>(
        name: &str,
        home: P,
    ) -> DbResult<Self> {
        dbg!();
        Self::open(name, &home).unwrap_or(Self::create(&home, name))
    }

    pub fn new<P: std::convert::AsRef<std::ffi::OsStr>>(name: &str, home: P) -> DbResult<Self> {
        // connect if it exists
        let db = Self::open(name, &home).transpose()?;
        if let Some(db) = db {
            // drop tables and recreate
            // dbg!(&db);
            db.drop_tables()?;
            // dbg!(&db);
            db.create_tables()?;
            // dbg!(&db);
            Ok(db)
        } else {
            // create
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
        // if the home exists, open the db
        let home = PathBuf::from(home);
        // dbg!(&home);
        if home.exists() {
            let path = home.join(name).with_extension(DB);
            // dbg!(&path);
            let db = Some(Connection::open(path).map(|conn| Self { conn }));
            // dbg!(&db);
            db
        } else {
            None
        }
    }

    pub(crate) fn create<P: std::convert::AsRef<std::ffi::OsStr>>(
        home: &P,
        name: &str,
    ) -> DbResult<Self> {
        dbg!();
        // create the home directory
        let home = PathBuf::from(home);
        std::fs::create_dir_all(&home).expect("could not create home directory");
        let db = home.join(name).with_extension(DB);
        // create an empty file
        std::fs::File::create(&db).expect("could not create db file");
        dbg!(&db);
        let db = Connection::open(db).map(|conn| Self { conn })?;
        // create the tables
        db.create_tables()?;
        Ok(db)
    }
}

impl TypeDb {
    pub(crate) fn _new_test_db(test_name: &str) -> DbResult<Self> {
        dbg!();
        let path = PathBuf::from(_DEFAULT_HOME).join(_TEST);
        dbg!(&path);
        TypeDb::open_or_create(test_name, path)
    }
}
