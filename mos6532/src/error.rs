use emu_utils::{pin::PinError, register::RegisterError};
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

impl RiotError {
    pub(crate) fn non_standard(s: &str) -> Self {
        Self::NonStandardUsage(String::from(s))
    }
}
