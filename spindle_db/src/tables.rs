use crate::{TypeDb, DbResult};

const CREATE_MAPS: &str = "
    CREATE TABLE maps (
    uuid TEXT PRIMARY KEY,
    ident TEXT NOT NULL         -- not a unique identifier
)";

const CREATE_IN_OUTS: &str = "
    CREATE TABLE in_outs (
    map_uuid TEXT NOT NULL,
    pos INTEGER NOT NULL,
    input_uuid TEXT,            -- NULL if the field is ()/None
    output_uuid TEXT,           -- NULL if the field is ()/None
    FOREIGN KEY (map_uuid) REFERENCES maps (uuid),
    FOREIGN KEY (input_uuid) REFERENCES primitives (uuid),
    FOREIGN KEY (output_uuid) REFERENCES primitives (uuid),
    PRIMARY KEY (map_uuid, pos)
)";


const CREATE_TABLE: &str = "
    CREATE TABLE primitives (
    uuid TEXT PRIMARY KEY,
    ident TEXT NOT NULL UNIQUE  -- unique identifier (there is only one `i32`)
)";

const DROP_TABLE: &str = "DROP TABLE IF EXISTS maps";
const DROP_JUNCTION: &str = "DROP TABLE IF EXISTS in_outs";

impl TypeDb {
    pub(crate) fn create_new_primitive_table(&self) -> DbResult<()> {
        dbg!();
        self.drop_primitive_table()?;
        let _: usize = self.conn.execute(CREATE_TABLE, [])?;
        Ok(())
    }
    
    pub(crate) fn create_new_map_tables(&self) -> DbResult<()> {
        self.drop_map_tables()?;
        let _: usize = self.conn.execute(CREATE_MAPS, [])?;
        let _: usize = self.conn.execute(CREATE_IN_OUTS, [])?;
        Ok(())
    }

    pub(crate) fn drop_map_tables(&self) -> DbResult<()> {
        let _: usize = self.conn.execute(DROP_TABLE, [])?;
        let _: usize = self.conn.execute(DROP_JUNCTION, [])?;
        Ok(())
    }
}