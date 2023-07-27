use crate::db::TypeDb;

impl TypeDb {
    pub fn new_unions(&self) -> Result<(), sqlite::Error> {
        self.conn.execute(
            "DROP TABLE IF EXISTS unions"
        )?;
        
        self.conn.execute(
            "CREATE TABLE unions (
                uuid TEXT NOT NULL PRIMARY KEY,     // Unique identifier
                ident TEXT NOT NULL,                // Union identifier
                span TEXT NOT NULL                  // Span information
            )"
        )
    }
}