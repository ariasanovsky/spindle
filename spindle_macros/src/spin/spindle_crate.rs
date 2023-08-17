use std::io::Write;

use crate::file_strings::{CARGO_TOML, CONFIG_TOML, RUST_TOOLCHAIN_TOML};

use super::{SpindleCrate, error::Error};

impl SpindleCrate {
    pub fn exists(&self) -> bool {
        self.home.exists()
    }

    pub fn populate(&self) -> Result<(), Error> {
        self.create_home()?;
        self.write_toml_files()?;
        self.create_src()?;
        self.write_lib_rs()?;
        self.write_device_rs()?;
        Ok(())
    }

    pub fn compile(&self) -> Result<std::process::Output, Error> {
        let mut cmd = std::process::Command::new("cargo");
        cmd.args([
            "+nightly",
            "-Z",
            "unstable-options",
            "-C",
            &self.home.to_string_lossy(),
            "fmt",
        ]);
        let _ = cmd.output()?;
        let mut cmd = std::process::Command::new("cargo");
        cmd.args([
            "+nightly",
            "-Z",
            "unstable-options",
            "-C",
            &self.home.to_string_lossy(),
            "build",
            "--release",
        ]);
        let output = cmd.output()?;
        Ok(output)
    }

    pub(crate) fn write_toml_files(&self) -> Result<(), Error> {
        self.write_cargo_toml()?;
        self.write_config_toml()?;
        self.write_rust_toolchain_toml()?;
        Ok(())
    }

    pub(crate) fn write_lib_rs(&self) -> Result<(), Error> {
        let kernel_rs: proc_macro2::TokenStream = self.lib_rs();
        let kernel_rs_path = self.home.join("src").join("lib.rs");
        let mut file = std::fs::File::create(&kernel_rs_path)?;
        file.write_all(kernel_rs.to_string().as_bytes())?;
        Ok(())
    }

    pub(crate) fn write_device_rs(&self) -> Result<(), Error> {
        let device_rs: proc_macro2::TokenStream = self.device_rs();
        let device_rs_path = self.home.join("src").join("device.rs");
        let mut file = std::fs::File::create(&device_rs_path)?;
        file.write_all(device_rs.to_string().as_bytes())?;
        Ok(())
    }

    pub(crate) fn write_cargo_toml(&self) -> Result<(), Error> {
        let toml_path = self.home.join("Cargo.toml");
        let mut file = std::fs::File::create(&toml_path)?;
        file.write_all(CARGO_TOML.as_bytes())?;
        Ok(())
    }

    pub(crate) fn write_config_toml(&self) -> Result<(), Error> {
        let dot_cargo_path = self.home.join(".cargo");
        std::fs::create_dir_all(&dot_cargo_path)?;
        let config_toml_path = dot_cargo_path.join("config.toml");
        let mut file = std::fs::File::create(&config_toml_path)?;
        file.write_all(CONFIG_TOML.as_bytes())?;
        Ok(())
    }

    pub(crate) fn write_rust_toolchain_toml(&self) -> Result<(), Error> {
        let rust_toolchain_toml_path = self.home.join("rust-toolchain.toml");
        let mut file = std::fs::File::create(&rust_toolchain_toml_path)?;
        file.write_all(RUST_TOOLCHAIN_TOML.as_bytes())?;
        Ok(())
    }

    pub(crate) fn create_home(&self) -> Result<(), Error> {
        std::fs::create_dir_all(&self.home)?;
        Ok(())
    }

    pub(crate) fn create_src(&self) -> Result<(), Error> {
        let src_path = self.home.join("src");
        std::fs::create_dir_all(&src_path)?;
        Ok(())
    }
}