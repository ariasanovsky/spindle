use uuid::Uuid;

use crate::{db::TypeDb, case::LowerSnakeIdent};

use super::_Primitive;

impl TypeDb {
    pub fn new_primitives(&self) -> Result<(), sqlite::Error> {
        self.conn.execute(
            "DROP TABLE IF EXISTS primitives"
        )?;
        
        self.conn.execute(
            "CREATE TABLE primitives (
                uuid TEXT NOT NULL PRIMARY KEY,     // Unique identifier
                ident TEXT NOT NULL,                // Primitive identifier
                span TEXT NOT NULL                  // Span information
            )"
        )
    }

    pub fn get_primitive_uuid(&self, prim: &_Primitive) -> Result<Option<String>, sqlite::Error> {
        let ident = prim.ident();
        let mut statement = self.conn.prepare(
            "SELECT uuid FROM primitives WHERE ident = ?"
        )?;
        let ident_str = ident.to_string();
        statement.bind((1, ident_str.as_str()))?;
        match statement.next()? {
            sqlite::State::Row => {
                let uuid: String = statement.read(0)?;
                Ok(Some(uuid))
            },
            sqlite::State::Done => Ok(None)
        }
    }

    pub fn add_primitive_if_not_exists(&self, lower_ident: LowerSnakeIdent) -> Result<_Primitive, sqlite::Error> {
        let ident = &lower_ident.0;
        let span = ident.span();
        // check for a primitive with the same ident
        let mut statement = self.conn.prepare(
            "SELECT * FROM primitives WHERE ident = ?"
        )?;
        let ident_str = ident.to_string();
        statement.bind((1, ident_str.as_str()))?;
        Ok(match statement.next()? {
            sqlite::State::Row => {
                // the primitive already exists
                // get the uuid and return the primitive
                let uuid: String = statement.read(0)?;
                _Primitive {
                    uuid,
                    ident: lower_ident,
                }
            },
            sqlite::State::Done => {
                // the primitive does not exist
                // create a new uuid
                let uuid = Uuid::new_v4().to_string();
                // insert the primitive into the database
                let mut statement = self.conn.prepare(
                    "INSERT INTO primitives (uuid, ident, span) VALUES (?, ?, ?)"
                )?;
                statement.bind((1, uuid.as_str()))?;
                statement.bind((2, ident_str.as_str()))?;
                statement.bind((3, format!("{span:?}").as_str()))?;
                statement.next()?;
                // return the primitive
                _Primitive {
                    uuid,
                    ident: lower_ident,
                }
            }
        })
    }
}