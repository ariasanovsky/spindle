use std::path::PathBuf;

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
    // connect to existing db
    pub fn connect_types() -> DbResult<Self> {
        todo!()
    }

    pub fn connect_test(name: &str) -> DbResult<Self> {
        todo!()
    }

    fn connect(db: &PathBuf) -> DbResult<Self> {
        let conn = Connection::open(db)?;
        Ok(Self { conn })
    }
    
    // create new db
    fn new(dir: &PathBuf, name: &str) -> DbResult<Self> {
        // create the home directory if it doesn't exist
        std::fs::create_dir_all(&dir).map_err(|e| {
            sqlite::Error {
                code: None, // todo! what code for not found?
                message: Some(format!("failed to create db home: {e}")),
            }
        })?;
        // delete the db if it already exists
        let db = dir.join(name).with_extension("db");
        if db.exists() {
            std::fs::remove_file(&db).map_err(|e| {
                sqlite::Error {
                    code: None, // todo! what code for not found?
                    message: Some(format!("failed to delete existing db: {e}")),
                }
            })?;
        }
        Self::connect(&db)
    }

    pub fn new_types() -> DbResult<Self> {
        // tables for primitives, unions, primitve_unions
        // tables for maps, inouts, map_inouts
        // table for spindles?
        todo!()
    }

    pub fn new_test_primitives() -> DbResult<Self> {
        // dbg!("new_test_primitives");
        let test_home = PathBuf::from(HOME).join(TEST);
        // dbg!(&test_home);
        let db = Self::new(&test_home, "primitives")?;
        // dbg!("made db");
        db.new_primitives()?;
        Ok(db)
    }

    pub fn new_test_unions() -> DbResult<Self> {
        todo!()
    }
}

// // create the home directory if it doesn't exist
// std::fs::create_dir_all(&dir).map_err(|e| {
//     sqlite::Error {
//         code: None, // todo! what code for not found?
//         message: Some(format!("failed to create db home: {e}")),
//     }
// })?;
