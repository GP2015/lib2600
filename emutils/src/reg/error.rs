use thiserror::Error;

#[derive(Error, Debug)]
pub enum RegisterError {
    #[error("register {name} does not have a bit {bit}")]
    BitOutOfRange {
        name: String,
        bit: usize,
        size: usize,
    },

    #[error("cannot write value {value} to {name} register without wrapping")]
    WriteValueTooLarge {
        name: String,
        value: usize,
        size: usize,
    },
}
