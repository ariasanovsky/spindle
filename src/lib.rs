use cudarc::driver::{CudaSlice, DeviceRepr};
pub use spindle_macros::{basic_range, map, spin};

pub mod error;
pub mod spindle;

use spindle::RawConvert;

pub type Result<T> = std::result::Result<T, error::Error>;

pub struct DevSpindle<U, X>(CudaSlice<U>, std::marker::PhantomData<X>)
where
    U: RawConvert<X> + DeviceRepr,
    X: Copy;

pub struct HostSpindle<U, X>(Vec<U>, std::marker::PhantomData<X>)
where
    U: RawConvert<X>,
    X: Copy;

pub mod range {
    #[derive(Debug)]
    pub enum Error {
        AllocationFailed,
        DriverError(cudarc::driver::result::DriverError),
        LayoutError(core::alloc::LayoutError),
        KernelNotFound,
        LengthMismatch,
    }

    impl From<cudarc::driver::result::DriverError> for Error {
        fn from(error: cudarc::driver::result::DriverError) -> Self {
            Error::DriverError(error)
        }
    }

    impl From<core::alloc::LayoutError> for Error {
        fn from(error: core::alloc::LayoutError) -> Self {
            Error::LayoutError(error)
        }
    }
}
