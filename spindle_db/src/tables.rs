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

const CREATE_PRIMITIVES: &str = "
    CREATE TABLE primitives (
    uuid TEXT PRIMARY KEY,
    ident TEXT NOT NULL UNIQUE  -- unique identifier (there is only one `i32`)
)";

const CREATE_CRATES: &str = "
    CREATE TABLE crates (
    uuid TEXT PRIMARY KEY
)";

const CREATE_LIFTS: &str = "
    CREATE TABLE lifts (
    uuid TEXT PRIMARY KEY,
    map_uuid TEXT NOT NULL
    )
";

const CREATE_LIFT_ENTRIES: &str = "
    CREATE TABLE lift_entries (
    lift_uuid TEXT NOT NULL,    -- unique id
    pos INTEGER,                -- (i = pos) s.t. X_i, Y_i are fields of U_i
    union_uuid TEXT NOT NULL,   -- U_i
    in_pos INTEGER,             -- (x_i = in_pos) s.t. X_i is the x_i-th field of U_i
    out_pos INTEGER,            -- (y_i = out_pos) s.t. Y_i is the y_i-th field of U_i
    FOREIGN KEY (lift_uuid) REFERENCES lifts (uuid),
    FOREIGN KEY (union_uuid) REFERENCES unions (uuid),
    PRIMARY KEY (lift_uuid, pos)    -- each lift has at most one associated at position pos
)";

const CREATE_LIFT_CRATES: &str = "
    CREATE TABLE lift_crates (
    lift_uuid TEXT NOT NULL,
    crate_uuid TEXT NOT NULL,
    pos INTEGER NOT NULL,
    FOREIGN KEY (lift_uuid) REFERENCES lifts (uuid),
    FOREIGN KEY (crate_uuid) REFERENCES crates (uuid),
    PRIMARY KEY (lift_uuid, pos)    -- each lift has at most one associated crate
)";

const CREATE_CRATE_UNIONS: &str = "
    CREATE TABLE crate_unions (
    crate_uuid TEXT NOT NULL,
    union_uuid TEXT NOT NULL,
    pos INTEGER NOT NULL,
    FOREIGN KEY (crate_uuid) REFERENCES crates (uuid),
    FOREIGN KEY (union_uuid) REFERENCES unions (uuid),
    PRIMARY KEY (crate_uuid, pos)    -- each crate has at most one associated union
)";

const DROP_MAPS: &str = "DROP TABLE IF EXISTS maps";
const DROP_IN_OUTS: &str = "DROP TABLE IF EXISTS in_outs";
const DROP_PRIMITIVES: &str = "DROP TABLE IF EXISTS primitives";


impl TypeDb {
    pub(crate) fn create_new_primitive_table(&self) -> DbResult<()> {
        dbg!();
        self.drop_primitive_table()?;
        let _: usize = self.conn.execute(CREATE_PRIMITIVES, [])?;
        Ok(())
    }

    pub(crate) fn drop_primitive_table(&self) -> DbResult<()> {
        dbg!();
        let _: usize = self.conn.execute(DROP_TABLE, [])?;
        Ok(())
    }
}

impl TypeDb {
    pub(crate) fn create_new_union_tables(&self) -> DbResult<()> {
        self.drop_union_tables()?;
        // self.create_new_primitive_table()?;
        let _: usize = self.conn.execute(CREATE_TABLE, [])?;
        let _: usize = self.conn.execute(CREATE_JUNCTION, [])?;
        Ok(())
    }

    pub(crate) fn drop_union_tables(&self) -> DbResult<()> {
        let _: usize = self.conn.execute(DROP_TABLE, [])?;
        let _: usize = self.conn.execute(DROP_JUNCTION, [])?;
        Ok(())
    }
}

impl TypeDb {
    pub(crate) fn create_new_map_tables(&self) -> DbResult<()> {
        self.drop_map_tables()?;
        let _: usize = self.conn.execute(CREATE_MAPS, [])?;
        let _: usize = self.conn.execute(CREATE_IN_OUTS, [])?;
        Ok(())
    }

    pub(crate) fn drop_map_tables(&self) -> DbResult<()> {
        let _: usize = self.conn.execute(DROP_MAPS, [])?;
        let _: usize = self.conn.execute(DROP_IN_OUTS, [])?;
        Ok(())
    }
}

impl TypeDb {
    pub(crate) fn create_new_crate_tables(&self) -> DbResult<()> {
        self.drop_crate_tables()?;
        let _: usize = self.conn.execute(CREATE_CRATES, [])?;
        let _: usize = self.conn.execute(CREATE_LIFTS, [])?;
        let _: usize = self.conn.execute(CREATE_LIFT_ENTRIES, [])?;
        let _: usize = self.conn.execute(CREATE_LIFT_CRATES, [])?;
        let _: usize = self.conn.execute(CREATE_CRATE_UNIONS, [])?;
        Ok(())
    }

    pub(crate) fn drop_crate_tables(&self) -> DbResult<()> {
        let _: usize = self.conn.execute("DROP TABLE IF EXISTS crates", [])?;
        let _: usize = self.conn.execute("DROP TABLE IF EXISTS lifts", [])?;
        let _: usize = self.conn.execute("DROP TABLE IF EXISTS lift_positions", [])?;
        let _: usize = self.conn.execute("DROP TABLE IF EXISTS lift_crates", [])?;
        let _: usize = self.conn.execute("DROP TABLE IF EXISTS crate_unions", [])?;
        Ok(())
    }
}