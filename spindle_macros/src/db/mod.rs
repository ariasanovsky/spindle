pub struct TypeDb {
    // pub conn: sqlite::Connection,
}

impl TypeDb {
    pub fn connect() -> Result<Self, sqlite::Error> {
        todo!()
        // sqlite::open(HOME).map(|conn| Self { conn })
    }
    
    pub fn new<P: Into<PathBuf>>(name: &str, home: P) -> Result<Self, sqlite::Error> {
        todo!()
        // sqlite::open(path).map(|conn| Self { conn })
    }
}
