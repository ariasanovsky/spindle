use std::{path::PathBuf, fmt::Debug};

use sqlite::Connection;

pub mod primitive;
pub mod union;

const HOME : &str = ".spindle";
const TEST: &str = "tests";
const PROJECT: &str = "types";

pub struct TypeDb {
    pub conn: Connection,
}

pub type DbResult<T> = Result<T, sqlite::Error>;

pub trait DbIdent {
    fn db_ident(&self) -> String;   // todo! Cow<str>
}

// todo! operators like this are neat
// pub trait Context {
//     fn context(&self) -> DbResult<()>;
// }

impl TypeDb {
    pub fn connect() -> Result<Self, sqlite::Error> {
        let home = PathBuf::from(HOME).join(PROJECT).with_extension("db");
        Self::connect_to(&home, PROJECT)
    }

    pub fn connect_test(name: &str) -> Result<Self, sqlite::Error> {
        let home = PathBuf::from(HOME).join(TEST);
        Self::connect_to(&home, name)
    }

    fn connect_to(dir: &PathBuf, name: &str) -> Result<Self, sqlite::Error> {
        // create the home directory if it doesn't exist
        std::fs::create_dir_all(&dir).map_err(|e| {
            sqlite::Error {
                code: None, // todo! what code for not found?
                message: Some(format!("failed to create db home: {e}")),
            }
        })?;
        let db = dir.join(name).with_extension("db");
        dbg!("got here", &dir);
        let db = sqlite::open(db).map(|conn| Self { conn })?;
        dbg!("got here");
        Ok(db)
    }
}
