use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
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

    #[error("bus {bus_name} and register {reg_name} have incompatible sizes")]
    IncompatibleRegister {
        bus_name: String,
        reg_name: String,
        bus_size: usize,
        reg_size: usize,
    },

    #[error("buses {out_name} and {source_name} have incompatible sizes")]
    IncompatibleBus {
        out_name: String,
        source_name: String,
        out_size: usize,
        source_size: usize,
    },
}
