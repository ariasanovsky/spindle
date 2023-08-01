use crate::{TypeDb, DbResult};

impl TypeDb {
    pub fn new_maps_test_db(test_name: &str) -> DbResult<TypeDb> {
        let db = Self::new_test_db(test_name)?;
        db.create_new_primitive_table()?;
        db.create_new_map_tables()?;
        Ok(db)
    }
}

#[cfg(test)]
mod db_tests {
    use crate::{map::{DbMap, AsDbMap}, TypeDb, PRIMITIVES, MAPS, IN_OUTS};

    impl<'a> AsDbMap for (&'a str, Vec<(Option<&'a str>, Option<&'a str>)>) {
        type Primitive = &'a str;

        fn db_ident(&self) -> String {
            self.0.to_string()
        }

        fn db_inout_pairs(&self) -> Vec<(Option<Self::Primitive>, Option<Self::Primitive>)> {
            self.1.iter().map(|(i, o)| (i.clone(), o.clone())).collect()
        }
    }

    #[test]
    fn maps_new_db_has_correct_table_names() {
        let db = TypeDb::new_maps_test_db("maps_new_db_has_correct_table_names").unwrap();
        let mut names = db.table_names().unwrap();
        names.sort();
        assert_eq!(&names, &[
            IN_OUTS.to_string(),
            MAPS.to_string(),
            PRIMITIVES.to_string(),
        ]);
    }

    #[test]
    fn maps_are_inserted_uniquely() {
        let db = TypeDb::new_maps_test_db("maps_are_added_uniquely").unwrap();
        assert_eq!(db.get_maps().unwrap(), vec![]);
        dbg!(db.get_maps().unwrap());
        let m = db.get_or_insert_map(&("foo", vec![
            (Some("f32"), Some("f32"))
        ])).unwrap();
        assert_eq!(db.get_maps().unwrap(), vec![m.clone()]);
        let n = db.get_or_insert_map(&("foo", vec![
            (Some("f32"), Some("f32"))
        ])).unwrap();
        assert_eq!(db.get_maps().unwrap(), vec![m]);
    }
}