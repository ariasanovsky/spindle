// use spindle::{DevSlice, HostSlice};
// use spindle::U64; // pub type U64 = u64;
// use spindle::Any; // pub struct Any;
// #[spindle::map(#example_02)]
// fn square_over_two(x: Any, i: U64) -> u64 {
//     (x * x) / 2
// }

// spindle::spin!(#example_02, U = u64);

fn main() -> spindle::Result<()> {
    todo!("implement range maps");
    // let nums: DevSlice<U, u64> = (0..100_000).square_over_two()?;
    // let nums: HostSlice<U, u64> = nums.try_to_host()?;
    // nums.iter().enumerate().for_each(|(i, x)| {
    //     assert_eq!(x, square_over_two(i as u64));
    // });
    // Ok(())
}
