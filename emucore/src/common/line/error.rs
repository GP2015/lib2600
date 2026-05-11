use thiserror::Error;

#[derive(Clone, Debug, Eq, Error, Hash, PartialEq)]
pub enum LineError {
    #[error("line {name}{bit} does not exist")]
    BitOutOfRange {
        name: String,
        bit: usize,
        size: usize,
    },

    #[error("cannot drive value {value} to {name} bus without wrapping")]
    DriveValueTooLarge {
        name: String,
        value: usize,
        size: usize,
    },

    #[error("cannot perform operation on line {name} without causing a short circuit")]
    ShortCircuit { name: String },

    #[error("cannot read line {name} as it has no possible signal through it")]
    ImpossibleLineSignal { name: String },

    #[error("cannot access connection on line {name} with non-existant ID")]
    ConnectionIdOutOfBounds { name: String },
}
