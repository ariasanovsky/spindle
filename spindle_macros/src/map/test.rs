use proc_macro2::TokenStream;
use spindle_db::{TypeDb, map::DbMap};
use syn::parse_quote;

use crate::map::{MapFn, tokens::MapTokens};

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

#[test]
fn emit_tokens_from_new_map() {
    // connect to database
    // add function to database
    const DB_NAME: &str = "emit_tokens_from_new_map";
    const DB_PATH: &str = "target/spindle/db/";
    let db = TypeDb::new(DB_NAME, DB_PATH).unwrap();
    
    // parse a map & insert it into the db
    let map: TokenStream = quote::quote! {
        fn foo(x: i32) -> f64 {
            x as f64
        }
    };
    let map: MapFn = parse_quote!(#map);
    let db_map: DbMap = db.get_or_insert_map(&map).unwrap();
    dbg!(&db_map);
    let map_2: MapFn = syn::parse_str::<MapFn>(&db_map.content).unwrap();
    assert_eq!(map, map_2);

    let decl = map_2.user_crate_declaration();
    let decl_2 = quote::quote! {
        fn foo(x: i32) -> f64 {
            x as f64
        }
    };
    assert_eq!(decl.to_string(), decl_2.to_string());

    let map_trait = map_2.user_crate_trait();
    let map_trait_2 = quote::quote! {
        mod __foo {
            use spindle::__cudarc::DeviceRepr as __DeviceRepr;
            use spindle::__cudarc::CudaSlice as __CudaSlice;
            unsafe trait __Foo
            where
                <Self as __Foo>::U: __DeviceRepr,
                Self: Into<__CudaSlice<Self::U>>,
                __CudaSlice<<Self as __Foo>::U>: Into<<Self as __Foo>::Return>,
            {
                type U;
                type Return;
                fn foo(&self) -> spindle::Result<Self::Return>;
            }
        }
    };
    assert_eq!(map_trait.to_string(), map_trait_2.to_string());
}