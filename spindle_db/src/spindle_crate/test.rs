use crate::{DbResult, TypeDb, _MAP_TAGS, _TAGS};

impl TypeDb {
    fn new_crates_test_db(test_name: &str) -> DbResult<TypeDb> {
        let db = Self::open_empty_db_in_memory()?;
        db.create_primitives_table()?;
        todo!("z")
        // db.create_new_union_tables()?;
        // db.create_new_map_tables()?;
        // db.create_new_crate_tables()?;
        // db.create_new_tag_table()?;
        // db.create_new_map_tag_table()?;
        // Ok(db)
    }
}

use crate::{
    _CRATES, _CRATE_UNIONS, _IN_OUTS, _LIFTS, _LIFT_CRATES, _LIFT_ENTRIES, _MAPS, _PRIMITIVES,
    _UNIONS, _UNION_FIELDS,
};

#[test]
fn spindle_crate_new_db_has_correct_table_names() {
    let db = TypeDb::new_crates_test_db("spindle_crate_new_db_has_correct_table_names").unwrap();
    let mut names = db.table_names().unwrap();
    names.sort();
    assert_eq!(
        &names,
        &[
            _CRATE_UNIONS.to_string(),
            _CRATES.to_string(),
            _IN_OUTS.to_string(),
            _LIFT_CRATES.to_string(),
            _LIFT_ENTRIES.to_string(),
            _LIFTS.to_string(),
            _MAP_TAGS.to_string(),
            _MAPS.to_string(),
            _PRIMITIVES.to_string(),
            _TAGS.to_string(),
            _UNION_FIELDS.to_string(),
            // _UNION_TAGS.to_string(),
            _UNIONS.to_string(),
        ]
    );
}

#[test]
#[allow(unused)]
fn can_form_a_crate_from_unions() {
    let tags: Vec<&str> = vec![];

    let db = TypeDb::new_crates_test_db("can_form_a_crate_from_unions").unwrap();
    let u = db.get_or_insert_union(&("U", vec!["f32"])).unwrap();
    let v = db.get_or_insert_union(&("V", vec!["f32", "u64"])).unwrap();
    assert_eq!(db.get_unions().unwrap().len(), 2);
    let m = db
        .get_or_insert_map(
            &(
                "foo",
                "pub fn foo(u64) -> f32;",
                vec![(Some("u64"), Some("f32"))],
            ),
            &tags,
        )
        .unwrap();
    let n = db
        .get_or_insert_map(
            &(
                "bar",
                "pub fn bar(f32, u64) -> (f32, ());",
                vec![(Some("f32"), Some("f32")), (Some("u64"), None)],
            ),
            &tags,
        )
        .unwrap();
    assert_eq!(db.get_maps().unwrap().len(), 2);
    // let c = db.get_or_insert_crate_from_unions(vec![u.clone()]).unwrap();
    // assert_eq!(c.unions.len(), 1);
    // assert_eq!(c.lifters.len(), 0);
    // let d = db.get_or_insert_crate_from_unions(vec![v.clone()]).unwrap();
    // assert_eq!(d.unions.len(), 1);
    // assert_eq!(d.lifters.len(), 1);
    let p = db
        .get_or_insert_map(
            &(
                "baz",
                "pub fn baz(f32, u64) -> ((), f32);",
                vec![(Some("f32"), None), (Some("u64"), Some("f32"))],
            ),
            &tags,
        )
        .unwrap();
    assert_eq!(db.get_maps().unwrap().len(), 3);

    let e = db.get_or_insert_crate_from_unions(vec![u, v]).unwrap();
    assert_eq!(e.unions.len(), 2);
    assert_eq!(e.lifts.len(), 2);

    println!("{e}");
}
