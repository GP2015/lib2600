pub mod contention;
pub mod core;
pub mod input;

#[cfg(test)]
pub mod mock_pin;

use crate::pin::{PinError, PinState};

pub trait SinglePin {
    type Error: From<PinError>;
    fn state(&self) -> PinState;
    fn prev_state(&self) -> PinState;
    fn state_as_bool(&self) -> Option<bool>;
    fn prev_state_as_bool(&self) -> Option<bool>;
    fn read(&self) -> Result<bool, Self::Error>;
    fn read_prev(&self) -> Result<bool, Self::Error>;
    fn signal_in(&mut self, state: PinState) -> Result<(), Self::Error>;
    fn drive_in(&mut self, state: bool) -> Result<(), Self::Error>;
    fn tri_state_in(&mut self);
    fn undefine_in(&mut self) -> Result<(), Self::Error>;
}

pub trait SinglePinNew {
    fn new(name: String) -> Self;
}

pub trait SinglePinOutput {
    type Error: From<PinError>;
    fn signal_out(&mut self, state: PinState) -> Result<(), Self::Error>;
    fn drive_out(&mut self, state: bool) -> Result<(), Self::Error>;
    fn tri_state_out(&mut self);
    fn undefine_out(&mut self) -> Result<(), Self::Error>;
}
