use syn::{Token, parse_quote};

use crate::init::{Attrs, InputInitFn};

#[test]
fn example_02_range_init() {
    let pound: Token![#] = Default::default();
    // test the attr parsing
    let attr = quote::quote! {
        #pound example_02_test
    };
    dbg!(attr.to_string());
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
    let item = quote::quote! {
        fn square_over_two(x: u64) -> f32 {
            ((x * x) / 2) as f32
        }
    };
    let input_item: InputInitFn = parse_quote! {
        #item
    };
    let InputInitFn {
        item_fn: _,
        input_type,
        output_type,
    } = &input_item;
    let expected_input_type = "u64";
    // todo! refactor input_type to_string
    assert!(input_type.to_string().contains(expected_input_type));
    let expected_output_type = "f32";
    assert!(output_type.to_string().contains(expected_output_type));
    // add to db
    let target = std::env::var("CARGO_TARGET_DIR").unwrap_or_else(|_| "target".to_string());
    let db_path: String = format!("{target}/spindle/db/");
    let db = spindle_db::TypeDb::new("test_example_02_range", db_path).unwrap();
    let tags: Vec<&str> = vec!["example_02_test"];
    let db_map = db.get_or_insert_map(&input_item, &tags).unwrap();
    // we'll add it to `maps` but include an Option<DbPrimitive> for the integer range
    // for example, this is a [Any,] (u64) -> [f32,] map
    // test the db struct
    // define `square_over_two` as a method on ranges, e.g., (..10u64).square_over_two();
    // test spin! input
    // test spin! output
    todo!("what else to test???")
}
