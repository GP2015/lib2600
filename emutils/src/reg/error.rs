use thiserror::Error;

#[derive(Clone, Debug, Eq, Error, Hash, PartialEq)]
pub enum RegisterError {
    #[error("register {name} has no bit {bit}")]
    BitOutOfRange {
        name: String,
        bit: usize,
        size: usize,
    },

    #[error("cannot write value {value:x} to register {name} without wrapping")]
    WriteValueTooLarge {
        name: String,
        value: usize,
        size: usize,
    },
}
