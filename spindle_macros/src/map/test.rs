use proc_macro2::{TokenStream, Ident, Span};
use spindle_db::{TypeDb, map::DbMap};
use syn::parse_quote;

use crate::{map::{MapFn, tokens::MapTokens}, case::UpperCamelIdent};

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

    let map_trait = map_2.user_crate_trait("foo_uuid");
    let map_trait_2 = quote::quote! {
        mod __foo {
            const __UUID: &str = "foo_uuid";
            use spindle::__cudarc::{
                CudaDevice as __CudaDevice,
                CudaFunction as __CudaFunction,
                CudaSlice as __CudaSlice,
                DeviceRepr as __DeviceRepr,
                LaunchConfig as __LaunchConfig,
                Ptx as __Ptx,
            };
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
                    let device: std::sync::Arc<__CudaDevice> = slice.device();
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

    let u = UpperCamelIdent(Ident::new("U", Span::call_site()));
    let crate_method = map_2.ptx_crate_method(&u);
    let crate_method_2 = quote::quote! {
        impl U {
            pub(crate) unsafe fn foo(&mut self) {
                let input_ref = &*(self as *mut _ as *mut _);
                let output = foo(*input_ref);
                let output_ptr: *mut _ = self as *mut _ as _;
                *output_ptr = output;
            }
        }
    };
    assert_eq!(crate_method.to_string(), crate_method_2.to_string());

    let crate_kernel = map_2.ptx_crate_kernel(&u);
    let crate_kernel_2 = quote::quote! {
        #[no_mangle]
        pub unsafe extern "ptx-kernel" fn foo_kernel(slice: *mut U, size: i32) {
            let thread_id: i32 = _thread_idx_x();
            let block_id: i32 = _block_idx_x();
            let block_dim: i32 = _block_dim_x();
            let grid_dim: i32 = _grid_dim_x();
            
            let n_threads: i32 = block_dim * grid_dim;
            let thread_index: i32 =  thread_id + block_id * block_dim;

            let mut i: i32 = thread_index;
            while i < size {
                let u: &mut U = &mut *slice.offset(i as isize);
                u.foo();
                i = i.wrapping_add(n_threads);
            }
        }
    };
    assert_eq!(crate_kernel.to_string(), crate_kernel_2.to_string());

    let crate_decl = map_2.ptx_crate_declaration();
    let crate_decl_2 = quote::quote! {
        fn foo(x: i32) -> f64 {
            x as f64
        }
    };
    assert_eq!(crate_decl.to_string(), crate_decl_2.to_string());
}