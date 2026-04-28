use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
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

    #[error("register {reg_name} and bus {bus_name} have incompatible sizes")]
    IncompatibleBus {
        reg_name: String,
        bus_name: String,
        reg_size: usize,
        bus_size: usize,
    },
}
