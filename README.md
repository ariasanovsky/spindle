# Spindle

Spindle is a Rust crate simplifies GPGPU multithreaded execution of embarrassingly parallel and data parallel tasks.

```rust
#[spindle::map]
fn _i32_to_f64(x: i32) -> f64 {
    x as f64
}

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
```

## License

Dual-licensed to be compatible with the Rust project.

Licensed under the [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0) or the [MIT license](http://opensource.org/licenses/MIT), at your option.
This file may not be copied, modified, or distributed except according to those terms.

## Contributing

Contributions are welcome!
If you have any ideas for new features or improvements, feel free to open an issue or submit a pull request.

## Acknowledgments

Spindle was inspired by many! `todo!()`
