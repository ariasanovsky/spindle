use crate::db::TypeDb;

impl TypeDb {
    pub fn new_maps(&self) -> Result<(), sqlite::Error> {
        self.conn.execute(
            "DROP TABLE IF EXISTS maps"
        )?;
        
        self.conn.execute(
            "CREATE TABLE maps (
                uuid TEXT NOT NULL PRIMARY KEY,     // Unique identifier
                ident TEXT NOT NULL,                // Map identifier
                span TEXT NOT NULL                  // Span information (only the ident)
            )"
        )
    }
}