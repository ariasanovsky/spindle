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

## Contributing

Welcome, idea-haver!

- Do you have a feature or syntax suggestion?
- Want to share ideas about bit fiddly implmentations?
- Did you know how to squash a bug?

Please skim the [Collaborator Guidelines](https://github.com/ariasanovsky/spindle/discussions/13) and say hello!

Thank you for your contributions!

## License

Dual-licensed to be compatible with the Rust project.

Licensed under the [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0) or the [MIT license](http://opensource.org/licenses/MIT), at your option.
This file may not be copied, modified, or distributed except according to those terms.

## Acknowledgments

(collaborators, feel free to expand)

Our work is inspired by so many great crates!
This includes, but is not limited to,

- [`rayon`](https://crates.io/crates/rayon), our inspiration in reliable fearless concurrency
- [`dfdx`](https://crates.io/crates/dfdx), for ergonomic machine learning
- [`faer`](https://crates.io/crates/faer), the uncompromisingly brilliant linear algebra backend
- [`cudarc`](https://crates.io/crates/cudarc), the smart-pointered, typesafe wrapper for `CUDA`

### [Alex](https://github.com/ariasanovsky/) is personally grateful for

- the [2023 Scientific Computing in Rust](https://scientificcomputing.rs/) conference
- the joy I felt when my first GPGPU project hit ~ `1_000_000_000_000_000` iterations in a single day
- the Rust community's committment to:
  - welcoming everyone,
  - tackling developer ergonomics,
  - teaching us correctness and safety, and
  - giving us tools to push the language further.
