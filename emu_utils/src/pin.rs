mod bus;
mod single;
mod state;

pub use crate::pin::{
    bus::{BusCore, BusInterface, BusOutput, standard::StandardBus},
    single::{
        SinglePinCore, SinglePinInterface, SinglePinOutput, contention::ContentionPin,
        input::InputPin,
    },
    state::PinState,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PinError {
    #[error("cannot access {name} pin as it is currently undefined")]
    ReadUndefined { name: String },

    #[error("cannot read {name} pin as it is currently tri-stated")]
    ReadTriStated { name: String },

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
        current_state: PinState,
        next_state: PinState,
    },

    #[error(
        "cannot resolve contention on pin {name} since it is being driven with an undefined state"
    )]
    PotentialShortCircuit { name: String },
}
