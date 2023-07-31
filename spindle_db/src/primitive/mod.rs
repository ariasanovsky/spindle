use crate::{TypeDb, DbResult};

#[allow(dead_code)]
mod test;

#[derive(Debug, Eq)]
pub struct DbPrimitive {
    pub uuid: String,
    pub ident: String,
}

impl PartialEq for DbPrimitive {
    fn eq(&self, other: &Self) -> bool {
        self.ident == other.ident
    }
}

impl DbPrimitive {
    pub(crate) fn new(ident: String) -> Self {
        Self {
            uuid: TypeDb::new_uuid(),
            ident,
        }
    }
}

pub trait AsDbPrimitive {
    fn db_ident(&self) -> String;
}

const CREATE_TABLE: &str = "
    CREATE TABLE primitives (
    uuid TEXT PRIMARY KEY,
    ident TEXT NOT NULL UNIQUE  -- unique identifier (there is only one `i32`)
)";
const DROP_TABLE: &str = "DROP TABLE IF EXISTS primitives";
const SELECT_UUID: &str = "SELECT uuid FROM primitives WHERE ident = ?";
const SELECT_IDENT: &str = "SELECT ident FROM primitives WHERE uuid = ?";
const SELECT_PRIMITIVE: &str = "SELECT uuid, ident FROM primitives";
const INSERT_PRIMITIVE: &str = "INSERT INTO primitives (uuid, ident) VALUES (?, ?)";

impl TypeDb {
    pub fn get_or_insert_primitive<P: AsDbPrimitive>(&self, primitive: &P) -> DbResult<DbPrimitive> {
        let ident = primitive.db_ident();
        let uuid = self.get_primitive_uuid(&ident)?;
        Ok(if let Some(uuid) = uuid {
            DbPrimitive { uuid, ident }
        } else {
            let primitive = DbPrimitive::new(ident);
            self.insert_primitive(&primitive)?;
            primitive
        })
    }

    pub fn get_primitive_from_uuid(&self, uuid: String) -> DbResult<Option<DbPrimitive>> {
        let mut statement = self.conn.prepare(SELECT_IDENT)?;
        let mut rows = statement.query([&uuid])?;
        // todo! more idiomatic with `map`
        Ok(if let Some(row) = rows.next()? {
            let ident = row.get(0)?;
            Some(DbPrimitive { uuid, ident })
        } else {
            None
        })
    }

    pub(crate) fn get_primitives(&self) -> DbResult<Vec<DbPrimitive>> {
        let mut statement = self.conn.prepare(SELECT_PRIMITIVE)?;
        let primitives = statement.query_map([], |row| {
            Ok(DbPrimitive {
                uuid: row.get(0)?,
                ident: row.get(1)?,
            })
        })?.collect::<DbResult<_>>();
        primitives
    }

    pub(crate) fn create_new_primitive_table(&self) -> DbResult<()> {
        self.drop_primitive_table()?;
        let _: usize = self.conn.execute(CREATE_TABLE, [])?;
        Ok(())
    }

    pub(crate) fn drop_primitive_table(&self) -> DbResult<()> {
        let _: usize = self.conn.execute(DROP_TABLE, [])?;
        Ok(())
    }

    fn get_primitive_uuid(&self, ident: &str) -> DbResult<Option<String>> {
        let mut stmt = self.conn.prepare(SELECT_UUID)?;
        let mut rows = stmt.query([ident])?;
        if let Some(row) = rows.next()? {
            let uuid = row.get(0)?;
            // since ident is unique, there is only one row
            Ok(Some(uuid))
        } else {
            Ok(None)
        }
    }

    fn insert_primitive(&self, prim: &DbPrimitive) -> DbResult<()> {
        let mut stmt = self.conn.prepare(INSERT_PRIMITIVE)?;
        stmt.execute([prim.uuid.as_str(), prim.ident.as_str()])?;
        Ok(())
    }
}
