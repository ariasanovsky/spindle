pub(super) static CARGO_TOML: &str = r#"[package]
name = "kernel"
version = "0.1.0"
edition = "2021"

[workspace]

[lib]
name = "kernel"
crate-type = ["cdylib"]
test = false
bench = false
"#;

pub(super) static RUST_TOOLCHAIN_TOML: &str = r#"[toolchain]
channel = "nightly"
"#;

pub(super) static CONFIG_TOML: &str = r#"[build]
target = "nvptx64-nvidia-cuda"
rustflags = ["--emit", "asm"]

[term]
color = "always" # "auto"
"#;

pub(super) static LIB_RS: &str = r#"#![no_std]
#![feature(abi_ptx)]
#![feature(stdsimd)]
#![feature(core_intrinsics)]

mod device;
use core::arch::nvptx::*;

#[panic_handler]
fn my_panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub unsafe extern "ptx-kernel" fn kernel(output: *mut i32, size: i32)
{
    let thread_id: i32 = _thread_idx_x();
    let block_id: i32 = _block_idx_x();
    
    let block_dim: i32 = _block_dim_x();
    let grid_dim: i32 = _grid_dim_x();
    
    let n_threads: i32 = block_dim * grid_dim;
    let thread_index: i32 =  thread_id + block_id * block_dim;
    
    let mut i: i32 = thread_index;
    while i < size {
        let value = device::device(i);
        *output.offset(i as isize) = value;
        i = i.wrapping_add(n_threads);
    }
    // while i < end && i < size {
    //     let value = device::device(i);
    //     *output.offset(i as isize) = value;
    //     i = i.wrapping_add(1);
    // }
}
"#;
