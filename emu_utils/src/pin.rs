mod bus;
mod single;
mod state;

pub use crate::pin::{
    bus::{Bus, BusOutput, contention::ContentionBus, input::InputBus},
    single::{SinglePin, SinglePinOutput, contention::ContentionPin, input::InputPin},
    state::PinState,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PinError {
    #[error("cannot access {name} pin as it is uninitialised")]
    PinUninitialised { name: String },

    #[error("cannot read {name} pin as it is currently tri-stated")]
    PinReadWhileTriStated { name: String },

    #[error("pin {name}{bit} does not exist")]
    BusPinOutOfRange {
        name: String,
        bit: usize,
        size: usize,
    },

    #[error("cannot drive value {value} to {name} bus without wrapping")]
    BusDriveValueTooLarge {
        name: String,
        value: usize,
        size: usize,
    },

    #[error("cannot drive pin {name} {next_state} as it is currently being driven {current_state}")]
    ShortCircuit {
        name: String,
        current_state: PinState,
        next_state: PinState,
    },

    #[error(
        "cannot drive pin {name} {state} as it is currently being driven with an unknown state"
    )]
    PotentialShortCircuit { name: String, state: PinState },
}
