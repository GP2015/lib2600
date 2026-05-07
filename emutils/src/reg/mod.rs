mod bit;
mod error;
mod mbit;

pub use crate::reg::{
    bit::{BitRegister, state::BitRegisterState},
    error::RegisterError,
    mbit::{MBitRegister, state::MBitRegisterState},
};
