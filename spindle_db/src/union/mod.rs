use crate::{
    primitive::{AsDbPrimitive, DbPrimitive},
    DbResult, TypeDb,
};

#[cfg(test)]
mod test;

#[derive(Clone, Debug, Eq)]
pub struct DbUnion {
    pub uuid: String,
    pub ident: String,
    pub fields: Vec<DbPrimitive>,
}

impl PartialEq for DbUnion {
    fn eq(&self, other: &Self) -> bool {
        self.ident == other.ident && self.fields == other.fields
    }
}

impl DbUnion {
    pub(crate) fn new(ident: String, fields: Vec<DbPrimitive>) -> Self {
        Self {
            uuid: TypeDb::new_uuid(),
            ident,
            fields,
        }
    }
}

pub trait AsDbUnion {
    type Primitive: AsDbPrimitive;
    fn db_ident(&self) -> String;
    fn db_fields(&self) -> Vec<Self::Primitive>;
}

impl TypeDb {
    #[allow(clippy::let_and_return)]
    pub fn get_unions(&self) -> DbResult<Vec<DbUnion>> {
        let mut statement = self.conn.prepare("SELECT * FROM unions")?;
        let rows = statement
            .query_map([], |row| {
                let uuid: String = row.get(0)?;
                let ident: String = row.get(1)?;
                let fields: Vec<DbPrimitive> = self.get_union_fields(&uuid)?;
                Ok(DbUnion {
                    uuid,
                    ident,
                    fields,
                })
            })?
            .collect::<DbResult<_>>();
        rows
    }

    pub fn get_or_insert_union<U: AsDbUnion>(&self, union: &U) -> DbResult<DbUnion> {
        let fields = union
            .db_fields()
            .into_iter()
            .map(|p| self.get_or_insert_primitive(&p))
            .collect::<DbResult<_>>()?;
        let db_union = self
            .get_union(union, &fields)
            .transpose()
            .unwrap_or_else(|| self.insert_union(union, fields))?;
        Ok(db_union)
    }

    pub(crate) fn get_union_from_uuid(&self, uuid: String) -> DbResult<DbUnion> {
        let mut statement = self
            .conn
            .prepare("SELECT ident FROM unions WHERE uuid = ?")?;
        let ident: String = statement.query_row([&uuid], |row| row.get(0))?;
        let fields = self.get_union_fields(&uuid)?;
        Ok(DbUnion {
            uuid,
            ident,
            fields,
        })
    }

    #[allow(clippy::let_and_return)]
    pub(crate) fn get_union<U: AsDbUnion>(
        &self,
        union: &U,
        fields: &Vec<DbPrimitive>,
    ) -> DbResult<Option<DbUnion>> {
        // get all unions with the same ident & fields
        let mut statement = self
            .conn
            .prepare("SELECT uuid FROM unions WHERE ident = ?")?;
        let mut unions = statement
            .query_map([&union.db_ident()], |row| {
                let uuid: String = row.get(0)?;
                let union = self.get_union_from_uuid(uuid);
                union
            })?
            .filter(|db_union| {
                !db_union
                    .as_ref()
                    .is_ok_and(|db_union| db_union.fields.ne(fields))
            });
        let first = unions.next().transpose()?;
        let second = unions.next().transpose()?;
        match (first, second) {
            (None, _) => Ok(None),
            (Some(db_union), None) => Ok(Some(db_union)),
            (Some(db_union_1), Some(db_union_2)) => {
                panic!("duplicate unions in db: {db_union_1:?} {db_union_2:?}")
            }
        }
    }

    pub(crate) fn insert_union<U: AsDbUnion>(
        &self,
        union: &U,
        fields: Vec<DbPrimitive>,
    ) -> DbResult<DbUnion> {
        let db_union = DbUnion::new(union.db_ident(), fields);
        self.insert_db_union(&db_union)?;
        Ok(db_union)
    }

    pub(crate) fn get_union_fields(&self, uuid: &str) -> DbResult<Vec<DbPrimitive>> {
        // get fields, sorted by pos
        let mut statement = self.conn.prepare(
            "SELECT pos, field_uuid FROM union_fields WHERE union_uuid = ? ORDER BY pos",
        )?;
        let fields = statement.query_map([&uuid], |row| {
            let pos: usize = row.get(0)?;
            let field_uuid: String = row.get(1)?;
            Ok((pos, field_uuid))
        })?;
        fields
            .enumerate()
            .map(|(i, row)| {
                let (pos, field_uuid) = row?;
                // todo! get_primitive_from_uuid be infallible
                let field = self.get_primitive_from_uuid(field_uuid)?;
                assert_eq!(pos, i, "malformed db: union_fields.pos is not sorted");
                Ok(field)
            })
            .collect::<DbResult<_>>()
    }

    pub(crate) fn insert_db_union(&self, map: &DbUnion) -> DbResult<()> {
        // add union to db
        let mut statement = self
            .conn
            .prepare("INSERT INTO unions (uuid, ident) VALUES (?, ?)")?;
        let _: usize = statement.execute([&map.uuid, &map.ident])?;
        // add union's fields to db
        let mut statement = self
            .conn
            .prepare("INSERT INTO union_fields (union_uuid, pos, field_uuid) VALUES (?, ?, ?)")?;
        for (pos, field) in map.fields.iter().enumerate() {
            let _: usize = statement.execute(rusqlite::params![&map.uuid, pos, &field.uuid])?;
        }
        Ok(())
    }
}
