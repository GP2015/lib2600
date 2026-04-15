use emu_utils::{pin::PinError, register::RegisterError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RiotError {
    #[error(transparent)]
    PinError(#[from] PinError),

    #[error(transparent)]
    RegisterError(#[from] RegisterError),
}
