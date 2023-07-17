pub use spindle_macros::basic_range;

pub mod range {
    #[derive(Debug)]
    pub enum Error {
        AllocationFailed,
    }
}
