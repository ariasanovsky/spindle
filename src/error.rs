#[derive(Debug)]
pub enum Error {
    CudaError(cudarc::driver::result::DriverError),
    FunctionNotFound,
}

impl From<cudarc::driver::result::DriverError> for Error {
    fn from(value: cudarc::driver::result::DriverError) -> Self {
        Self::CudaError(value)
    }
}
