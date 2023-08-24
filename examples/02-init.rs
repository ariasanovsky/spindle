use spindle::{DevSlice, HostSlice};

#[spindle::init(#example_02)]
fn square_over_two(x: u64) -> f32 {
    ((x * x) / 2) as f32
}

// writes the trait
// mod __square_over_two {
//     pub unsafe trait __SquareOverTwo {
//         type Return;
//         const PTX_PATH: &'static str;
//         fn square_over_two(&self, size: i32) -> spindle::Result<Self::Return>;
//     }
// }
// use __square_over_two::__SquareOverTwo;

spindle::spin!(#example_02, U = f32);
// defines the union and implements the trait
// #[repr(C)]
// pub union U {
//     _0: f32,
// }
// unsafe impl spindle::__cudarc::DeviceRepr for U {}
// unsafe impl spindle::__union::RawConvert<f32> for U {}
unsafe impl __square_over_two::__SquareOverTwo for std::ops::RangeFrom<u64> {
    type Return = spindle::DevSlice<U, f32>;
    const PTX_PATH: &'static str = "target/spindle/map/example_02/target/nvptx64-nvidia-cuda/target/release/kernel.ptx";
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

fn main() -> spindle::Result<()> {
    let nums: DevSlice<U, f32> = (100_000..).square_over_two(200_000)?;
    let nums: HostSlice<U, f32> = nums.try_to_host()?;
    nums.iter().enumerate().for_each(|(i, x)| {
        assert_eq!(*x, square_over_two(i as u64 + 100_000));
    });
    Ok(())
}
