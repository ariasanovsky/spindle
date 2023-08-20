// use spindle::{DevSlice, HostSlice};
// #[spindle::init(#example_02)]
// fn square_over_two(x: u64) -> f32 {
//     ((x * x) / 2) as f32
// }

// spindle::spin!(#example_02, U = u64);

fn main() -> spindle::Result<()> {
    todo!("implement range maps");
    // let nums: DevSlice<U, u64> = (100_000..200_000).square_over_two()?;
    // let nums: HostSlice<U, u64> = nums.try_to_host()?;
    // nums.iter().enumerate().for_each(|(i, x)| {
    //     assert_eq!(x, square_over_two(i as u64));
    // });
    // Ok(())
}
