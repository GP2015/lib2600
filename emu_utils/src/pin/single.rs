pub mod contention;
pub mod input;

use crate::pin::{PinError, PinState};

pub trait SinglePin {
    type Error: From<PinError>;
    fn state(&self) -> PinState;
    fn read(&self) -> Result<bool, Self::Error>;
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
