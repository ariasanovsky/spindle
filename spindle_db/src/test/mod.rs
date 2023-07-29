#[cfg(test)]
mod db_tests {
    use crate::{TypeDb, DbIdent};

    impl DbIdent for &str {
        fn db_ident(&self) -> String {
            self.to_string()
        }
    }
    
    #[test]
    fn db_test_primitive() {
        let db = TypeDb::new_test_primitives().unwrap();
        let p = db.get_or_insert_primitive(&"f32").unwrap();
        assert_eq!(p.ident, "f32");
        let p = db.get_primitive_from_uuid(&p.uuid).unwrap();
        assert_eq!(p.ident, "f32");
        let q = db.get_or_insert_primitive(&"f32").unwrap();
        assert_eq!(p.uuid, q.uuid);
        let r = db.get_or_insert_primitive(&"f64").unwrap();
        assert_ne!(p.uuid, r.uuid);
        db.drop_primitives().unwrap();
    }

    #[test]
    fn db_test_union() {
        let db = TypeDb::new_test_unions().unwrap();
        // todo! is this good design?
        // I assume that the database is used in a specific order
        // first primitives are addeed, then unions
        // this is helpful because it helps me regulate the valid state of the database
        // the parse method ?will automatically add primitives to the database
        // therefore the union parse method will not need to add primitives to the database
        // this is bad because it is not obvious to the user (me, for now)
        // add `f32` and `u64` to the database
        dbg!("adding primitives");
        db.get_or_insert_primitive(&"f32").unwrap();
        dbg!("added f32");
        db.get_or_insert_primitive(&"u64").unwrap();
        dbg!("added u64");
        let u = db.get_or_insert_union(&"U", Some(&vec!["f32", "u64"])).unwrap();
        dbg!("added union");
        assert_eq!(u.ident, "U");
        let u = db.get_union_from_uuid(&u.uuid).unwrap();
        dbg!("got union");
        assert_eq!(u.ident, "U");
        let v = db.get_or_insert_union(&"U", None::<&Vec<&str>>).unwrap();    // todo! ergonomics, typestate database
        assert_ne!(u.uuid, v.uuid);
        // let w = db.get_or_insert_union(&"Bar").unwrap();
        // assert_ne!(u.uuid, w.uuid);
        // db.drop_unions().unwrap();
    }
}
