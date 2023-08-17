use super::{SpindleCrate, error::Error};

impl SpindleCrate {
    pub fn exists(&self) -> bool {
        self.home.exists()
    }

    pub fn populate(&self) -> Result<(), Error> {
        self.write_toml_files()?;
        self.write_kernel_rs()?;
        self.write_device_rs()?;
        Ok(())
    }

    pub fn compile(&self) -> Result<(), Error> {
        todo!()
    }

    pub(crate) fn write_toml_files(&self) -> Result<(), Error> {
        todo!()
    }

    pub(crate) fn write_kernel_rs(&self) -> Result<(), Error> {
        let kernel_rs: proc_macro2::TokenStream = self.lib_rs();
        todo!()
    }

    pub(crate) fn write_device_rs(&self) -> Result<(), Error> {
        let device_rs: proc_macro2::TokenStream = self.device_rs();
        todo!()
    }

    pub(crate) fn lib_rs(&self) -> proc_macro2::TokenStream {
        todo!()
    }

    pub(crate) fn device_rs(&self) -> proc_macro2::TokenStream {
        todo!()
    }
}