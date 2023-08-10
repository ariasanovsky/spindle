#[spindle::map]
fn _i32_to_f64(x: i32) -> f64 {
    x as f64
}

fn main() -> spindle::Result<()> {
    spindle::spin!(U = i32 | f64);
    let nums: Vec<i32> = (0..10).collect();
    let spindle: spindle::DevSpindle<U, i32> = nums.try_into()?;
    let spindle: spindle::DevSpindle<U, f64> = unsafe { spindle.i32_to_f64() }?;
    let spindle: spindle::HostSpindle<U, f64> = spindle.try_to_host()?;
    assert!(spindle.iter().enumerate().all(|(i, x)| {
        *x == i as f64
    }));
    Ok(())
}
