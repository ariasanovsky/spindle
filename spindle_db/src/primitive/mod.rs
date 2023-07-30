use crate::{TypeDb, DbResult};

#[derive(Debug)]
pub struct DbPrimitive {
    pub uuid: String,
    pub ident: String,
}

pub trait AsDbPrimitive {
    fn db_ident(&self) -> String;
}

const CREATE: &str = "
    CREATE TABLE primitives (
    uuid TEXT PRIMARY KEY,
    ident TEXT NOT NULL UNIQUE
)";
const DROP: &str = "DROP TABLE IF EXISTS primitives";
const SELECT: &str = "SELECT uuid FROM primitives WHERE ident = ?";
const INSERT: &str = "INSERT INTO primitives (uuid, ident) VALUES (?, ?)";

impl TypeDb {
    pub fn get_or_create_primitive<P: AsDbPrimitive>(&self, primitive: &P) -> DbResult<DbPrimitive> {
        let ident = primitive.db_ident();
        let uuid = self.get_primitive_uuid(&ident)?.unwrap_or({
            let uuid = Self::new_uuid();
            self.insert_primitive(&uuid, &ident)?;
            uuid
        });
        Ok(DbPrimitive { uuid, ident })
    }

    pub(crate) fn create_new_primitive_table(&self) -> DbResult<()> {
        self.drop_primitive_table()?;
        let _: usize = self.conn.execute(CREATE, [])?;
        Ok(())
    }

    pub(crate) fn drop_primitive_table(&self) -> DbResult<()> {
        let _: usize = self.conn.execute(DROP, [])?;
        Ok(())
    }

    fn get_primitive_uuid(&self, ident: &str) -> DbResult<Option<String>> {
        let mut stmt = self.conn.prepare(SELECT)?;
        let mut rows = stmt.query([ident])?;
        if let Some(row) = rows.next()? {
            let uuid = row.get(0)?;
            // since ident is unique, there is only one row
            Ok(Some(uuid))
        } else {
            Ok(None)
        }
    }

    fn insert_primitive(&self, uuid: &str, ident: &str) -> DbResult<()> {
        let mut stmt = self.conn.prepare(INSERT)?;
        let _: usize = stmt.execute([uuid, ident])?;
        Ok(())
    }
}