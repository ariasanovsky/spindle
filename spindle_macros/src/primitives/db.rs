use crate::db::TypeDb;

impl TypeDb {
    pub fn new_primitives(&self) -> Result<(), sqlite::Error> {
        self.conn.execute(
            "DROP TABLE IF EXISTS primitives"
        )?;
        
        self.conn.execute(
            "CREATE TABLE primitives (
                uuid TEXT NOT NULL PRIMARY KEY,     // Unique identifier
                ident TEXT NOT NULL,                // Primitive identifier
                span TEXT NOT NULL                  // Span information
            )"
        )
    }
}