use crate::{DbResult, TypeDb};

// const TEST: &str = "tests";
const PRIMITIVES: &str = "primitives";

impl TypeDb {
    fn new_primitives_test_db(test_name: &str) -> DbResult<TypeDb> {
        let db = Self::_new_test_db(test_name)?;
        db.drop_tables()?;
        db.create_new_primitive_table()?;
        Ok(db)
    }
}

use crate::primitive::{AsDbPrimitive, DbPrimitive};
impl AsDbPrimitive for &str {
    fn db_ident(&self) -> String {
        self.to_string()
    }
}

#[test]
fn primitives_new_db_has_correct_table_names() {
    let db = TypeDb::new_primitives_test_db("primitives_new_db_has_correct_table_names").unwrap();
    let mut names = db.table_names().unwrap();
    names.sort();
    assert_eq!(&names, &[PRIMITIVES.to_string()]);
}

#[test]
fn primitives_are_added_uniquely() {
    let db = TypeDb::new_primitives_test_db("primitives_are_added_uniquely").unwrap();
    assert_eq!(db._get_primitives().unwrap(), vec![]);
    dbg!();
    let p = db.get_or_insert_primitive(&"f32").unwrap();
    dbg!(&p);
    assert_eq!(
        db._get_primitives().unwrap(),
        vec![DbPrimitive::new("f32".to_string())]
    );
    dbg!();
    let q = db.get_primitive_from_uuid(p.uuid).unwrap();
    dbg!(&q);
    assert_eq!(
        db._get_primitives().unwrap(),
        vec![DbPrimitive::new("f32".to_string())]
    );
    dbg!();
    let r = db.get_or_insert_primitive(&"f32").unwrap();
    dbg!(&r);
    assert_eq!(
        db._get_primitives().unwrap(),
        vec![DbPrimitive::new("f32".to_string())]
    );
    dbg!();
    assert_eq!(q.uuid, r.uuid);
    dbg!();
    let s = db.get_or_insert_primitive(&"f64").unwrap();
    dbg!(&s);
    assert_eq!(
        db._get_primitives().unwrap(),
        vec![
            DbPrimitive::new("f32".to_string()),
            DbPrimitive::new("f64".to_string())
        ]
    );
    dbg!();
    assert_ne!(q.uuid, s.uuid);
    dbg!();
}
