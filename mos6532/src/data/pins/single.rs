pub mod contention;
pub mod input;

use crate::{PinState, RiotError};

pub trait SinglePin {
    fn read(&self) -> Result<bool, RiotError>;
    fn state(&self) -> Option<PinState>;
    fn set_signal_in(&mut self, state: PinState) -> Result<(), RiotError>;
    fn drive_in(&mut self, state: bool) -> Result<(), RiotError>;
    fn tri_state_in(&mut self);
}

pub trait SinglePinNew {
    fn new(name: String) -> Self;
}

pub trait SinglePinOutput {
    fn set_signal_out(&mut self, state: PinState) -> Result<(), RiotError>;
    fn drive_out(&mut self, state: bool) -> Result<(), RiotError>;
    fn tri_state_out(&mut self);
}
