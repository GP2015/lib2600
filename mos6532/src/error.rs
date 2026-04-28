use emutils::{line::LineError, reg::RegisterError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RiotError {
    #[error(transparent)]
    LineError(#[from] LineError),

    #[error(transparent)]
    RegisterError(#[from] RegisterError),

    #[error("non-standard usage not yet implemented: {0}")]
    NonStandardUsage(String),
}
