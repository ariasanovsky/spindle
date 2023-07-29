use crate::{TypeDb, DbResult, DbIdent, primitive::DbPrimitive};

// todo! distinguish bare unions from explicit ones (e.g., do we need to define union U, or is it in scope?)
#[derive(Debug)]
pub struct DbUnion {
    pub uuid: String,
    pub ident: String,
    pub fields: Vec<DbPrimitive>,
}

impl TypeDb {
    pub fn new_unions(&self) -> DbResult<()> {
        dbg!("new unions");
        self.conn.execute(
            "DROP TABLE IF EXISTS unions"
        )?;
        dbg!("dropped unions");
        // union idents are not unique because they belong to a module
        // however, the combination of a union ident and its fields is unique
        self.conn.execute(
            "CREATE TABLE unions (
                uuid TEXT NOT NULL PRIMARY KEY,     -- Unique identifier
                ident TEXT NOT NULL                 -- Rust identifier
            )"
        )?;
        dbg!("created unions");

        self.conn.execute(
            "DROP TABLE IF EXISTS union_fields"
        )?;
        dbg!("dropped union_fields");
        self.conn.execute(
            "CREATE TABLE union_fields (
                union_uuid TEXT NOT NULL,           -- Union identifier
                pos INTEGER NOT NULL,               -- Field index
                field_uuid TEXT NOT NULL,           -- Field identifier
                PRIMARY KEY (union_uuid, pos)
            )"
        )?;
        dbg!("created union_fields");
        Ok(())
    }

    pub fn get_or_insert_union<U, P>(&self, u: &U, p: Option<&Vec<P>>) -> DbResult<DbUnion>
    where
        U: DbIdent,
        P: DbIdent,
    {
        let ident = u.db_ident();
        dbg!(&ident);
        // sieve for unions by ident first
        // then check if the fields match by uuid(!)
        let mut statement = self.conn.prepare(
            "SELECT uuid FROM unions WHERE ident = ?"
        )?;
        statement.bind((1, ident.as_str()))?;
        while let sqlite::State::Row = statement.next()? {
            let uuid: String = statement.read(0)?;
            dbg!(&uuid);
            let db_u = self.get_union_from_uuid(&uuid)?;
            dbg!(&db_u.ident);
            // since primitives have unique idents, we can verify the fields by ident
            if let Some(p) = p {
                // if the fields match, return the union
                if db_u.fields.iter().zip(p.iter()).all(|(f, p)| f.ident == p.db_ident()) {
                    return Ok(DbUnion { uuid, ident, fields: db_u.fields })
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
        // if no union was found, hopefully we came with fields
        match p {
            Some(p) => Ok({
                // insert the union
                dbg!("inserting union");
                let uuid = uuid::Uuid::new_v4().to_string();
                let mut statement = self.conn.prepare(
                    "INSERT INTO unions (uuid, ident) VALUES (?, ?)"
                )?;
                statement.bind((1, uuid.as_str()))?;
                statement.bind((2, ident.as_str()))?;
                statement.next()?;
                dbg!("inserted union");
                // insert the fields
                let fields = p.iter().enumerate().map(|(i, p)| {
                    let mut insert_statement = self.conn.prepare(
                        "INSERT INTO union_fields (union_uuid, pos, field_uuid) VALUES (?, ?, ?)"
                    )?;
                    let field_uuid = uuid::Uuid::new_v4().to_string();
                    insert_statement.bind((1, uuid.as_str()))?;
                    insert_statement.bind((2, i as i64))?;
                    insert_statement.bind((3, field_uuid.as_str()))?;
                    insert_statement.next()?;
                    let p = DbPrimitive { uuid: field_uuid, ident: p.db_ident() };
                    dbg!(&p);
                    Ok(p)
                }).collect::<Result<_, _>>()?;
                dbg!("returning union");
                DbUnion { uuid, ident, fields }
            }),
            None => Err(sqlite::Error {
                code: None, // todo! what code for not found?
                message: Some(format!("`spin!({ident})` is not supported. Please use `spin!({ident} = f32 | u64)`, for example."))
            })
        }
    }

    pub fn get_union_from_uuid(&self, uuid: &str) -> DbResult<DbUnion> {
        dbg!("get union from uuid");
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
        let fields = fields.into_iter().enumerate().map(|(i, (index, field))| {
            if i as i64 != index {
                Err(sqlite::Error {
                    code: None, // todo! what code for index mismatch?
                    message: Some("index mismatch".to_string()),
                })
            } else {
                Ok(field)
            }
        }).collect::<Result<_, _>>()?;
        // get the union ident
        let mut statement = self.conn.prepare(
            "SELECT ident FROM unions WHERE uuid = ?"
        )?;
        statement.bind((1, uuid))?;
        let ident: String = statement.read(0)?;
        // return the union
        Ok(DbUnion { uuid: uuid.to_string(), ident, fields })
    }
}