use crate::{DbResult, TypeDb};

#[cfg(test)]
mod test;

#[derive(Clone, Debug, Eq)]
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

impl AsDbPrimitive for String {
    fn db_ident(&self) -> String {
        self.clone()
    }
}

impl TypeDb {
    pub fn get_or_insert_primitive<P: AsDbPrimitive>(
        &self,
        primitive: &P,
    ) -> DbResult<DbPrimitive> {
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

    pub(crate) fn get_primitive_from_uuid(&self, uuid: String) -> DbResult<DbPrimitive> {
        let mut statement = self.conn.prepare("SELECT * FROM primitives WHERE uuid = ?")?;
        let (uuid, ident) = statement.query_row([uuid], |row| {
            Ok((row.get(0)?, row.get(1)?))
        })?;
        Ok(DbPrimitive { uuid, ident })
    }

    // todo! clippy wanted me to lose the let binding
    // following the opaque suggestion led to `temporary value` binding errors
    #[allow(clippy::needless_return)]
    pub(crate) fn _get_primitives(&self) -> DbResult<Vec<DbPrimitive>> {
        let mut statement = self.conn.prepare("SELECT * FROM primitives")?;
        return statement
            .query_map([], |row| {
                Ok(DbPrimitive {
                    uuid: row.get(0)?,
                    ident: row.get(1)?,
                })
            })?
            .collect::<DbResult<_>>();
        // primitives
    }

    pub(crate) fn get_primitive_uuid(&self, ident: &str) -> DbResult<Option<String>> {
        let mut stmt = self.conn.prepare("SELECT uuid FROM primitives WHERE ident = ?")?;
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
        let mut stmt = self.conn.prepare("INSERT INTO primitives VALUES (?, ?)")?;
        stmt.execute([prim.uuid.as_str(), prim.ident.as_str()])?;
        Ok(())
    }
}
