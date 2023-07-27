use proc_macro2::Ident;
use uuid::Uuid;

use crate::db::TypeDb;

use super::Primitive;

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

    pub fn add_primitive_if_not_exists(&self, prim: &Primitive) -> Result<(), sqlite::Error> {
        let ident = prim.ident();
        let span = prim.span();
        // check for a primitive with the same ident
        let mut statement = self.conn.prepare(
            "SELECT * FROM primitives WHERE ident = ?"
        )?;
        let ident_str = ident.to_string();
        statement.bind((1, ident_str.as_str()))?;
        match statement.next()? {
            sqlite::State::Row => return Ok(()),
            sqlite::State::Done => {}
        }
        // insert the primitive
        let mut statement = self.conn.prepare(
            "INSERT INTO primitives (uuid, ident, span) VALUES (?, ?, ?)"
        )?;
        let span = format!("{:?}", ident.span());
        statement.bind((1, Uuid::new_v4().to_string().as_str()))?;
        statement.bind((2, ident_str.as_str()))?;
        statement.bind((3, span.as_str()))?;
        statement.next()?;
        Ok(())
    }
}