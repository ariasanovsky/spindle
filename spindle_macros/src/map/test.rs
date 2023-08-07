use proc_macro2::TokenStream;
use spindle_db::TypeDb;
use syn::parse_quote;

use crate::map::MapFn;

#[test]
fn add_univariate_pure_function_to_db() {
    let input: TokenStream = quote::quote! {
        fn foo(x: i32) -> f64 {
            x as f64
        }
    };
    let input: MapFn = parse_quote!(#input);
    dbg!(&input);
    // connect to database
    // add function to database
    const DB_NAME: &str = "add_univariate_pure_function_to_db";
    const DB_PATH: &str = "target/spindle/db/";
    let db = TypeDb::open_or_create(DB_NAME, DB_PATH).unwrap();
    panic!();
}