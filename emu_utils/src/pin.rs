mod bus;
mod signal;
mod single;
mod state;

pub use crate::pin::{
    bus::{BusCore, BusInterface, BusOutput, standard::StandardBus},
    signal::PinSignal,
    single::{
        SinglePinCore, SinglePinInterface, SinglePinOutput, contention::ContentionPin,
        input::InputPin,
    },
    state::PinState,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PinError {
    #[error("pin {name}{bit} does not exist")]
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

    #[error("cannot drive pin {name} {next_state} as it is currently being driven {current_state}")]
    ShortCircuit {
        name: String,
        current_state: PinSignal,
        next_state: PinSignal,
    },

    #[error("cannot resolve contention on pin {name} since it may currently be driven ???")]
    PotentialShortCircuit { name: String },
}
