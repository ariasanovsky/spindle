# Spindle

Spindle is a Rust crate simplifies GPGPU multithreaded execution of embarrassingly parallel and (soon) data parallel tasks.

```rust
#[spindle::basic_range]
fn square_over_two(x: i32) -> i32 {
    (x * x) / 2
}

fn main() {
    let foo = unsafe { 33.square_over_two() }.unwrap();
    println!("{foo:?}");
    let bar: Box<[_; 33]> = unsafe { _square_over_two() }.unwrap();
    println!("{bar:?}");
}
/*
[0, 0, 2, 4, 8, 12, 18, 24, 32, 40, 50, 60, 72, 84, 98, 112, 128, 144, 162, 180, 200, 220, 242, 264, 288, 312, 338, 364, 392, 420, 450, 480, 512]
[0, 0, 2, 4, 8, 12, 18, 24, 32, 40, 50, 60, 72, 84, 98, 112, 128, 144, 162, 180, 200, 220, 242, 264, 288, 312, 338, 364, 392, 420, 450, 480, 512]
*/
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
