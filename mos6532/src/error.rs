use emutils::line::LineError;
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum RiotError {
    #[error(transparent)]
    LineError(#[from] LineError),

    #[error("bus {name} has size {actual_size} when it should have size {required_size}")]
    InvalidBusSize {
        name: String,
        required_size: usize,
        actual_size: usize,
    },
    // #[error("non-standard usage not yet implemented: {0}")]
    // NonStandardUsage(String),
}
