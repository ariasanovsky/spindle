use crate::{TypeDb, DbResult};

impl TypeDb {
    fn new_crates_test_db(test_name: &str) -> DbResult<TypeDb> {
        let db = Self::new_test_db(test_name)?;
        db.drop_tables()?;
        db.create_new_primitive_table()?;
        db.create_new_union_tables()?;
        db.create_new_map_tables()?;
        db.create_new_crate_tables()?;
        Ok(db)
    }
}

#[cfg(test)]
mod db_tests {
    use crate::{TypeDb, PRIMITIVES, UNION_FIELDS, UNIONS, MAPS, IN_OUTS, CRATES, LIFT_ENTRIES, LIFTS, CRATE_UNIONS, LIFT_CRATES};

    #[test]
    fn spindle_crate_new_db_has_correct_table_names() {
        let db = TypeDb::new_crates_test_db("spindle_crate_new_db_has_correct_table_names").unwrap();
        let mut names = db.table_names().unwrap();
        names.sort();
        assert_eq!(&names, &[
            CRATE_UNIONS.to_string(),
            CRATES.to_string(),
            IN_OUTS.to_string(),
            LIFT_CRATES.to_string(),
            LIFT_ENTRIES.to_string(),
            LIFTS.to_string(),
            MAPS.to_string(),
            PRIMITIVES.to_string(),
            UNION_FIELDS.to_string(),
            UNIONS.to_string(),
        ]);
    }

    #[test]
    fn can_form_a_crate_from_unions() {
        let db = TypeDb::new_crates_test_db("can_form_a_crate_from_unions").unwrap();
        let u = db.get_or_insert_union(&("U", vec!["f32"])).unwrap();
        let v = db.get_or_insert_union(&("V", vec!["f32", "u64"])).unwrap();
        assert_eq!(db.get_unions().unwrap().len(), 2);
        let m = db.get_or_insert_map(&("pub fn foo(u64) -> f32;", vec![
            (Some("u64"), Some("f32")),
        ])).unwrap();
        let n = db.get_or_insert_map(&("pub fn bar(f32, u64) -> (f32, ());", vec![
            (Some("f32"), Some("f32")),
            (Some("u64"), None),
        ])).unwrap();
        assert_eq!(db.get_maps().unwrap().len(), 2);
        // let c = db.get_or_insert_crate_from_unions(vec![u.clone()]).unwrap();
        // assert_eq!(c.unions.len(), 1);
        // assert_eq!(c.lifters.len(), 0);
        // let d = db.get_or_insert_crate_from_unions(vec![v.clone()]).unwrap();
        // assert_eq!(d.unions.len(), 1);
        // assert_eq!(d.lifters.len(), 1);
        let p = db.get_or_insert_map(&("pub fn baz(f32, u64) -> ((), f32);", vec![
            (Some("f32"), None),
            (Some("u64"), Some("f32")),
        ])).unwrap();
        assert_eq!(db.get_maps().unwrap().len(), 3);
        
        let e = db.get_or_insert_crate_from_unions(vec![u, v]).unwrap();
        assert_eq!(e.unions.len(), 2);
        assert_eq!(e.lifts.len(), 2);

        println!("{e}");
    }
}
