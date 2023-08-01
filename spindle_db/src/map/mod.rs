use crate::{TypeDb, DbResult, primitive::{AsDbPrimitive, DbPrimitive}};

#[allow(dead_code)]
mod test;

#[derive(Debug, Eq)]
pub struct DbMap {
    pub uuid: String,
    pub in_outs: Vec<(Option<DbPrimitive>, Option<DbPrimitive>)>,
}

impl PartialEq for DbMap {
    fn eq(&self, other: &Self) -> bool {
        self.in_outs == other.in_outs
    }
}

impl DbMap {
    pub(crate) fn new(in_outs: Vec<(Option<DbPrimitive>, Option<DbPrimitive>)>) -> Self {
        Self {
            uuid: TypeDb::new_uuid(),
            in_outs,
        }
    }
}

pub trait AsDbMap {
    type Primitive: AsDbPrimitive;
    fn db_ident(&self) -> String;
    fn db_inout_pairs(&self) -> Vec<(Option<Self::Primitive>, Option<Self::Primitive>)>;
}

const CREATE_TABLE: &str = "
    CREATE TABLE maps (
    uuid TEXT PRIMARY KEY,
    ident TEXT NOT NULL         -- not a unique identifier
)";
const CREATE_JUNCTION: &str = "
    CREATE TABLE map_fields (
    map_uuid TEXT NOT NULL,
    pos INTEGER NOT NULL,
    input_uuid TEXT,            -- NULL if the field is ()/None
    output_uuid TEXT,           -- NULL if the field is ()/None
    FOREIGN KEY (map_uuid) REFERENCES maps (uuid),
    FOREIGN KEY (input_uuid) REFERENCES primitives (uuid),
    FOREIGN KEY (output_uuid) REFERENCES primitives (uuid),
    PRIMARY KEY (map_uuid, pos)
)";

const DROP_TABLE: &str = "DROP TABLE IF EXISTS maps";
const DROP_JUNCTION: &str = "DROP TABLE IF EXISTS map_fields";

impl TypeDb {
    pub(crate) fn create_new_map_tables(&self) -> DbResult<()> {
        self.drop_map_tables()?;
        self.conn.execute(CREATE_TABLE, [])?;
        self.conn.execute(CREATE_JUNCTION, [])?;
        Ok(())
    }

    pub(crate) fn drop_map_tables(&self) -> DbResult<()> {
        self.conn.execute(DROP_TABLE, [])?;
        self.conn.execute(DROP_JUNCTION, [])?;
        Ok(())
    }

    pub(crate) fn get_maps(&self) -> DbResult<Vec<DbMap>> {
        let mut stmt = self.conn.prepare("SELECT uuid FROM maps")?;
        let uuids = stmt.query_map([], |row| row.get(0))?;
        let mut maps = Vec::new();
        for uuid in uuids {
            let uuid: String = uuid?;
            maps.push(self.get_map_from_uuid(uuid)?);
        }
        Ok(maps)
    }

    pub(crate) fn get_map_from_uuid(&self, uuid: String) -> DbResult<DbMap> {
        todo!("get_map_from_uuid")
    }

    pub(crate) fn get_or_insert_map<M: AsDbMap>(&self, map: &M) -> DbResult<DbMap> {
        let ident = map.db_ident();
        let mut statement = self.conn.prepare("SELECT uuid FROM maps WHERE ident = ?")?;
        let uuids = statement.query_map([ident], |row| row.get::<_, String>(0))?;
        // todo! `filter`
        let uuids: Vec<String> = uuids.collect::<Result<_, _>>()?;
        let pairs = map.db_inout_pairs();
        let maps: Vec<Vec<_>> = uuids.into_iter().map(|uuid| {
            // get the position, input_uuid, and output_uuid but sort by position
            let mut statement = self.conn.prepare(
                "SELECT pos, input_uuid, output_uuid FROM map_fields WHERE map_uuid = ? ORDER BY pos",
            )?;
            let fields = statement.query_map([uuid], |row| {
                let pos: i64 = row.get(0)?;
                let input_uuid: Option<String> = row.get(1)?;
                let output_uuid: Option<String> = row.get(2)?;
                Ok((pos, input_uuid, output_uuid))
            })?;
            let fields: Vec<_> = fields
                .enumerate()
                .map(|(i, x)| {
                    let (pos, input_uuid, output_uuid) = x?;
                    assert_eq!(i as i64, pos);
                    Ok((input_uuid, output_uuid))
                })
                .collect::<DbResult<Vec<_>>>()?;
            let fields: Vec<_> = fields.into_iter().map(|(input, output)| {
                let input: Option<DbPrimitive> = input.map(|uuid| self.get_primitive_from_uuid(uuid))
                    .transpose()?.unwrap(); // todo! we could `flatten`, but the error is fatal in this case
                let output: Option<DbPrimitive> = output.map(|uuid| self.get_primitive_from_uuid(uuid))
                    .transpose()?.unwrap(); // todo! we could `flatten`, but the error is fatal in this case
                Ok((input, output))
            }).collect::<DbResult<_>>()?;
            Ok(fields)
        }).collect::<DbResult<_>>()?;
        // todo! crashes on fatal error, db malformed
        assert!(maps.len() <= 1, "more than one map with the same ident and in_outs");
        if let Some(map) = maps.into_iter().next() {
            todo!()
        } else {
            todo!("insert map")
        }
    }
}