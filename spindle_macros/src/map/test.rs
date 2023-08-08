use proc_macro2::TokenStream;
use spindle_db::TypeDb;
use syn::parse_quote;

use crate::map::MapFn;

// this works once, then not a 2nd time
#[test]
#[allow(unused)]
fn add_univariate_pure_function_to_db() {
    // connect to database
    // add function to database
    const DB_NAME: &str = "add_univariate_pure_function_to_db";
    const DB_PATH: &str = "target/spindle/db/";
    let db = TypeDb::new(DB_NAME, DB_PATH).unwrap();
    dbg!(&db);

    // first, let's show all `DbMap`s in the db
    let maps = db.map_iter().unwrap();
    maps.for_each(|map| {
        let map = map;
        dbg!(&map);
    });
    dbg!();

    // add map to database
    let map: TokenStream = quote::quote! {
        fn foo(x: i32) -> f64 {
            x as f64
        }
    };
    dbg!(&map);
    let map: MapFn = parse_quote!(#map);
    dbg!(&map);
    let map = db.get_or_insert_map(&map).unwrap();

    let maps = db.map_iter().unwrap();
    maps.for_each(|map| {
        let map = map;
        dbg!(&map);
    });
    dbg!();
}
