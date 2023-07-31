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

const DROP_TABLE: &str = "DROP TABLE IF EXISTS unions";
const DROP_JUNCTION: &str = "DROP TABLE IF EXISTS union_fields";

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
    FOREIGN KEY (union_uuid) REFERENCES unions (uuid),
    FOREIGN KEY (field_uuid) REFERENCES primitives (uuid),
    PRIMARY KEY (union_uuid, pos)
)";

const INSERT_UNION: &str = "INSERT INTO unions (uuid, ident) VALUES (?1, ?2)";
const INSERT_UNION_FIELD: &str = "
    INSERT INTO union_fields (union_uuid, pos, field_uuid)
    VALUES (?1, ?2, ?3)
";
const SELECT_UUID: &str = "SELECT unions.uuid FROM unions WHERE unions.ident = ?";




// from a union's uuid, we get the uuids, idents, and positions of its fields
const SELECT_FIELDS: &str = "
    SELECT union_fields.pos, primitives.uuid, primitives.ident
    FROM union_fields
    INNER JOIN primitives ON union_fields.field_uuid = primitives.uuid
    WHERE union_fields.union_uuid = ?
    ORDER BY union_fields.pos
";











const SELECT_UNION : &str = "SELECT uuid, ident FROM unions";
const SELECT_UNION_FIELDS: &str = "
    SELECT * FROM union_fields
";

impl TypeDb {
    pub fn get_or_insert_union<U: AsDbUnion>(&self, union: &U) -> DbResult<DbUnion> {
        let ident = union.db_ident();
        dbg!(&ident);
        let fields = union.db_fields();
        let fields = fields.iter().map(|p| self.get_or_insert_primitive(p)).collect::<DbResult<Vec<_>>>()?;
        dbg!(&fields);
        let uuid = self.get_union_uuid(&ident, &fields)?;
        // todo! unwrap_or* is more idiomatic
        Ok(if let Some(uuid) = uuid {
            dbg!(&uuid);
            DbUnion { uuid, ident, fields }
        } else {
            let union = DbUnion::new(ident, fields);
            dbg!(&union);
            self.insert_union(&union)?;
            union
        })
    }

    pub fn get_union_from_uuid(&self, uuid: String) -> DbResult<Option<DbUnion>> {
        let mut statement = self.conn.prepare(SELECT_UUID)?;
        let mut rows = statement.query([&uuid])?;
        // todo! `map` is more idiomatic
        if let Some(row) = rows.next()? {
            let uuid: String = row.get(0)?; // todo! redundant
            let ident: String = row.get(1)?;
            self.get_union_from_uuid_and_ident(uuid, ident).map(Some)
        } else {
            Ok(None)
        }
    }
    
    pub(crate) fn get_union_from_uuid_and_ident(&self, uuid: String, ident: String) -> DbResult<DbUnion> {
        let mut statement = self.conn.prepare(SELECT_FIELDS)?;
        let fields = statement.query_map([&uuid], |row| {
            let pos: i64 = row.get(0)?;
            let uuid: String = row.get(1)?;
            let ident: String = row.get(2)?;
            Ok((pos, DbPrimitive { uuid, ident }))
        })?.enumerate().map(|(i, x)| {
            x.map(|(pos, x)| {
                assert_eq!(i as i64, pos);  // todo! handle this error
                x
            })
        });
        let fields = fields.collect::<DbResult<_>>()?;
        Ok(DbUnion { uuid, ident, fields })
    }
    
    pub(crate) fn get_unions(&self) -> DbResult<Vec<DbUnion>> {
        let mut statement = self.conn.prepare(SELECT_UNION)?;
        let rows = statement.query_map([], |row| {
            let uuid: String = row.get(0)?;
            let ident: String = row.get(1)?;
            self.get_union_from_uuid_and_ident(uuid, ident)
        })?;
        rows.collect::<DbResult<_>>()
    }

    pub(crate) fn get_union_fields(&self) -> DbResult<Vec<(String, i64, String)>> {
        let mut statement = self.conn.prepare(SELECT_UNION_FIELDS)?;
        let rows = statement.query_map([], |row| {
            let union_uuid: String = row.get(0)?;
            let pos: i64 = row.get(1)?;
            let field_uuid: String = row.get(2)?;
            Ok((union_uuid, pos, field_uuid))
        })?;
        rows.collect::<DbResult<_>>()
    }
    
    pub(crate) fn create_new_union_tables(&self) -> DbResult<()> {
        self.drop_union_tables()?;
        self.create_new_primitive_table()?;
        let _: usize = self.conn.execute(CREATE_TABLE, [])?;
        let _: usize = self.conn.execute(CREATE_JUNCTION, [])?;
        Ok(())
    }

    pub(crate) fn drop_union_tables(&self) -> DbResult<()> {
        self.conn.execute(DROP_TABLE, [])?;
        self.conn.execute(DROP_JUNCTION, [])?;
        Ok(())
    }
    
    fn get_union_uuid(&self, ident: &str, fields: &Vec<DbPrimitive>) -> DbResult<Option<String>> {
        // todo! ?overkill
        dbg!(&ident);
        let mut statement = self.conn.prepare(SELECT_UUID)?;
        let rows = statement.query_map([&ident], |row| {
            dbg!(&row);
            let uuid: String = row.get(0)?;
            dbg!(&uuid);
            let union = self.get_union_from_uuid_and_ident(uuid, ident.to_string())?;
            dbg!(&union);
            Ok(union)
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
        let mut statement = self.conn.prepare(INSERT_UNION_FIELD)?;
        for (pos, field) in union.fields.iter().enumerate() {
            statement.execute(rusqlite::params![&union.uuid, pos as i64, &field.uuid])?;
        }
        Ok(())
    }
}