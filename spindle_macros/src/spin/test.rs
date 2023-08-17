use std::path::PathBuf;

use quote::ToTokens;
use spindle_db::map::DbMap;
use syn::parse_quote;

use crate::map::MapFn;

use super::{SpinInputs, SpindleCrate};

#[test]
fn example_01_spin() {
    // first, we need to add the map to the db explicitly
    let map_input = quote::quote! {
        fn i32_to_f64(x: i32) -> f64 {
            x as f64
        }
    };
    let map_fn: MapFn = parse_quote! { #map_input };
    let db = spindle_db::TypeDb::new("test_example_01_spin", "target/spindle/db").unwrap();
    let db_map = db.get_or_insert_map(&map_fn, &vec!["test_example_01_spin"]).unwrap();
    
    // now we parse the spin input
    let pound = syn::token::Pound::default();
    let spin_input = quote::quote! {
        #pound test_example_01_spin, U = i32 | f64
    };
    let spin_inputs: SpinInputs = parse_quote! { #spin_input };
    let spindle_crate: SpindleCrate = (spin_inputs, "test_example_01_spin").try_into().unwrap();
    let SpindleCrate {
        home,
        maps,
        tag,
        unions,
    } = &spindle_crate;
    // test the home directory
    let expected_home: PathBuf = "target/spindle/crates/test_example_01_spin".into();
    assert_eq!(home, &expected_home);
    // test the map
    assert_eq!(maps.len(), 1);
    let map = maps.get(0).unwrap();
    let DbMap {
        uuid: _,
        ident,
        content,
        in_outs,
    } = map;
    assert_eq!(ident.to_string(), "i32_to_f64");
    assert_eq!(content, &map_fn.item_fn.into_token_stream().to_string());
    assert_eq!(in_outs.len(), 1);
    let in_out = in_outs.get(0).unwrap();
    assert_eq!(in_out.input.as_ref().unwrap().ident.as_str(), "i32");
    assert_eq!(in_out.output.as_ref().unwrap().ident.as_str(), "f64");
    
    let kernel_rs: proc_macro2::TokenStream = spindle_crate.lib_rs();
    let expected_kernel = quote::quote! {
        #![no_std]
        #![feature(abi_ptx)]
        #![feature(stdsimd)]
        // #![feature(core_intrinsics)]

        use core::arch::nvptx::*;

        mod device;
        use device::*;

        #[panic_handler]
        fn my_panic(_: &core::panic::PanicInfo) -> ! {
            loop {}
        }

        #[no_mangle]
        pub unsafe extern "ptx-kernel" fn i32_to_f64_kernel(slice: *mut U, size: i32) {
            // todo! try other thread geometry
            let thread_id: i32 = _thread_idx_x();
            let block_id: i32 = _block_idx_x();
            let block_dim: i32 = _block_dim_x();
            let grid_dim: i32 = _grid_dim_x();
            
            let n_threads: i32 = block_dim * grid_dim;
            let thread_index: i32 =  thread_id + block_id * block_dim;

            let mut i: i32 = thread_index;
            while i < size {
                let u: &mut U = &mut *slice.offset(i as isize);
                u.i32_to_f64();
                i = i.wrapping_add(n_threads);
            }
        }
    };
    assert_eq!(kernel_rs.to_string(), expected_kernel.to_string());
    let device: proc_macro2::TokenStream = spindle_crate.device_rs();
    let expected_device = quote::quote! {
        fn i32_to_f64(x: i32) -> f64 {
            x as f64
        }
        #pound[repr(C)]
        pub union U {
            _0: i32,
            _1: f64,
        }
        impl U {
            pub(crate) unsafe fn i32_to_f64(&mut self) {
                let input_ref = &*(self as *mut _ as *mut _);
                let output = i32_to_f64(*input_ref);
                let output_ptr: *mut _ = self as *mut _ as _;
                *output_ptr = output;
            }
        }
    };
    assert_eq!(device.to_string(), expected_device.to_string());
    let _: () = spindle_crate.populate().unwrap();
    let out: std::process::Output = spindle_crate.compile().unwrap();
}
