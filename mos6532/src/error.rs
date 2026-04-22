use emutils::{pin::PinError, reg::RegisterError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RiotError {
    #[error(transparent)]
    PinError(#[from] PinError),

    #[error(transparent)]
    RegisterError(#[from] RegisterError),

    #[error("non-standard usage not yet implemented: {0}")]
    NonStandardUsage(String),
}
