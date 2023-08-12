use crate::{DbResult, TypeDb, _TAGS};

impl TypeDb {
    fn new_maps_test_db(test_name: &str) -> DbResult<TypeDb> {
        let db = Self::_new_test_db(test_name)?;
        db.drop_tables()?;
        db.create_new_primitive_table()?;
        db.create_new_map_tables()?;
        db.create_new_tag_table()?;
        db.create_new_map_tag_table()?;
        Ok(db)
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

    fn db_inout_pairs(&self) -> Vec<Self::InOut> {
        self.2.iter().map(|(i, o)| (i.clone(), o.clone())).collect()
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

    let db = TypeDb::new_maps_test_db("maps_are_added_uniquely").unwrap();
    assert_eq!(db.get_maps().unwrap(), vec![]);
    dbg!(db.get_maps().unwrap());
    let m = db
        .get_or_insert_map(&("foo", "pub fn foo(...)", vec![(Some("f32"), Some("f32"))]), &tags)
        .unwrap();
    assert_eq!(db.get_maps().unwrap(), vec![m.clone()]);
    let n = db
        .get_or_insert_map(&("foo", "pub fn foo(...)", vec![(Some("f32"), Some("f32"))]), &tags)
        .unwrap();
    assert_eq!(db.get_maps().unwrap(), vec![m.clone()]);
    assert_eq!(m, n);
    let o = db
        .get_or_insert_map(&("bar", "unsafe fn bar(...)", vec![(Some("f32"), Some("f32"))]), &tags)
        .unwrap();
    assert_eq!(db.get_maps().unwrap().len(), 2);
    assert_ne!(m, o);
    let p = db
        .get_or_insert_map(&(
            "foo",
            "pub fn foo(...)",
            vec![(Some("f32"), Some("f32")), (Some("u64"), Some("u64"))],
        ), &tags)
        .unwrap();
    assert_eq!(db.get_maps().unwrap().len(), 3);
    // insert a previous element
    let q = db
        .get_or_insert_map(&("foo", "pub fn foo(...)", vec![(Some("f32"), Some("f32"))]), &tags)
        .unwrap();
    assert_eq!(db.get_maps().unwrap().len(), 3);
    // and make sure to test `None` examples, too
    let r = db
        .get_or_insert_map(&(
            "foo",
            "pub fn foo(...)",
            vec![(Some("f32"), Some("f32")), (None, Some("u64"))],
        ), &tags)
        .unwrap();
    assert_eq!(db.get_maps().unwrap().len(), 4);
    assert_ne!(m, r);
    // let's look up the map with the `None` value
    let s = db
        .get_or_insert_map(&(
            "foo",
            "pub fn foo(...)",
            vec![(Some("f32"), Some("f32")), (None, Some("u64"))],
        ), &tags)
        .unwrap();
    assert_eq!(db.get_maps().unwrap().len(), 4);
}
