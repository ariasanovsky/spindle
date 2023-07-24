#[derive(Debug)]
pub enum Error {
    CudaError(cudarc::driver::result::DriverError),
    IoError(std::io::Error),
    FunctionNotFound,
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