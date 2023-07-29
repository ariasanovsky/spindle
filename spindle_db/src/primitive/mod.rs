use crate::{TypeDb, DbResult, DbIdent};

pub struct DbPrimitive {
    pub uuid: String,
    pub ident: String,
}

impl TypeDb {
    pub fn new_primitives(&self) -> DbResult<()> {
        self.conn.execute(
            "DROP TABLE IF EXISTS primitives"
        )?;
        
        self.conn.execute(
            "CREATE TABLE primitives (
                uuid TEXT NOT NULL PRIMARY KEY,     // Unique identifier
                ident TEXT NOT NULL UNIQUE,         // Rust identifier
            )"
        )
    }

    pub fn get_or_insert_primitive<P: DbIdent>(&self, prim: &P) -> DbResult<DbPrimitive> {
        let ident = prim.db_ident();
        let mut statement = self.conn.prepare(
            "SELECT uuid FROM primitives WHERE ident = ?"
        )?;
        statement.bind((1, ident.as_str()))?;
        Ok(match statement.next()? {
            sqlite::State::Row => {
                DbPrimitive { uuid: statement.read(0)?, ident }
            },
            sqlite::State::Done => {
                let uuid = uuid::Uuid::new_v4().to_string();
                let mut statement = self.conn.prepare(
                    "INSERT INTO primitives (uuid, ident) VALUES (?, ?)"
                )?;
                statement.bind((1, uuid.as_str()))?;
                statement.bind((2, ident.as_str()))?;
                statement.next()?;
                DbPrimitive { uuid, ident }
            },
        })
    }

    pub fn get_primitive_from_uuid(&self, uuid: &str) -> DbResult<DbPrimitive> {
        let mut statement = self.conn.prepare(
            "SELECT ident FROM primitives WHERE uuid = ?"
        )?;
        statement.bind((1, uuid))?;
        match statement.next()? {
            sqlite::State::Row => {
                Ok(DbPrimitive { uuid: uuid.to_string(), ident: statement.read(0)? })
            },
            sqlite::State::Done => {
                Err(sqlite::Error {
                    code: None, // todo! what code for not found?
                    message: Some(format!("primitive uuid not found: {uuid}")),
                })
            },
        }
    }
}

#[cfg(test)]
mod primitive_db_tests {
    use super::*;

    impl DbIdent for &str {
        fn db_ident(&self) -> String {
            self.to_string()
        }
    }
    
    #[test]
    fn test_primitive_db() {
        let db = TypeDb::connect_test("primitive").unwrap();
        db.new_primitives().unwrap();
        let p = db.get_or_insert_primitive(&"i32").unwrap();
        assert_eq!(p.ident, "i32");
        let p = db.get_primitive_from_uuid(&p.uuid).unwrap();
        assert_eq!(p.ident, "i32");
    }
}