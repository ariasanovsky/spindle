use std::sync::Arc;

use cudarc::{driver::{CudaSlice, CudaDevice, CudaFunction, LaunchConfig, LaunchAsync, DeviceRepr, DeviceSlice}, nvrtc::Ptx};

#[spindle::map]
fn _i32_to_f64(x: i32) -> f64 {
    x as f64
}

unsafe trait I32ToF64U
where
    <Self as I32ToF64U>::U: DeviceRepr,
    Self: Into<CudaSlice<Self::U>>,
    CudaSlice<<Self as I32ToF64U>::U>: Into<<Self as I32ToF64U>::Return>,
{
    type U;
    type Return;
    fn i32_to_f64(self) -> Result<Self::Return, spindle::error::Error>
    {
        let mut slice: CudaSlice<Self::U> = self.into();
        let n: usize = slice.len();
        let device: Arc<CudaDevice> = slice.device();
        device.load_ptx(Ptx::from_file("target/spindle/i32_to_f64"), "kernel", &["kernel"])?;
        let f: CudaFunction = device.get_func("kernel", "kernel")
            .ok_or(spindle::error::Error::FunctionNotFound)?;
        let config: LaunchConfig = LaunchConfig::for_num_elems(n as u32);
        unsafe { f.launch(config, (&mut slice, n as i32)) }?;
        Ok(slice.into())
    }

}

fn main() -> Result<(), spindle::error::Error> {
    spindle::spin!(U, i32, f64);
    
    // union U {
    //     i32: i32,
    //     f64: f64,
    // }
    // unsafe impl RawConvert<i32> for U {}
    // unsafe impl RawConvert<f64> for U {}
    // unsafe impl DeviceRepr for U {}
    unsafe impl I32ToF64U for spindle::DevSpindle<U, i32> {
        type U = U;
        type Return = spindle::DevSpindle<U, f64>;
    }
    let nums: Vec<i32> = (0..10).collect();
    let spindle: spindle::DevSpindle<U, i32> = nums.try_into()?;
    let spindle: spindle::DevSpindle<U, f64> = spindle.i32_to_f64()?;
    let spindle: spindle::HostSpindle<U, f64> = spindle.try_to_host()?;
    for (i, x) in spindle.iter().enumerate() {
        println!("{}: {}", i, x);
    }
    Ok(())
}