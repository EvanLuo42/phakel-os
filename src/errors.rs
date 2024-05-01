use alloc::string::String;

use thiserror_no_std::Error;

#[derive(Error, Debug)]
pub enum KernelError {
    #[error("Failed to convert from {from:?} to {to:?}: {reason:?}")]
    Conversion {
        from: String,
        to: String,
        reason: String
    },
    #[error("Frame {ppn:?} has not been allocated")]
    Deallocate {
        ppn: usize
    }
}