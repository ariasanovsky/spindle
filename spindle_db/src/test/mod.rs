use std::path::PathBuf;

use crate::{TypeDb, HOME, DbResult, DB, TABLES};

const TEST: &str = "tests";
const PRIMITIVES: &str = "primitives";

impl TypeDb {
    fn table_names(&self) -> DbResult<Vec<String>> {
        let mut statement = self.conn.prepare(TABLES)?;
        let mut rows = statement.query([])?;
        let mut names = Vec::new();
        while let Some(row) = rows.next()? {
            let name: String = row.get(0)?;
            names.push(name);
        }
        Ok(names)
    }

    fn new_test_db(name: &str) -> DbResult<TypeDb> {
        let path = PathBuf::from(HOME).join(TEST).join(name).with_extension(DB);
        let db = TypeDb::new(path)?;
        Ok(db)
    }

    fn new_primitives_test_db() -> DbResult<TypeDb> {
        let db = Self::new_test_db(PRIMITIVES)?;
        db.create_new_primitive_table()?;
        Ok(db)
    }
}

#[cfg(test)]
mod db_tests {
    use crate::TypeDb;
    use super::*;

    #[test]
    fn new_primitves_db_table_names() {
        let db = TypeDb::new_primitives_test_db().unwrap();
        let mut names = db.table_names().unwrap();
        names.sort();
        assert_eq!(&names, &["primitives"]);
    }

    
    #[test]
    fn db_test_primitive() {
        let db = TypeDb::new_primitives_test_db();
        // let p = db.get_or_insert_primitive(&"f32").unwrap();
        // assert_eq!(p.ident, "f32");
        // let p = db.get_primitive_from_uuid(&p.uuid).unwrap();
        // assert_eq!(p.ident, "f32");
        // let q = db.get_or_insert_primitive(&"f32").unwrap();
        // assert_eq!(p.uuid, q.uuid);
        // let r = db.get_or_insert_primitive(&"f64").unwrap();
        // assert_ne!(p.uuid, r.uuid);
        // db.drop_all().unwrap();
    }

    // #[test]
    // fn db_test_union() {
    //     let db = TypeDb::new_test_unions().unwrap();
    //     // todo! is this good design?
    //     // I assume that the database is used in a specific order
    //     // first primitives are addeed, then unions
    //     // this is helpful because it helps me regulate the valid state of the database
    //     // the parse method ?will automatically add primitives to the database
    //     // therefore the union parse method will not need to add primitives to the database
    //     // this is bad because it is not obvious to the user (me, for now)
    //     // add `f32` and `u64` to the database
    //     dbg!("adding primitives");
    //     db.get_or_insert_primitive(&"f32").unwrap();
    //     dbg!("added f32");
    //     db.get_or_insert_primitive(&"u64").unwrap();
    //     dbg!("added u64");
    //     let u = db.get_or_insert_union(&"U", Some(&vec!["f32", "u64"])).unwrap();
    //     dbg!("added union");
    //     assert_eq!(u.ident, "U");
    //     let u = db.get_union_from_uuid(&u.uuid).unwrap();
    //     dbg!("got union");
    //     assert_eq!(u.ident, "U");
    //     let v = db.get_or_insert_union(&"U", None::<&Vec<&str>>).unwrap();    // todo! ergonomics, typestate database
    //     assert_ne!(u.uuid, v.uuid);
    //     // let w = db.get_or_insert_union(&"Bar").unwrap();
    //     // assert_ne!(u.uuid, w.uuid);
    //     // db.drop_unions().unwrap();
    // }
}
