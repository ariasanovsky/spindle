// use spindle::{DevSlice, HostSlice};

// spindle::spin!(#example_03, U = u64);

// we also want something like:
// impl spindle::__sum::Summands for spindle::DevSlice<U, u64> {}

fn main() -> spindle::Result<()> {
    todo!("write a function to sum/reduce over a CudaSlice");
    // let nums: Vec<u64> = (0..100_000).collect();
    // let nums: DevSlice<U, u64> = nums.try_into()?;
    // let nums: DevSlice<U, u64> = nums.summands()?;
    // let nums: HostSlice<U, u64> = nums.try_to_host()?;
    // let sum: u64 = nums.iter().sum();
    // Ok(())
}
