mod error;
mod possible;
mod signal;

pub(crate) mod bus;
pub(crate) mod core;

pub use crate::pin::{
    bus::{
        defs::{BusCore, BusInputUI, BusOutput},
        standard::StandardBus,
    },
    error::PinError,
    line::{
        defs::{PinCore, PinInput, PinQuery},
        standard::StandardPin,
    },
    signal::PinSignal,
};
