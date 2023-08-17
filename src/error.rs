#[derive(Debug)]
pub enum Error {
    CudaError(cudarc::driver::result::DriverError),
    IoError(std::io::Error),
    FunctionNotFound(&'static str, &'static str),
}

impl From<cudarc::driver::result::DriverError> for Error {
    fn from(value: cudarc::driver::result::DriverError) -> Self {
        Self::CudaError(value)
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value)
    }
}

pub fn function_not_found(ptx_path: &'static str, kernel_name: &'static str) -> Error {
    Error::FunctionNotFound(ptx_path, kernel_name)
}
