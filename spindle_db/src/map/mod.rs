use crate::{TypeDb, DbResult, primitive::AsDbPrimitive};

#[allow(dead_code)]
mod test;

#[derive(Debug, Eq)]
pub struct DbMap {
    pub uuid: String,
    pub in_field: String,
    pub out_field: String,
}

impl PartialEq for DbMap {
    fn eq(&self, other: &Self) -> bool {
        self.in_field == other.in_field && self.out_field == other.out_field
    }
}

impl DbMap {
    pub(crate) fn new(in_field: String, out_field: String) -> Self {
        Self {
            uuid: TypeDb::new_uuid(),
            in_field,
            out_field,
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

impl TypeDb {
    pub fn create_new_map_tables(&self) -> DbResult<()> {
        self.conn.execute(CREATE_TABLE, [])?;
        Ok(())
    }
}