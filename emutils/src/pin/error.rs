use thiserror::Error;

#[derive(Error, Debug)]
pub enum PinError {
    #[error("pin {name}{bit} does not exist")]
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

    #[error("cannot perform operation on pin {name} without causing a short circuit")]
    ShortCircuit { name: String },
}
