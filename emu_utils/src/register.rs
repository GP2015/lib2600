mod bit;
mod mbit;
mod val;

pub use crate::register::{bit::BitRegister, mbit::MBitRegister, val::ValueRegister};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RegisterError {
    #[error("cannot access {name} register as it is uninitialised")]
    RegisterUninitialised { name: String },

    #[error("cannot access bit {bit} of {name} register as it is uninitialised")]
    RegisterBitUninitialised { name: String, bit: usize },
}
