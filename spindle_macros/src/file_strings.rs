pub(crate) static CARGO_TOML: &str = r#"[package]
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

pub(crate) static RUST_TOOLCHAIN_TOML: &str = r#"[toolchain]
channel = "nightly"
"#;

pub(crate) static CONFIG_TOML: &str = r#"[build]
target = "nvptx64-nvidia-cuda"
rustflags = ["--emit", "asm"]

[term]
color = "always" # "auto"
"#;
