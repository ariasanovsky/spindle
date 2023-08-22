use quote::ToTokens;
use spindle_db::TypeDb;
use syn::{Token, parse_quote};

use crate::init::{Attrs, DevInitFn};

#[test]
fn example_02_range_init() {
    let pound: Token![#] = Default::default();
    // test the attr parsing
    let attr = quote::quote! {
        #pound example_02_test
    };
    let attrs: Attrs = parse_quote! {
        #attr
    };
    let expected_tag = parse_quote! {
        #pound example_02_test
    };
    let expected_attrs: Attrs = Attrs {
        tags: vec![expected_tag],
    };
    assert_eq!(attrs, expected_attrs);

    // test the function parsing
    let init_fn: DevInitFn = parse_quote! {
        fn square_over_two(x: u64) -> f32 {
            ((x * x) / 2) as f32
        }
    };
    let tokens = init_fn.to_token_stream();
    let expected_tokens = quote::quote! {
        fn square_over_two(x: u64) -> f32 {
            ((x * x) / 2) as f32
        }
    };
    assert_eq!(tokens.to_string(), expected_tokens.to_string());
    // add to db
    let db = TypeDb::open_empty_db_in_memory().unwrap();
    db.create_new_item_fn_table().unwrap();
    db.create_new_tag_table().unwrap();
    db.create_new_tagged_item_fn_table().unwrap();
    let tags: Vec<&str> = vec!["example_02_range_init"];
    let db_init_fn = db.get_or_insert_item_fn(&init_fn, &tags).unwrap();
    let db_init_fn: DevInitFn = db_init_fn.try_into().unwrap();
    assert_eq!(db_init_fn.to_token_stream().to_string(), init_fn.to_token_stream().to_string());
    
    todo!("test lib.rs tokens");
    // todo!("test device.rs tokens");
    // todo!("test trait tokens");
    // todo!("test impl tokens");
}
