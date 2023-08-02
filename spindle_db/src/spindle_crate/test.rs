use crate::{TypeDb, DbResult};

impl TypeDb {
    fn new_crates_test_db(test_name: &str) -> DbResult<TypeDb> {
        todo!()
    }
}

#[cfg(test)]
mod db_tests {
    use crate::TypeDb;

    #[test]
    fn spindle_crate_new_db_has_correct_table_names() {
        let db = TypeDb::new_crates_test_db("spindle_crate_new_db_has_correct_table_names").unwrap();
        let mut names = db.table_names().unwrap();
        names.sort();
        assert_eq!(&names, &[
            "primitives".to_string(),
            "union_fields".to_string(),
            "unions".to_string(),
        ]);
    }
}