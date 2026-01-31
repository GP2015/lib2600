mod bit;
mod mbit;
mod val;

pub use crate::register::{bit::BitReg, mbit::MBitReg, val::ValueReg};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RegError {
    #[error("cannot access {name} register as it is uninitialised")]
    RegisterUninitialised { name: String },

    #[error("cannot access bit {bit} of {name} register as it is uninitialised")]
    RegisterBitUninitialised { name: String, bit: usize },
}
