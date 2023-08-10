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
            use spindle::__cudarc::{
                CudaDevice as __CudaDevice,
                CudaFunction as __CudaFunction,
                CudaSlice as __CudaSlice,
                DeviceRepr as __DeviceRepr,
                LaunchConfig as __LaunchConfig,
                Ptx as __Ptx,
            };
            use std::sync::Arc as __Arc;
            unsafe trait __Foo
            where
                <Self as __Foo>::U:
                    __DeviceRepr,
                Self:
                    Into<__CudaSlice<<Self as __Foo>::U>>,
                __CudaSlice<<Self as __Foo>::U>:
                    Into<<Self as __Foo>::Return>,
            {
                type U;
                type Return;
                const PTX_PATH: &'static str;
                fn foo(&self, n: u32) -> spindle::Result<Self::Return> {
                    let mut slice: __CudaSlice<Self::U> = self.into();
                    let device: __Arc<__CudaDevice> = slice.device();
                    let ptx: __Ptx = __Ptx::from_file(Self::PTX_PATH);
                    device.load_ptx(ptx, "kernels", &["foo_kernel"])?;
                    let f: __CudaFunction =
                        device.get_function("foo_kernel")
                        .ok_or(spindle::Error::FunctionNotFound)?;
                    let config: __LaunchConfig = __LaunchConfig::for_num_elems(n as u32);
                    unsafe { f.launch(config, (&mut slice, n as i32)) }?;
                    Ok(slice.into())
                }
            }
        }
    };
    assert_eq!(map_trait.to_string(), map_trait_2.to_string());
}