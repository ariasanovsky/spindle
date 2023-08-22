use crate::{TypeDb, item_fn::DbItemFn};

use super::AsDbItemFn;

struct DummyItemFn {
    ident: String,
    content: String,
}

impl AsDbItemFn for DummyItemFn {
    fn db_item_ident(&self) -> String {
        self.ident.clone()
    }

    fn db_item_content(&self) -> String {
        self.content.clone()
    }
}

#[test]
fn get_or_insert_a_single_item_fn() {
    let db = TypeDb::open_empty_db_in_memory().unwrap();
    db.create_new_item_fn_table().unwrap();
    db.create_new_tag_table().unwrap();
    db.create_new_tagged_item_fn_table().unwrap();
    let tags = vec!["fizz", "buzz"];
    let foo = DummyItemFn {
        ident: "foo".to_string(),
        content: "fn foo() -> i32 { 42 }".to_string(),
    };
    let db_item_fn = db.get_or_insert_item_fn(&foo, &tags).unwrap();
    let db_item_fn_again = db.get_or_insert_item_fn(&foo, &tags).unwrap();
    assert_eq!(db_item_fn, db_item_fn_again);
    let DbItemFn { uuid, ident, content } = &db_item_fn;
    assert_eq!(uuid.len(), 36);
    assert_eq!(ident, "foo");
    assert_eq!(content, "fn foo() -> i32 { 42 }");
    let tags_on_foo = db.get_tags_on_item_fn(&db_item_fn).unwrap();
    assert_eq!(tags_on_foo, tags);
    let item_fns_with_fizz_tag = db.get_item_fns_with_tag("fizz").unwrap();
    assert_eq!(item_fns_with_fizz_tag, vec![db_item_fn.clone()]);
    let item_fns_with_buzz_tag = db.get_item_fns_with_tag("buzz").unwrap();
    assert_eq!(item_fns_with_buzz_tag, vec![db_item_fn.clone()]);
}

#[test]
fn get_or_insert_two_item_fns_with_the_same_ident() {
    todo!("Implement this test")
}