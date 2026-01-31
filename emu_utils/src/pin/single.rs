pub mod contention;
pub mod input;

use crate::pin::{PinError, PinState};

pub trait SinglePin {
    type Error: From<PinError>;
    fn read(&self) -> Result<bool, Self::Error>;
    fn state(&self) -> Option<PinState>;
    fn set_signal_in(&mut self, state: PinState) -> Result<(), Self::Error>;
    fn drive_in(&mut self, state: bool) -> Result<(), Self::Error>;
    fn tri_state_in(&mut self);
}

pub trait SinglePinNew {
    fn new(name: String) -> Self;
}

pub trait SinglePinOutput {
    type Error: From<PinError>;
    fn set_signal_out(&mut self, state: PinState) -> Result<(), Self::Error>;
    fn drive_out(&mut self, state: bool) -> Result<(), Self::Error>;
    fn tri_state_out(&mut self);
}
