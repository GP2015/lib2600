mod bit;
mod error;
mod mbit;
mod val;

pub use crate::register::{
    bit::BitRegister, error::RegisterError, mbit::MBitRegister, val::ValueRegister,
};
