use crate::{DbResult, TypeDb};

impl TypeDb {
    fn new_unions_test_db(test_name: &str) -> DbResult<TypeDb> {
        let db = Self::_new_test_db(test_name)?;
        db.drop_tables()?;
        db.create_new_primitive_table()?;
        db.create_new_union_tables()?;
        Ok(db)
    }
}

use crate::{
    primitive::DbPrimitive,
    union::{AsDbUnion, DbUnion},
    _PRIMITIVES, _UNIONS, _UNION_FIELDS,
};
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
    assert_eq!(
        &names,
        &[
            _PRIMITIVES.to_string(),
            _UNION_FIELDS.to_string(),
            _UNIONS.to_string(),
        ]
    );
}

#[test]
fn unions_are_added_uniquely() {
    let db = TypeDb::new_unions_test_db("unions_are_added_uniquely").unwrap();
    assert_eq!(db.get_unions().unwrap(), vec![]);
    let u = db.get_or_insert_union(&("U", vec!["f32"])).unwrap();
    assert_eq!(
        db.get_unions().unwrap(),
        vec![DbUnion::new(
            "U".to_string(),
            vec![DbPrimitive::new("f32".to_string())]
        )]
    );
    let _v = db.get_union_from_uuid(u.uuid.clone()).unwrap();
    assert_eq!(
        db.get_unions().unwrap(),
        vec![DbUnion::new(
            "U".to_string(),
            vec![DbPrimitive::new("f32".to_string())]
        )]
    );
    // same ident, different fields
    let _w = db.get_or_insert_union(&("U", vec!["u64"])).unwrap();
    assert_eq!(db.get_unions().unwrap().len(), 2);
    // multiple fields
    let _x = db.get_or_insert_union(&("X", vec!["f32", "u64"])).unwrap();
    assert_eq!(db.get_unions().unwrap().len(), 3);
}
