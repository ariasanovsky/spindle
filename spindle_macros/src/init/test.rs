use proc_macro2::Ident;
use quote::ToTokens;
use spindle_db::TypeDb;
use syn::{Token, parse_quote};

use crate::init::{Attrs, DevInitFn};

use pretty_assertions::assert_eq;

#[test]
fn example_02_init() {
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
    db.create_or_ignore_tables_for_tagged_item_fns().unwrap();
    let tags: Vec<&str> = vec!["example_02_init"];
    let db_init_fn = db.get_or_insert_item_fn(&init_fn, &tags).unwrap();
    let db_init_fn: DevInitFn = db_init_fn.try_into().unwrap();
    assert_eq!(db_init_fn.to_token_stream().to_string(), init_fn.to_token_stream().to_string());
    let union_ident: Ident = parse_quote! { U };
    let union_idents: Vec<Ident> = vec![union_ident];
    let init_kernel = db_init_fn.kernel(&union_idents).unwrap();
    let expected_kernel = quote::quote! {
        #[no_mangle]
        pub unsafe extern "ptx-kernel" fn square_over_two_init_kernel(slice: *mut U, size: i32) {
            let thread_id: i32 = _thread_idx_x();
            let block_id: i32 = _block_idx_x();
            let block_dim: i32 = _block_dim_x();
            let grid_dim: i32 = _grid_dim_x();

            let n_threads: i32 = block_dim * grid_dim;
            let thread_index: i32 =  thread_id + block_id * block_dim;

            let mut i: i32 = thread_index;
            while i < size {
                let u: &mut U = &mut *slice.offset(i as isize);
                u.square_over_two_init(i as u64);
                i = i.wrapping_add(n_threads);
            }
        }
    };
    assert_eq!(init_kernel.to_string(), expected_kernel.to_string());
    let device_item_fn = db_init_fn.device_item_fn();
    let expected_device_item_fn = quote::quote! {
        fn square_over_two(x: u64) -> f32 {
            ((x * x) / 2) as f32
        }
    };
    assert_eq!(device_item_fn.to_string(), expected_device_item_fn.to_string());
    let device_method = db_init_fn.device_method(&union_idents).unwrap();
    let expected_device_method = quote::quote! {
        pub(crate) unsafe fn square_over_two_init(&mut self, i: u64) {
            let init_value = square_over_two(i);
            let output_ptr: *mut _ = self as *mut _ as _;
            *output_ptr = init_value;
        }
    };
    assert_eq!(device_method.to_string(), expected_device_method.to_string());
    
    let launch_trait = db_init_fn.launch_trait();
    let expected_launch_trait = quote::quote! {
        mod __square_over_two {
            pub unsafe trait __SquareOverTwo {
                type Return;
                const PTX_PATH: &'static str;
                fn square_over_two(&self, size: i32) -> spindle::Result<Self::Return>;        
            }
        }
        pub use __square_over_two::__SquareOverTwo;
    };
    assert_eq!(launch_trait.to_string(), expected_launch_trait.to_string());
    let launch_impl = db_init_fn.launch_impl("example_02_test", &union_idents).unwrap();
    let target_dir = std::env::var("CARGO_TARGET_DIR").unwrap_or_else(|_| "target".to_string());
    let ptx_path = format!("{target_dir}/spindle/map/example_02_test/target/nvptx64-nvidia-cuda/target/release/kernel.ptx");
    let expected_launch_impl = quote::quote! {
        unsafe impl __square_over_two::__SquareOverTwo for std::ops::RangeFrom<u64> {
            type Return = spindle::DevSlice<U, f32>;
            const PTX_PATH: &'static str = #ptx_path;
            fn square_over_two(&self, size: i32) -> spindle::Result<Self::Return> {
                use spindle::__cudarc::{
                    CudaDevice as __CudaDevice,
                    CudaFunction as __CudaFunction,
                    CudaSlice as __CudaSlice,
                    // DeviceRepr as __DeviceRepr,
                    LaunchAsync as __LaunchAsync,
                    LaunchConfig as __LaunchConfig,
                    Ptx as __Ptx,
                };
                let device: std::sync::Arc<__CudaDevice> = __CudaDevice::new(0)?;
                let mut slice: __CudaSlice<_> = unsafe { device.alloc(size as usize) }?;
                let ptx: __Ptx = __Ptx::from_file(Self::PTX_PATH);
                device.load_ptx(ptx, "kernels", &["square_over_two_kernel"])?;
                let f: __CudaFunction = device.get_func(
                    "kernels",
                    "square_over_two_kernel"
                )
                .ok_or(spindle::error::function_not_found(
                    Self::PTX_PATH,
                    "square_over_two_kernel"
                ))?;
                let config: __LaunchConfig = __LaunchConfig::for_num_elems(size as u32);
                unsafe { f.launch(config, (&mut slice, size)) }?;
                Ok(spindle::DevSlice::from(slice))
            }
        }
    };
    assert_eq!(launch_impl.to_string(), expected_launch_impl.to_string());
}
