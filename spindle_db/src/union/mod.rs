use crate::{TypeDb, DbResult, DbIdent, primitive::DbPrimitive};

// todo! distinguish bare unions from explicit ones (e.g., do we need to define union U, or is it in scope?)
pub struct DbUnion {
    pub uuid: String,
    pub ident: String,
    pub fields: Vec<DbPrimitive>,
}

impl TypeDb {
    pub fn new_unions(&self) -> DbResult<()> {
        self.conn.execute(
            "DROP TABLE IF EXISTS unions"
        )?;
        
        // union idents are not unique because they belong to a module
        // however, the combination of a union ident and its fields is unique
        self.conn.execute(
            "CREATE TABLE unions (
                uuid TEXT NOT NULL PRIMARY KEY,     -- Unique identifier
                ident TEXT NOT NULL                 -- Rust identifier
            )"
        )?;

        self.conn.execute(
            "DROP TABLE IF EXISTS union_fields"
        )?;

        self.conn.execute(
            "CREATE TABLE union_fields (
                union_uuid TEXT NOT NULL,           -- Union identifier
                index INTEGER NOT NULL,             -- Field index
                field_uuid TEXT NOT NULL,           -- Field identifier
                PRIMARY KEY (union_uuid, index)
            )"
        )
    }

    pub fn get_or_insert_union<U, P>(&self, u: &U, p: Option<&Vec<P>>) -> DbResult<DbUnion>
    where
        U: DbIdent,
        P: DbIdent,
    {
        let ident = u.db_ident();
        // sieve for unions by ident first
        // then check if the fields match by uuid(!)
        let mut statement = self.conn.prepare(
            "SELECT uuid FROM unions WHERE ident = ?"
        )?;
        statement.bind((1, ident.as_str()))?;
        while let sqlite::State::Row = statement.next()? {
            let uuid: String = statement.read(0)?;
            let fields = self.get_union_fields(&uuid)?;
            // since primitives have unique idents, we can verify the fields by ident
            if let Some(p) = p {
                // if the fields match, return the union
                if fields.iter().zip(p.iter()).all(|(f, p)| f.ident == p.db_ident()) {
                    return Ok(DbUnion { uuid, ident, fields })
                }
            } else {
                // bare unions only make sense when they are unique in the database w.r.t. the host scope
                // this is tricky because we don't encode the host scope in the database
                // encoding the file itself is a nightly feature (we'd lose semver with proc-macro2)
                // perhaps we should? or is it better to just disallow bare unions?
                // should names be unique in the database?
                return Err(sqlite::Error {
                    code: None, // todo! what code for not found?
                    message: Some(format!("`spin!({ident})` is not supported. Please use `spin!({ident} = f32 | u64)`, for example."))
                })
            }
        }
        // if no union was found, the database is in an invalid state
        Err(sqlite::Error {
            code: None, // todo! what code for not found?
            message: Some(format!("Fatal db error: union {ident} not found")),
        })
    }

    pub fn get_union_fields(&self, uuid: &str) -> DbResult<Vec<DbPrimitive>> {
        // get the index and field uuids for the union
        let mut statement = self.conn.prepare(
            "SELECT index, field_uuid FROM union_fields WHERE union_uuid = ?"
        )?;
        statement.bind((1, uuid))?;
        let mut fields = Vec::new();
        while let sqlite::State::Row = statement.next()? {
            let index: i64 = statement.read(0)?;
            let field_uuid: String = statement.read(1)?;
            let field = self.get_primitive_from_uuid(&field_uuid)?;
            fields.push((index, field));
        }
        // sort the fields by index
        fields.sort_by(|(i1, _), (i2, _)| i1.cmp(i2));
        // verify index by enumeration
        fields.into_iter().enumerate().map(|(i, (index, field))| {
            if i as i64 != index {
                Err(sqlite::Error {
                    code: None, // todo! what code for index mismatch?
                    message: Some("index mismatch".to_string()),
                })
            } else {
                Ok(field)
            }
        }).collect()
    }
}