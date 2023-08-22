use crate::{DbResult, TypeDb};

const CREATE_TAGS: &str = "
    CREATE TABLE tags (
    tag TEXT NOT NULL PRIMARY KEY
)";

const CREATE_MAPS: &str = "
    CREATE TABLE maps (
    uuid TEXT PRIMARY KEY,
    ident TEXT NOT NULL,        -- not a unique identifier
    content TEXT NOT NULL,      -- todo! ?what uniqueness do we want here
    range_uuid TEXT
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

const CREATE_UNIONS: &str = "
    CREATE TABLE unions (
    uuid TEXT PRIMARY KEY,
    ident TEXT NOT NULL             -- not a unique identifier (there may be multiple unions `U`)
)";

const CREATE_UNION_FIELDS: &str = "
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

// const CREATE_UNION_TAGS: &str = "
//     CREATE TABLE union_tags (
//     union_uuid TEXT NOT NULL,
//     union_ident TEXT NOT NULL,
//     tag TEXT NOT NULL,
//     FOREIGN KEY (union_uuid) REFERENCES unions (uuid),
//     FOREIGN KEY (union_ident) REFERENCES unions (ident),
//     FOREIGN KEY (tag) REFERENCES tags (tag),
//     PRIMARY KEY (union_ident, tag)
// )";

const _CREATE_MAP_TAGS: &str = "
    CREATE TABLE map_tags (
    map_uuid TEXT NOT NULL,
    tag TEXT NOT NULL,
    FOREIGN KEY (map_uuid) REFERENCES maps (uuid),
    FOREIGN KEY (tag) REFERENCES tags (tag),
    PRIMARY KEY (map_uuid, tag)
)";

// caconst DROP: &str = "DROP TABLE IF EXISTS ?";
const DROP_PRIMITIVES: &str = "DROP TABLE IF EXISTS primitives";
const DROP_UNIONS: &str = "DROP TABLE IF EXISTS unions";
const DROP_UNION_FIELDS: &str = "DROP TABLE IF EXISTS union_fields";
const DROP_MAPS: &str = "DROP TABLE IF EXISTS maps";
const DROP_IN_OUTS: &str = "DROP TABLE IF EXISTS in_outs";
const DROP_CRATES: &str = "DROP TABLE IF EXISTS crates";
const DROP_LIFTS: &str = "DROP TABLE IF EXISTS lifts";
const DROP_LIFT_ENTRIES: &str = "DROP TABLE IF EXISTS lift_entries";
const DROP_LIFT_CRATES: &str = "DROP TABLE IF EXISTS lift_crates";
const DROP_CRATE_UNIONS: &str = "DROP TABLE IF EXISTS crate_unions";
const DROP_TAGS: &str = "DROP TABLE IF EXISTS tags";
// const DROP_UNION_TAGS: &str = "DROP TABLE IF EXISTS union_tags";
const DROP_MAP_TAGS: &str = "DROP TABLE IF EXISTS map_tags";

impl TypeDb {
    pub(crate) fn create_tables(&self) -> DbResult<()> {
        self.create_new_map_tables()?;
        self.create_new_union_tables()?;
        self.create_new_primitive_table()?;
        self.create_new_crate_tables()?;
        self.create_new_tag_table()?;
        self.create_new_map_tag_table()?;
        Ok(())
    }

    pub(crate) fn drop_tables(&self) -> DbResult<()> {
        // todo! ?tables in the right order
        const DROP_TABLES: &[&str] = &[
            DROP_MAP_TAGS,
            DROP_TAGS,
            DROP_CRATE_UNIONS,
            DROP_LIFT_CRATES,
            DROP_LIFT_ENTRIES,
            DROP_LIFTS,
            DROP_CRATES,
            DROP_IN_OUTS,
            DROP_MAPS,
            DROP_UNION_FIELDS,
            DROP_UNIONS,
            DROP_PRIMITIVES,
        ];
        DROP_TABLES.iter().try_for_each(|&table| {
            let _: usize = self.conn.execute(table, [])?;
            Ok(())
        })
    }

    pub fn table_names(&self) -> DbResult<Vec<String>> {
        const TABLES: &str =
            "SELECT name FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%'";
        let mut statement = self.conn.prepare(TABLES)?;
        let mut rows = statement.query([])?;
        let mut names = Vec::new();
        while let Some(row) = rows.next()? {
            let name: String = row.get(0)?;
            names.push(name);
        }
        Ok(names)
    }
}

impl TypeDb {
    pub(crate) fn create_new_primitive_table(&self) -> DbResult<()> {
        self.drop_primitive_table()?;
        let _: usize = self.conn.execute(CREATE_PRIMITIVES, [])?;
        Ok(())
    }

    pub(crate) fn drop_primitive_table(&self) -> DbResult<()> {
        let _: usize = self.conn.execute(DROP_PRIMITIVES, [])?;
        Ok(())
    }
}

impl TypeDb {
    pub(crate) fn create_new_union_tables(&self) -> DbResult<()> {
        self.drop_union_tables()?;
        // self.create_new_primitive_table()?;
        let _: usize = self.conn.execute(CREATE_UNIONS, [])?;
        let _: usize = self.conn.execute(CREATE_UNION_FIELDS, [])?;
        Ok(())
    }

    pub(crate) fn drop_union_tables(&self) -> DbResult<()> {
        let _: usize = self.conn.execute(DROP_UNIONS, [])?;
        let _: usize = self.conn.execute(DROP_UNION_FIELDS, [])?;
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
    pub fn create_new_tag_table(&self) -> DbResult<()> {
        self._drop_tag_table()?;
        let _: usize = self.conn.execute(CREATE_TAGS, [])?;
        Ok(())
    }

    pub(crate) fn _drop_tag_table(&self) -> DbResult<()> {
        let _: usize = self.conn.execute(DROP_TAGS, [])?;
        Ok(())
    }

    pub(crate) fn create_new_map_tag_table(&self) -> DbResult<()> {
        self._drop_map_tag_table()?;
        let _: usize = self.conn.execute(_CREATE_MAP_TAGS, [])?;
        Ok(())
    }

    pub(crate) fn _drop_map_tag_table(&self) -> DbResult<()> {
        let _: usize = self.conn.execute(DROP_MAP_TAGS, [])?;
        Ok(())
    }

    // pub(crate) fn create_new_union_tag_table(&self) -> DbResult<()> {
    //     self.drop_union_tag_table()?;
    //     let _: usize = self.conn.execute(CREATE_UNION_TAGS, [])?;
    //     Ok(())
    // }

    // pub(crate) fn drop_union_tag_table(&self) -> DbResult<()> {
    //     let _: usize = self.conn.execute(DROP_UNION_TAGS, [])?;
    //     Ok(())
    // }
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
        let _: usize = self.conn.execute(DROP_CRATES, [])?;
        let _: usize = self.conn.execute(DROP_LIFTS, [])?;
        let _: usize = self.conn.execute(DROP_LIFT_ENTRIES, [])?;
        let _: usize = self.conn.execute(DROP_LIFT_CRATES, [])?;
        let _: usize = self.conn.execute(DROP_CRATE_UNIONS, [])?;
        Ok(())
    }
}
