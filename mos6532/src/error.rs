use emutils::pin::PinError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RiotError {
    #[error(transparent)]
    PinError(#[from] PinError),

    #[error("non-standard usage not yet implemented: {0}")]
    NonStandardUsage(String),
}
