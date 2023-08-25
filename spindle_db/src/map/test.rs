use crate::{DbResult, TypeDb, _MAP_TAGS, _TAGS};

impl TypeDb {
    fn new_maps_test_db(test_name: &str) -> DbResult<TypeDb> {
        todo!();
        // let db = Self::open_empty_db_in_memory()?;
        // db.drop_tables()?;
        // db.create_primitives_table()?;
        // db.create_new_map_tables()?;
        // db.create_new_tag_table()?;
        // db.create_new_map_tag_table()?;
        // Ok(db)
        // todo!("y")
    }
}

use crate::{map::AsDbMap, _IN_OUTS, _MAPS, _PRIMITIVES};

use super::AsDbInOut;

impl<'a> AsDbInOut for (Option<&'a str>, Option<&'a str>) {
    type Primitive = &'a str;

    fn db_inout(&self) -> (Option<Self::Primitive>, Option<Self::Primitive>) {
        (self.0, self.1)
    }
}

impl<'a> AsDbMap for (&'a str, &'a str, Vec<(Option<&'a str>, Option<&'a str>)>) {
    type InOut = (Option<&'a str>, Option<&'a str>);

    fn db_ident(&self) -> String {
        self.0.to_string()
    }

    fn db_content(&self) -> String {
        self.1.to_string()
    }

    fn db_inouts(&self) -> Vec<Self::InOut> {
        self.2.iter().map(|(i, o)| (i.clone(), o.clone())).collect()
    }

    fn range_type(&self) -> Option<String> {
        None
    }
}

#[test]
fn maps_new_db_has_correct_table_names() {
    let db = TypeDb::new_maps_test_db("maps_new_db_has_correct_table_names").unwrap();
    let mut names = db.table_names().unwrap();
    names.sort();
    assert_eq!(
        &names,
        &[
            _IN_OUTS.to_string(),
            _MAP_TAGS.to_string(),
            _MAPS.to_string(),
            _PRIMITIVES.to_string(),
            _TAGS.to_string(),
        ]
    );
}

#[test]
#[allow(unused)]
fn maps_are_inserted_uniquely() {
    let tags: Vec<&str> = vec!["example"];
    let other_tags: Vec<&str> = vec!["other"];

    let db = TypeDb::new_maps_test_db("maps_are_added_uniquely").unwrap();
    assert_eq!(db.get_maps().unwrap(), vec![]);
    let m: crate::map::DbMap = db
        .get_or_insert_map(
            &(
                "foo",
                "pub fn foo(x: f32) -> f32 { x + 1.0 }",
                vec![(Some("f32"), Some("f32"))],
            ),
            &tags,
        )
        .unwrap();
    assert_eq!(db.get_maps().unwrap(), vec![m.clone()]);
    let n = db
        .get_or_insert_map(
            &(
                "foo",
                "pub fn foo(x: f32) -> f32 { x + 1.0 }",
                vec![(Some("f32"), Some("f32"))],
            ),
            &tags,
        )
        .unwrap();
    assert_eq!(db.get_maps().unwrap(), vec![m.clone()]);
    assert_eq!(m, n);
    let o = db
        .get_or_insert_map(
            &(
                "bar",
                "unsafe fn bar(x: f32) -> f32 { x * x }",
                vec![(Some("f32"), Some("f32"))],
            ),
            &tags,
        )
        .unwrap();
    assert_eq!(db.get_maps().unwrap().len(), 2);
    assert_ne!(m, o);
    let p = db
        .get_or_insert_map(
            &(
                "foo",
                "pub fn foo(x: f32, y: u64) -> (u64, f32) { (y / 10, x + y as f32) }",
                vec![(Some("f32"), Some("f32")), (Some("u64"), Some("u64"))],
            ),
            /* &tags <- this correctly panics */ &other_tags,
        )
        .unwrap();
    assert_eq!(db.get_maps().unwrap().len(), 3);
    // insert a previous element
    let q = db
        .get_or_insert_map(
            &(
                "foo",
                "pub fn foo(x: f32) -> f32 { x + 1.0 }",
                vec![(Some("f32"), Some("f32"))],
            ),
            &tags,
        )
        .unwrap();
    assert_eq!(db.get_maps().unwrap().len(), 3);
    // and make sure to test `None` examples, too
    let r = db
        .get_or_insert_map(
            &(
                "baz",
                "pub fn baz(x: &mut f32) -> u64 { *x += 1.0; *x as u64 }",
                vec![(Some("f32"), Some("f32")), (None, Some("u64"))],
            ),
            /* &tags <- this correctly panics */ &other_tags,
        )
        .unwrap();
    assert_eq!(db.get_maps().unwrap().len(), 4);
    assert_ne!(m, r);
    // let's look up the map with the `None` value
    let s = db
        .get_or_insert_map(
            &(
                "baz",
                "pub fn baz(x: &mut f32) -> u64 { *x += 1.0; *x as u64 }",
                vec![(Some("f32"), Some("f32")), (None, Some("u64"))],
            ),
            /* &tags &other_tags <- these correctly panic */ &vec!["other_other"],
        )
        .unwrap();
    assert_eq!(db.get_maps().unwrap().len(), 4);
}
