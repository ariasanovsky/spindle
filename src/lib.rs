pub use spindle_macros::{init, map, spin};

pub mod __cudarc;
pub mod __union;

pub mod error;
pub mod spindle;

pub type Result<T> = std::result::Result<T, error::Error>;

pub struct DevSlice<U, X>(__cudarc::CudaSlice<U>, std::marker::PhantomData<X>)
where
    U: __union::RawConvert<X> + __cudarc::DeviceRepr,
    X: Copy;

pub struct HostSlice<U, X>(Vec<U>, std::marker::PhantomData<X>)
where
    U: __union::RawConvert<X>,
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
