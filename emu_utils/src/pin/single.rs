pub mod contention;
pub mod input;

use crate::pin::{PinError, PinState};

pub trait SinglePin {
    fn read(&self) -> Result<bool, PinError>;
    fn state(&self) -> Option<PinState>;
    fn set_signal_in(&mut self, state: PinState) -> Result<(), PinError>;
    fn drive_in(&mut self, state: bool) -> Result<(), PinError>;
    fn tri_state_in(&mut self);
}

pub trait SinglePinNew {
    fn new(name: String) -> Self;
}

pub trait SinglePinOutput {
    fn set_signal_out(&mut self, state: PinState) -> Result<(), PinError>;
    fn drive_out(&mut self, state: bool) -> Result<(), PinError>;
    fn tri_state_out(&mut self);
}
