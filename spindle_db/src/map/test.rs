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
    use crate::{map::{DbMap, AsDbMap}, TypeDb, PRIMITIVES, MAPS};

    impl<'a> AsDbMap for (&'a str, &'a str) {
        type Primitive = &'a str;

        fn db_ident(&self) -> String {
            todo!()
        }

        fn db_inout_pairs(&self) -> Vec<(Option<Self::Primitive>, Option<Self::Primitive>)> {
            todo!()
        }
    }

    #[test]
    fn maps_new_db_has_correct_table_names() {
        let db = TypeDb::new_maps_test_db("maps_new_db_has_correct_table_names").unwrap();
        let mut names = db.table_names().unwrap();
        names.sort();
        assert_eq!(&names, &[
            MAPS.to_string(),
            PRIMITIVES.to_string(),
        ]);
    }

    #[test]
    fn maps_are_added_uniquely() {
        todo!()
    }
}