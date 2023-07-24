#[spindle::map]
fn _i32_to_f64(x: i32) -> f64 {
    x as f64
}


// mod __i32_to_f64
// {
//     use cudarc ::
//     {
//         driver ::
//         {
//             CudaDevice, CudaFunction, CudaSlice, DeviceRepr, DeviceSlice,
//             LaunchAsync, LaunchConfig,
//         }, nvrtc :: Ptx,
//     } ; use spindle :: error :: Error ; use std ::
//     { path :: PathBuf, sync :: Arc } ; pub unsafe trait I32ToF64 where < Self
//     as I32ToF64 > :: U : DeviceRepr, Self : Into < CudaSlice < Self :: U >>,
//     CudaSlice << Self as I32ToF64 > :: U > : Into << Self as I32ToF64 > ::
//     Return >,
//     {
//         type U ; type Return ; unsafe fn i32_to_f64(self) -> Result < Self ::
//         Return, Error >
//         {
//             let mut slice : CudaSlice < Self :: U > = self.into() ; let n :
//             usize = slice.len() ; let device : Arc < CudaDevice > =
//             slice.device() ; let ptx_path : PathBuf = PathBuf ::
//             from("target/spindle/map/").join(U).join("target").join("nvptx64-nvidia-cuda").join("release").join("kernel.ptx")
//             ; let ptx_contents : String = std :: fs ::
//             read_to_string(& ptx_path) ? ; let _res : () =
//             device.load_ptx(Ptx :: from_src(ptx_contents), "kernel", &
//             ["i32_to_f64_kernel"],) ? ; let f : CudaFunction =
//             device.get_func("kernel",
//             "i32_to_f64_kernel").ok_or(Error :: FunctionNotFound) ? ; let
//             config : LaunchConfig = LaunchConfig :: for_num_elems(n as u32) ;
//             unsafe
//             {
//                 f.launch(config, (& mut slice, n as i32))
//             }.map(| () | slice.into()).map_err(Into :: into)
//         }
//     }
    
//     #[repr(C)] pub union U { _0 : i32, _1 : f64 }
    
//     unsafe impl I32ToF64 for spindle :: DevSpindle < U, i32 > {
//         type U = U ;
//         type Return = spindle :: DevSpindle < U, f64 > ;
//     }

//     unsafe impl spindle::spindle::RawConvert<i32> for U {}
//     unsafe impl spindle::spindle::RawConvert<f64> for U {}
//     unsafe impl cudarc::driver::DeviceRepr for U {}
// } use __i32_to_f64 :: * ;







fn main() -> Result<(), spindle::error::Error> {
    spindle::spin!(U, i32, f64);
    let nums: Vec<i32> = (0..10).collect();
    let spindle: spindle::DevSpindle<U, i32> = nums.try_into()?;
    let spindle: spindle::DevSpindle<U, f64> = unsafe { spindle.i32_to_f64() }?;
    let spindle: spindle::HostSpindle<U, f64> = spindle.try_to_host()?;
    for (i, x) in spindle.iter().enumerate() {
        assert_eq!(*x, i as f64);
    }
    Ok(())
}

// union U {
//     _0: i32,
//     _1: f64,
// }
// unsafe impl spindle::spindle::RawConvert<i32> for U {}
// unsafe impl spindle::spindle::RawConvert<f64> for U {}
// unsafe impl cudarc::driver::DeviceRepr for U {}

// use __i32_to_f64::I32ToF64;

// unsafe impl I32ToF64 for spindle::DevSpindle<U, i32> {
//     type U = U;
//     type Return = spindle::DevSpindle<U, f64>;
// }

// mod __i32_to_f64 {
//     use cudarc::{
//         driver::{
//             CudaDevice, CudaFunction, CudaSlice, DeviceRepr, DeviceSlice, LaunchAsync, LaunchConfig,
//         },
//         nvrtc::Ptx,
//     };
//     use spindle::error::Error;
//     use std::sync::Arc;
//     pub unsafe trait I32ToF64
//     where
//         <Self as I32ToF64>::U: DeviceRepr,
//         Self: Into<CudaSlice<Self::U>>,
//         CudaSlice<<Self as I32ToF64>::U>: Into<<Self as I32ToF64>::Return>,
//     {
//         type U;
//         type Return;
//         fn i32_to_f64(self) -> Result<Self::Return, Error> {
//             let mut slice: CudaSlice<Self::U> = self.into();
//             let n: usize = slice.len();
//             let device: Arc<CudaDevice> = slice.device();
//             let _res: () = device.load_ptx(
//                 Ptx::from_file("target/spindle/i32_to_f64"), // todo! panic -> error
//                 "kernel",
//                 &["kernel"],
//             )?;
//             let f: CudaFunction = device
//                 .get_func("kernel", "kernel")
//                 .ok_or(Error::FunctionNotFound)?;
//             let config: LaunchConfig = LaunchConfig::for_num_elems(n as u32); // todo! inspect cudarc fn
//             unsafe { f.launch(config, (&mut slice, n as i32)) }
//                 .map(|()| slice.into())
//                 .map_err(Into::into)
//         }
//     }
// }
