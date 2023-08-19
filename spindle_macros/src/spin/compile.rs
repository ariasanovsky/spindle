use std::{io::{Read, Write}, process::{Command, Output}};

use super::{SpindleCrate, error::Error};

#[derive(Debug)]
pub enum CompilationStatus {
    Failed { colored_output: String, colorless_output: String }, // both exist, but kernel.ptx does not
    Succeeded { colored_output: String, colorless_output: String }, // all files exist
}

fn output_string(output: &Output) -> String {
    let lossy_error = String::from_utf8_lossy(&output.stderr);
    let lossy_output = String::from_utf8_lossy(&output.stdout);
    match (lossy_error.is_empty(), lossy_output.is_empty()) {
        (true, true) => panic!("no colorless output???"),
        (true, false) => lossy_output.to_string(),
        (false, true) => lossy_error.to_string(),
        (false, false) => format!("{lossy_error}\n{lossy_output}"),
    }
}

impl SpindleCrate {
    pub fn ptx_path(&self) -> std::path::PathBuf {
        // e.g., `target/nvptx64-nvidia-cuda/release/kernel.ptx`
        self.home.join("target").join("nvptx64-nvidia-cuda").join("release").join("kernel.ptx")
    }

    pub fn status(&self) -> Result<CompilationStatus, Error> {
        // if the crate does not exist, make it
        if !self.exists() {
            self.populate()?;
            // format the crate
            let _ = self.format()?;
        }
        // try to get the colored compiler output
        let colored_output: String = self.get_colorful_compile_output()?;
        // try to get the colorless compiler output
        let colorless_output: String = self.get_colorless_compile_output()?;
        // use the existence of kernel.ptx to determine compilation status
        Ok(if self.ptx_path().exists() {
            CompilationStatus::Succeeded { colored_output, colorless_output }
        } else {
            CompilationStatus::Failed { colored_output, colorless_output }
        })
    }

    fn get_colorful_compile_output(&self) -> Result<String, Error> {
        // look for `out_colorful.txt`
        let out_colorful_path = self.home.join("out_colorful.txt");
        // if it exists, read it
        Ok(if out_colorful_path.exists() {
            let mut file = std::fs::File::open(out_colorful_path)?;
            let mut contents = String::new();
            let _: usize = file.read_to_string(&mut contents)?;
            contents
        } else {
            // otherwise, compile the crate
            let mut cmd = Command::new("cargo");
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
            let colorful_output = output_string(&output);
            let mut file = std::fs::File::create(out_colorful_path)?;
            file.write_all(colorful_output.as_bytes())?;
            colorful_output
        })
    }

    fn get_colorless_compile_output(&self) -> Result<String, Error> {
        // look for `out_colorless.txt`
        let out_colorless_path = self.home.join("out_colorless.txt");
        // if it exists, read it
        Ok(if out_colorless_path.exists() {
            let mut file = std::fs::File::open(out_colorless_path)?;
            let mut contents = String::new();
            let _: usize = file.read_to_string(&mut contents)?;
            contents
        } else {
            // otherwise, compile the crate
            let mut cmd = Command::new("cargo");
            cmd.args([
                "+nightly",
                "-Z",
                "unstable-options",
                "-C",
                &self.home.to_string_lossy(),
                "build",
                "--release",
                "--color",
                "never",
            ]);
            let output = cmd.output()?;
            let colorless_output = output_string(&output);
            let mut file = std::fs::File::create(out_colorless_path)?;
            file.write_all(colorless_output.as_bytes())?;
            colorless_output
        })
    }
}

impl CompilationStatus {
    pub fn print_colorful_output(&self) {
        match self {
            Self::Failed { colored_output, .. } => println!("{colored_output}"),
            Self::Succeeded { colored_output, .. } => println!("{colored_output}"),
        }
    }
}