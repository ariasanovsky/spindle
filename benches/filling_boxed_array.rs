#![allow(unused)]

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;

#[spindle::init(#bench_filling_boxed_array)]
fn i32_identity(x: i32) -> i32 {
    x
}

// spindle::spin!(#example_02, U = f32);
#[repr(C)]
pub union U {
    _0: i32,
}
unsafe impl spindle::__cudarc::DeviceRepr for U {}
unsafe impl spindle::__union::RawConvert<i32> for U {}

unsafe impl __i32_identity::__I32Identity for std::ops::RangeFrom<i32> {
    type Return = spindle::DevSlice<U, i32>;
    const PTX_PATH: &'static str = "target/spindle/map/example_02/target/nvptx64-nvidia-cuda/target/release/kernel.ptx";
    fn i32_identity(&self, size: i32) -> spindle::Result<Self::Return> {
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
        device.load_ptx(ptx, "kernels", &["i32_identity_kernel"])?;
        let f: __CudaFunction = device.get_func(
            "kernels",
            "i32_identity_kernel"
        )
        .ok_or(spindle::error::function_not_found(
            Self::PTX_PATH,
            "i32_identity_kernel"
        ))?;
        let config: __LaunchConfig = __LaunchConfig::for_num_elems(size as u32);
        unsafe { f.launch(config, (&mut slice, size)) }?;
        Ok(spindle::DevSlice::from(slice))
    }
}

fn rayon_vec(x: i32) -> Vec<i32> {
    (0..x).into_par_iter().collect()
}

fn bench_square_over_two(c: &mut Criterion) {
    let mut group = c.benchmark_group("filling_boxed_array");
    for n in [
        10_000i32,
        100_000,
        1_000_000,
        10_000_000,
        100_000_000,
        1_000_000_000,
    ] {
        group.bench_with_input(BenchmarkId::new("spindle identity", n), &n, |b, &n| {
            b.iter(|| black_box((0..).i32_identity(n).unwrap()))
        });

        group.bench_with_input(BenchmarkId::new("rayon identity", n), &n, |b, &n| {
            b.iter(|| black_box(rayon_vec(n)))
        });
    }
}

criterion_group!(benches, bench_square_over_two);
criterion_main!(benches);
