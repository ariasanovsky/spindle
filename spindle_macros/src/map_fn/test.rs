use proc_macro2::{Ident, Span, TokenStream};
use quote::ToTokens;
use spindle_db::TypeDb;
use syn::parse_quote;

use crate::{
    case::UpperCamelIdent,
    map_fn::{tokens::MapTokens, DevMapFn},
};

use super::MapAttrs;

#[test]
fn example_01_map() {
    let pound = syn::token::Pound::default();
    let map_attr_input = quote::quote! {
        #pound example_01
    };
    let attrs: MapAttrs = parse_quote!(#map_attr_input);
    let expected_tags = vec!["example_01"];
    assert_eq!(
        attrs
            .tags
            .iter()
            .map(|attr| attr.0 .0.to_string())
            .collect::<Vec<_>>(),
        expected_tags
    );

    let map_fn_input = quote::quote! {
        fn i32_to_f64(x: i32) -> f64 {
            x as f64
        }
    };
    let map: DevMapFn = parse_quote!(#map_fn_input);
    let map_host_crate_tokens = crate::map_fn::map(attrs, map, "example_01_map_test").unwrap();
    let expected_map_host_crate_tokens = quote::quote! {
        fn i32_to_f64(x: i32) -> f64 {
            x as f64
        }
        mod __i32_to_f64 {
            use spindle::__cudarc::{
                CudaDevice as __CudaDevice,
                CudaFunction as __CudaFunction,
                CudaSlice as __CudaSlice,
                DeviceRepr as __DeviceRepr,
                LaunchAsync as __LaunchAsync,
                LaunchConfig as __LaunchConfig,
                Ptx as __Ptx,
            };
            pub unsafe trait __I32ToF64
            where
                <Self as __I32ToF64>::U: __DeviceRepr,
                Self: Into<__CudaSlice<<Self as __I32ToF64>::U>>,
                __CudaSlice<<Self as __I32ToF64>::U>: Into<<Self as __I32ToF64>::Return>,
            {
                type U;
                type Return;
                const PTX_PATH: &'static str;
                fn i32_to_f64(self, n: i32) -> spindle::Result<Self::Return> {
                    let mut slice: __CudaSlice<Self::U> = self.into();
                    let device: std::sync::Arc<__CudaDevice> = slice.device();
                    let ptx: __Ptx = __Ptx::from_file(Self::PTX_PATH);
                    device.load_ptx(ptx, "kernels", &["i32_to_f64_kernel"])?;
                    let f: __CudaFunction =
                        device.get_func("kernels", "i32_to_f64_kernel")
                        .ok_or(spindle::error::function_not_found(Self::PTX_PATH, "i32_to_f64_kernel"))?;
                    let config: __LaunchConfig = __LaunchConfig::for_num_elems(n as u32);
                    unsafe { f.launch(config, (&mut slice, n)) }?;
                    Ok(slice.into())
                }
            }
        }
        pub use __i32_to_f64::__I32ToF64;
    };
    assert_eq!(
        map_host_crate_tokens.to_string(),
        expected_map_host_crate_tokens.to_string()
    );

    // now we verify that the MapFn will write the correct tokens to the ptx crate
}

#[test]
#[allow(unused)]
fn add_univariate_pure_function_to_db() {
    // connect to database
    // add function to database
    let target = std::env::var("CARGO_TARGET_DIR").unwrap_or_else(|_| "target".to_string());
    const DB_NAME: &str = "add_univariate_pure_function_to_db";
    let db_path: String = format!("{target}/spindle/db/");
    let db = TypeDb::new(DB_NAME, db_path).unwrap();

    // first, let's show all `DbMap`s in the db
    todo!("refactor tests");
    // let maps = db.get_maps().unwrap();
    // maps.into_iter().for_each(|map| {
    //     let map = map;
    // });

    // // add map to database
    // let map: TokenStream = quote::quote! {
    //     fn foo(x: i32) -> f64 {
    //         x as f64
    //     }
    // };
    // let map: DevMapFn = parse_quote!(#map);
    // let tags: Vec<&str> = Vec::new();
    // todo!("refactor tests");
    // // let map = db.get_or_insert_map(&map, &tags).unwrap();

    // // let maps = db.get_maps().unwrap();
    // // maps.into_iter().for_each(|map| {
    // //     let map = map;
    // // });
}

#[test]
fn emit_tokens_from_new_map() {
    let map_fn: DevMapFn = syn::parse_quote! {
        fn foo(x: i32) -> f64 {
            x as f64
        }
    };
    let map_trait = map_fn.map_trait();
    let expected_map_trait = quote::quote! {
        mod __foo {
            use spindle::__cudarc::{
                CudaDevice as __CudaDevice,
                CudaFunction as __CudaFunction,
                CudaSlice as __CudaSlice,
                DeviceRepr as __DeviceRepr,
                LaunchAsync as __LaunchAsync,
                LaunchConfig as __LaunchConfig,
                Ptx as __Ptx,
            };
            pub unsafe trait __Foo
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
                fn foo(self, n: i32) -> spindle::Result<Self::Return> {
                    let mut slice: __CudaSlice<Self::U> = self.into();
                    let device: std::sync::Arc<__CudaDevice> = slice.device();
                    let ptx: __Ptx = __Ptx::from_file(Self::PTX_PATH);
                    device.load_ptx(ptx, "kernels", &["foo_kernel"])?;
                    let f: __CudaFunction =
                        device.get_func("kernels", "foo_kernel")
                        .ok_or(spindle::error::function_not_found(Self::PTX_PATH, "foo_kernel"))?;
                    let config: __LaunchConfig = __LaunchConfig::for_num_elems(n as u32);
                    unsafe { f.launch(config, (&mut slice, n)) }?;
                    Ok(slice.into())
                }
            }
        }
        pub use __foo::__Foo;
    };
    assert_eq!(map_trait.to_string(), expected_map_trait.to_string());
    let u = UpperCamelIdent(Ident::new("U", Span::call_site()));
    let crate_method = map_fn.ptx_crate_method(&u);
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

    let crate_kernel = map_fn.ptx_crate_kernel(&u);
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

    let crate_decl = map_fn.ptx_crate_declaration();
    let expected_crate_decl = quote::quote! {
        fn foo(x: i32) -> f64 {
            x as f64
        }
    };
    assert_eq!(crate_decl.to_string(), expected_crate_decl.to_string());
}

#[test]
fn parse_tags_from_map_macro_attrs() {
    /* we want the user to be able to write
        #[spindle::map(#example, #other)]
        fn foo(x: i32) -> f64 {
            x as f64
        }
    and associate the tags to the MapFn
    */
    let pound = syn::token::Pound::default();
    let input = quote::quote! {
        #pound example, #pound other
    };
    let output: MapAttrs = parse_quote! { #input };
    let example = output.tags.get(0).unwrap();
    assert_eq!(example.0 .0.to_string(), "example");
    let other = output.tags.get(1).unwrap();
    assert_eq!(other.0 .0.to_string(), "other");
    assert!(output.tags.get(2).is_none());
}
