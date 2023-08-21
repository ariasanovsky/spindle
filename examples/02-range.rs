use std::ops::Range;

use spindle::{DevSlice, HostSlice};

// #[spindle::init(#example_02)]
fn square_over_two(x: u64) -> f32 {
    ((x * x) / 2) as f32
}

// writes the trait
mod __square_over_two {
    pub trait __SquareOverTwo {
        type Return;
        fn square_over_two(&self) -> spindle::Result<Self::Return>;
    }
}
use __square_over_two::__SquareOverTwo;

// spindle::spin!(#example_02, U = u64);
// defines the union and implements the trait
#[repr(C)]
pub union U {
    _0: f32,
}
unsafe impl spindle::__cudarc::DeviceRepr for U {}
unsafe impl spindle::__union::RawConvert<f32> for U {}
impl __square_over_two::__SquareOverTwo for Range<u64> {
    type Return = spindle::DevSlice<U, f32>;

    fn square_over_two(&self) -> spindle::Result<Self::Return> {
        todo!("implement range/init maps");
    }
}

fn main() -> spindle::Result<()> {
    let nums: DevSlice<U, f32> = (100_000..300_000).square_over_two()?;
    let nums: HostSlice<U, f32> = nums.try_to_host()?;
    nums.iter().enumerate().for_each(|(i, x)| {
        assert_eq!(*x, square_over_two(i as u64 + 100_000));
    });
    Ok(())
}
