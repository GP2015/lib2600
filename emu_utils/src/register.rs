mod bit;
mod mbit;
mod val;

pub use crate::register::{bit::BitRegister, mbit::MBitRegister, val::ValueRegister};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RegisterError {
    #[error("cannot access {name} register as it is uninitialised")]
    ReadUndefined { name: String },

    #[error("cannot access bit {bit} of {name} register as it is uninitialised")]
    ReadUndefinedBit { name: String, bit: usize },

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
