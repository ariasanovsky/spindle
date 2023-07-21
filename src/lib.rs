pub use spindle_macros::{basic_range, spin};

pub mod spindle;

pub mod range {
    #[derive(Debug)]
    pub enum Error {
        AllocationFailed,
        DriverError(cudarc::driver::result::DriverError),
        LayoutError(core::alloc::LayoutError),
        KernelNotFound,
        LengthMismatch,
    }
}

impl From<cudarc::driver::result::DriverError> for range::Error {
    fn from(error: cudarc::driver::result::DriverError) -> Self {
        range::Error::DriverError(error)
    }
}

impl From<core::alloc::LayoutError> for range::Error {
    fn from(error: core::alloc::LayoutError) -> Self {
        range::Error::LayoutError(error)
    }
}
