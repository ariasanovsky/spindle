use sqlite::State;
use uuid::Uuid;

use crate::{db::TypeDb, case::UpperCamelIdent};

use super::_Union;

impl TypeDb {
    pub fn new_unions(&self) -> Result<(), sqlite::Error> {
        self.conn.execute(
            "DROP TABLE IF EXISTS unions"
        )?;
        
        self.conn.execute(
            "CREATE TABLE unions (
                uuid TEXT NOT NULL PRIMARY KEY,     // Unique identifier
                ident TEXT NOT NULL,                // Union identifier
                span TEXT NOT NULL                  // Span information
            )"
        )?;

        self.conn.execute(
            "CREATE TABLE union_primitives (
                union_uuid TEXT NOT NULL,                                   // Union's unique identifier
                index INTEGER NOT NULL,                                     // Index of the primitive in the union
                primitive_uuid TEXT NOT NULL,                               // Primitive's unique identifier
                PRIMARY KEY (union_uuid, primitive_uuid),                   // Primary key
                FOREIGN KEY (union_uuid) REFERENCES unions (uuid),          // Foreign key referencing 'unions' table
                FOREIGN KEY (primitive_uuid) REFERENCES primitives (uuid)   // Foreign key referencing 'primitives' table
            )"
        )
    }

    pub fn add_union_if_not_exists(&self, upper_ident: UpperCamelIdent, fields: Option<Vec<String>>) -> Result<_Union, sqlite::Error> {
        // check for a union with the same ident
        let ident = &upper_ident.0;
        let span = ident.span();
        // we can have 2 unions in the db with the same name
        // but not also with the same fields
        match fields {
            Some(fields) => {
                // check for a union with the same ident and fields
                let prefix = "SELECT unions.uuid FROM unions WHERE unionds.ident = ?";
                let joins = fields.iter().enumerate().map(|(i, _)| {
                    format!("JOIN union_primitives AS up{} ON up{}.union_uuid = unions.uuid", i, i)
                }).collect::<Vec<_>>().join(" ");
                todo!()
            },
            None => todo!("b"),
        }
    }

    // gets all primitive_uuids associated to a union_uuid through the junction table
    pub fn get_union_fields(&self, union_uuid: &str) -> Result<Vec<String>, sqlite::Error> {
        let mut statement = self.conn.prepare(
            "SELECT index, primitive_uuid FROM union_primitives WHERE union_uuid = ?"
        )?;
        statement.bind((1, union_uuid))?;
        let mut fields = Vec::new();
        while let State::Row = statement.next()? {
            let index: i64 = statement.read(0)?;
            let primitive_uuid: String = statement.read(1)?;
            fields.push((index, primitive_uuid));
        }
        // sort by index
        fields.sort_by(|(i1, _), (i2, _)| i1.cmp(i2));
        fields.into_iter().enumerate().map(|(index, (db_index, uuid))| {
            if index != db_index as usize {
                Err(sqlite::Error {
                    code: None, // todo! what error code?
                    message: Some(format!("Union field index mismatch: expected {}, got {}", index, db_index))
                })
            } else {
                Ok(uuid)
            }
        }).collect()
    }

    // pub fn get_union_uuid(&self, u: &_Union) -> Result<Option<String>, sqlite::Error> {
    //     let ident = u.ident();
    //     let mut statement = self.conn.prepare(
    //         "SELECT uuid FROM unions WHERE ident = ?"
    //     )?;
    //     let ident_str = ident.to_string();
    //     statement.bind((1, ident_str.as_str()))?;
    //     match statement.next()? {
    //         sqlite::State::Row => {
    //             let uuid: String = statement.read(0)?;
    //             Ok(Some(uuid))
    //         },
    //         sqlite::State::Done => Ok(None)
    //     }
    // }
}