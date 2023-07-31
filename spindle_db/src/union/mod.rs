use crate::{TypeDb, DbResult, primitive::{DbPrimitive, AsDbPrimitive}};

#[allow(dead_code)]
mod test;

#[derive(Debug, Eq)]
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
    fn db_fields(&self) -> Vec<<Self as AsDbUnion>::Primitive>;
}

const CREATE_TABLE: &str = "
    CREATE TABLE unions (
    uuid TEXT PRIMARY KEY,
    ident TEXT NOT NULL             -- not a unique identifier (there may be multiple unions `U`)
)";
const CREATE_JUNCTION: &str = "
    CREATE TABLE union_fields (
    union_uuid TEXT NOT NULL,
    pos INTEGER NOT NULL,
    field_uuid TEXT NOT NULL,
    PRIMARY KEY (union_uuid, pos)   -- each union has a unique enumerated set of fields
)";
const DROP_TABLE: &str = "DROP TABLE IF EXISTS unions";
const DROP_JUNCTION: &str = "DROP TABLE IF EXISTS union_fields";
const SELECT_UUID: &str = "SELECT unions.uuid FROM unions WHERE unions.ident = ?";
const SELECT_FIELDS: &str = "
    SELECT fields.uuid, fields.ident
    FROM fields
    INNER JOIN union_fields ON fields.uuid = union_fields.field_uuid
    INNER JOIN unions ON unions.uuid = union_fields.union_uuid
    WHERE unions.ident = ?1
    ORDER BY union_fields.pos ASC
";
const INSERT_UNION: &str = "INSERT INTO unions (uuid, ident) VALUES (?1, ?2)";
const JOIN_UNION_FIELD: &str = "
    INSERT INTO union_fields (union_uuid, pos, field_uuid)
    VALUES (?1, ?2, ?3)
";

impl TypeDb {
    pub fn get_or_insert_union<U: AsDbUnion>(&self, union: &U) -> DbResult<DbUnion> {
        let ident = union.db_ident();
        let fields = union.db_fields();
        let fields = fields.iter().map(|p| self.get_or_insert_primitive(p)).collect::<DbResult<Vec<_>>>()?;
        let uuid = self.get_union_uuid(&ident, &fields)?;
        Ok(if let Some(uuid) = uuid {
            DbUnion { uuid, ident, fields }
        } else {
            let union = DbUnion::new(ident, fields);
            self.insert_union(&union)?;
            union
        })
    }
    
    fn get_union_uuid(&self, ident: &str, fields: &Vec<DbPrimitive>) -> DbResult<Option<String>> {
        // todo! what a silly mess 🫣
        let mut statement = self.conn.prepare(SELECT_UUID)?;
        let rows = statement.query_map([&ident], |row| {
            let uuid: String = row.get(0)?;
            let mut statement = self.conn.prepare(SELECT_FIELDS)?;
            let fields = statement.query_map([&uuid], |row| {
                Ok(DbPrimitive {
                    uuid: row.get(0)?,
                    ident: row.get(1)?,
                })
            })?;
            let fields = fields.collect::<DbResult<_>>()?;
            Ok(DbUnion { uuid, ident: ident.to_string(), fields })
        })?;
        let mut rows: Vec<DbUnion> = rows.filter(|r| true).collect::<DbResult<_>>()?;
        rows.retain(|r| {
            r.fields.len() == fields.len() 
            && r.fields.iter().zip(fields.iter()).all(|(a, b)| a == b)
        });
        // todo! this is a hack on top of a hack 🫣 todo! handle this error
        assert!(rows.len() <= 1, "multiple unions with the same ident and fields");
        Ok(rows.into_iter().next().map(|r| r.uuid))
    }

    fn insert_union(&self, union: &DbUnion) -> DbResult<()> {
        // first insert the union with INSERT_UNION
        let mut statement = self.conn.prepare(INSERT_UNION)?;
        statement.execute([&union.uuid, &union.ident])?;
        // then join the union with its fields with JOIN_UNION_FIELD
        let mut statement = self.conn.prepare(JOIN_UNION_FIELD)?;
        for (pos, field) in union.fields.iter().enumerate() {
            statement.execute(rusqlite::params![&union.uuid, pos as i64, &field.uuid])?;
        }
        Ok(())
    }
}