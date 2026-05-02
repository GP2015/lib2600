use emutils::line::LineError;
use thiserror::Error;

#[derive(Clone, Debug, Eq, Error, Hash, PartialEq)]
pub enum RiotError {
    #[error(transparent)]
    LineError(#[from] LineError),
}
