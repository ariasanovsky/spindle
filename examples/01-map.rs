#[spindle::map(#example_01)]
fn i32_to_f64(x: i32) -> f64 {
    x as f64
}

// write fn to examples/01-map.rs
// fn i32_to_f64(x: i32) -> f64 {
//     x as f64
// }

// write trait to examples/01-map.rs
// mod __i32_to_f64 {
//     use spindle::__cudarc::{
//         CudaDevice as __CudaDevice,
//         CudaFunction as __CudaFunction,
//         CudaSlice as __CudaSlice,
//         DeviceRepr as __DeviceRepr,
//         LaunchAsync as __LaunchAsync,
//         LaunchConfig as __LaunchConfig,
//         Ptx as __Ptx,
//     };
//     pub unsafe trait __I32ToF64
//     where
//         <Self as __I32ToF64>::U: __DeviceRepr,
//         Self: Into<__CudaSlice<<Self as __I32ToF64>::U>>,
//         __CudaSlice<<Self as __I32ToF64>::U>: Into<<Self as __I32ToF64>::Return>,
//     {
//         type U;
//         type Return;
//         const PTX_PATH: &'static str;
//         fn i32_to_f64(self, n: i32) -> spindle::Result<Self::Return> {
//             let mut slice: __CudaSlice<Self::U> = self.into();
//             let device: std::sync::Arc<__CudaDevice> = slice.device();
//             let ptx: __Ptx = __Ptx::from_file(Self::PTX_PATH);
//             device.load_ptx(ptx, "kernels", &["i32_to_f64_kernel"])?;
//             let f: __CudaFunction =
//                 device.get_func("kernels", "i32_to_f64_kernel")
//                 .ok_or(spindle::error::function_not_found(Self::PTX_PATH, "i32_to_f64_kernel"))?;
//             let config: __LaunchConfig = __LaunchConfig::for_num_elems(n as u32);
//             unsafe { f.launch(config, (&mut slice, n)) }?;
//             Ok(slice.into())
//         }
//     }
// }
// pub use __i32_to_f64::__I32ToF64;

// add fn as a [i32,] -> [f64,] to database
// tag in database with #example_01

spindle::spin!(#example_01, U = i32 | f64);

// write new union to examples/01-map.rs
// #[repr(C)]
// union U {
//     _0: i32,
//     _1: f64,
// }

// write new union impls to examples/01-map.rs
// unsafe impl spindle::__cudarc::DeviceRepr for U {}
// unsafe impl spindle::__union::RawConvert<i32> for U {}
// unsafe impl spindle::__union::RawConvert<f64> for U {}

// get the only #example_01 map from the database
// assemble the example_01 crate
// and implement the method on spindle::DevSlice<U, i32>
// unsafe impl __i32_to_f64::__I32ToF64 for spindle::DevSlice<U, i32> {
//     type U = U;
//     type Return = spindle::DevSlice<U, f64>;
//     const PTX_PATH: &'static str = "target/spindle/crates/example_01/target/nvptx64-nvidia-cuda/target/release/kernel.ptx";
// }

fn main() -> spindle::Result<()> {
    let nums: Vec<i32> = (0..10).collect();
    let spindle: spindle::DevSlice<U, i32> = nums.try_into()?;
    let spindle: spindle::DevSlice<U, f64> = spindle.i32_to_f64(10)?;
    let spindle: spindle::HostSlice<U, f64> = spindle.try_to_host()?;
    assert!(spindle.iter().enumerate().all(|(i, x)| {
        *x == i32_to_f64(i as i32)
    }));
    Ok(())
}
