pub mod standard;

use crate::pin::{PinState, SinglePin, SinglePinOutput};

pub trait Bus {
    type Error;
    fn pin(&self, bit: usize) -> Result<&impl SinglePin, Self::Error>;
    fn pin_mut(&mut self, bit: usize) -> Result<&mut impl SinglePin, Self::Error>;
    fn state(&self) -> Vec<PinState>;
    fn prev_state(&self) -> Vec<PinState>;
    fn state_as_bool(&self) -> Vec<Option<bool>>;
    fn prev_state_as_bool(&self) -> Vec<Option<bool>>;
    fn read(&self) -> Result<usize, Self::Error>;
    fn read_prev(&self) -> Result<usize, Self::Error>;
    fn drive_in(&mut self, val: usize) -> Result<(), Self::Error>;
    fn wrapping_drive_in(&mut self, val: usize) -> Result<(), Self::Error>;
    fn tri_state_in(&mut self);
    fn undefine_in(&mut self) -> Result<(), Self::Error>;
}

pub trait BusOutput {
    type Error;
    fn pin_out(&self, bit: usize) -> Result<&impl SinglePinOutput, Self::Error>;
    fn pin_out_mut(&mut self, bit: usize) -> Result<&mut impl SinglePinOutput, Self::Error>;
    fn drive_out(&mut self, val: usize) -> Result<(), Self::Error>;
    fn tri_state_out(&mut self);
    fn undefine_out(&mut self) -> Result<(), Self::Error>;
}
