use std::sync::Arc;

use cudarc::{driver::{CudaSlice, CudaDevice, CudaFunction, LaunchConfig, LaunchAsync, DeviceRepr}, nvrtc::Ptx};

fn _i32_to_f64(x: i32) -> f64 {
    x as f64
}

unsafe trait _I32ToF64
where
    <Self as _I32ToF64>::U: DeviceRepr,
    Self: Into<CudaSlice<Self::U>>,
    CudaSlice<<Self as _I32ToF64>::U>: Into<<Self as _I32ToF64>::Return>,
{
    type U;
    type Return;
    fn i32_to_f64(self, n: usize) -> Result<Self::Return, spindle::spindle::error::Error>
    {
        let mut slice: CudaSlice<Self::U> = self.into();
        let device: Arc<CudaDevice> = slice.device();
        device.load_ptx(Ptx::from_file("target/spindle/i32_to_f64"), "kernel", &["kernel"])?;
        let f: CudaFunction = device.get_func("kernel", "kernel")
            .ok_or(spindle::spindle::error::Error::FunctionNotFound)?;
        let config: LaunchConfig = LaunchConfig::for_num_elems(n as u32);
        unsafe { f.launch(config, (&mut slice, n as i32)) }?;
        Ok(slice.into())
    }

}

fn main() -> Result<(), spindle::spindle::error::Error> {
    Ok(())
}