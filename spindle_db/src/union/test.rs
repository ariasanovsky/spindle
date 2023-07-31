use crate::{TypeDb, DbResult};

impl TypeDb {
    fn new_unions_test_db(test_name: &str) -> DbResult<TypeDb> {
        let db = Self::new_test_db(test_name)?;
        db.create_new_union_tables()?;
        Ok(db)
    }
}

#[cfg(test)]
mod db_tests {
    use crate::{TypeDb, primitive::{AsDbPrimitive, DbPrimitive}, PRIMITIVES, UNIONS, UNION_FIELDS, union::{AsDbUnion, DbUnion}};
    use super::*;

    impl<'a> AsDbUnion for (&'a str, Vec<&'a str>) {
        type Primitive = &'a str;

        fn db_ident(&self) -> String {
            self.0.to_string()
        }

        fn db_fields(&self) -> Vec<<Self as AsDbUnion>::Primitive> {
            self.1.clone()
        }
    }

    #[test]
    fn unions_new_db_has_correct_table_names() {
        let db = TypeDb::new_unions_test_db("unions_new_db_has_correct_table_names").unwrap();
        let mut names = db.table_names().unwrap();
        names.sort();
        assert_eq!(&names, &[
            PRIMITIVES.to_string(),
            UNION_FIELDS.to_string(),
            UNIONS.to_string(),
        ]);
    }

    #[test]
    fn unions_are_added_uniquely() {
        let db = TypeDb::new_unions_test_db("unions_are_added_uniquely").unwrap();
        assert_eq!(db.get_unions().unwrap(), vec![]);
        let u = db.get_or_insert_union(&("U", vec!["f32"])).unwrap();
        assert_eq!(db.get_unions().unwrap(), vec![
            DbUnion::new("U".to_string(), vec![
                DbPrimitive::new("f32".to_string())
            ])
        ]);
        let v = db.get_union_from_uuid_and_ident(u.uuid, u.ident).unwrap();
        dbg!(&v);
        assert_eq!(db.get_unions().unwrap(), vec![
            DbUnion::new("U".to_string(), vec![
                DbPrimitive::new("f32".to_string())
            ])
        ]);
        // same ident, different fields
        let w = db.get_or_insert_union(&("U", vec!["u64"])).unwrap();
        assert_eq!(db.get_unions().unwrap().len(), 2);
        // multiple fields
        let x = db.get_or_insert_union(&("X", vec!["f32", "u64"])).unwrap();
        assert_eq!(db.get_unions().unwrap().len(), 3);
    }
}
