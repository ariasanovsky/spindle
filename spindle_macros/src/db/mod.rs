const HOME: &str = ".spindle/values.db";

pub struct TypeDb {
    // pub conn: sqlite::Connection,
}

impl TypeDb {
    pub fn connect() -> Result<Self, sqlite::Error> {
        todo!()
        // sqlite::open(HOME).map(|conn| Self { conn })
    }
}
