pub mod contention;
pub mod core;
pub mod input;

#[cfg(test)]
pub mod mock_pin;

use crate::pin::{PinError, PinState};

pub trait SinglePinCore {
    fn new(name: String) -> Self;
    fn state(&self) -> PinState;
    fn prev_state(&self) -> PinState;
    fn state_as_bool(&self) -> Option<bool>;
    fn prev_state_as_bool(&self) -> Option<bool>;
    fn read(&self) -> Result<bool, PinError>;
    fn read_prev(&self) -> Result<bool, PinError>;
    fn signal_in(&mut self, state: PinState) -> Result<(), PinError>;
    fn drive_in(&mut self, state: bool) -> Result<(), PinError>;
    fn tri_state_in(&mut self);
    fn undefine_in(&mut self) -> Result<(), PinError>;
}

pub trait SinglePinOutput {
    fn signal_out(&mut self, state: PinState) -> Result<(), PinError>;
    fn drive_out(&mut self, state: bool) -> Result<(), PinError>;
    fn tri_state_out(&mut self);
    fn undefine_out(&mut self) -> Result<(), PinError>;
}
