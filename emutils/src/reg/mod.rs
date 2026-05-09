mod bit;
mod error;
mod mbit;

pub use crate::reg::{
    bit::{BitReg, state::BitRegState},
    error::RegError,
    mbit::{MBitReg, state::MBitRegState},
};
