use crate::{DbResult, TypeDb};

// const TEST: &str = "tests";
const PRIMITIVES: &str = "primitives";

impl TypeDb {
    fn new_primitives_test_db(test_name: &str) -> DbResult<TypeDb> {
        dbg!();
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
    let p = db.get_or_insert_primitive(&"f32").unwrap();
    assert_eq!(
        db._get_primitives().unwrap(),
        vec![DbPrimitive::new("f32".to_string())]
    );
    let q = db.get_primitive_from_uuid(p.uuid).unwrap().unwrap();
    assert_eq!(
        db._get_primitives().unwrap(),
        vec![DbPrimitive::new("f32".to_string())]
    );
    let r = db.get_or_insert_primitive(&"f32").unwrap();
    assert_eq!(
        db._get_primitives().unwrap(),
        vec![DbPrimitive::new("f32".to_string())]
    );
    assert_eq!(q.uuid, r.uuid);
    let s = db.get_or_insert_primitive(&"f64").unwrap();
    dbg!(&s);
    assert_eq!(
        db._get_primitives().unwrap(),
        vec![
            DbPrimitive::new("f32".to_string()),
            DbPrimitive::new("f64".to_string())
        ]
    );
    assert_ne!(q.uuid, s.uuid);
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
