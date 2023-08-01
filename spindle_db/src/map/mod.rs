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
        todo!()
    }
}