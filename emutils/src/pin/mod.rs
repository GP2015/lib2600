mod error;
mod possible;
mod signal;

pub(crate) mod bus;
pub(crate) mod single;

pub use crate::pin::{
    bus::{
        defs::{BusCore, BusInputUI, BusOutput},
        standard::StandardBus,
    },
    error::PinError,
    signal::PinSignal,
    single::{
        contention::ContentionPin,
        defs::{PinCore, PinInputUI, PinOutput},
        input::InputPin,
    },
};
