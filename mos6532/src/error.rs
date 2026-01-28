use thiserror::Error;

use crate::data::pins::state::PinState;

#[derive(Error, Debug)]
pub enum RiotError {
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

    #[error("cannot read RAM byte at {address:x} as it is uninitialised ")]
    RamByteUninitialised { address: usize },

    #[error("cannot access {name} register as it is uninitialised")]
    RegisterUninitialised { name: String },

    #[error("cannot access bit {bit} of {name} register as it is uninitialised")]
    RegisterBitUninitialised { name: String, bit: usize },
}
