pub mod standard;

use crate::pin::{PinState, SinglePin};

pub trait Bus {
    type Error;
    fn pin(&self, bit: usize) -> &impl SinglePin;
    fn pin_mut(&self, bit: usize) -> &mut impl SinglePin;
    fn state(&self) -> Vec<PinState>;
    fn prev_state(&self) -> Vec<PinState>;
    fn state_as_bool(&self) -> Vec<Option<bool>>;
    fn prev_state_as_bool(&self) -> Vec<Option<bool>>;
    fn read(&self) -> Result<usize, Self::Error>;
    fn prev(&self) -> Option<usize>;
    fn drive_in(&mut self, val: usize) -> Result<(), Self::Error>;
    fn wrapping_drive_in(&mut self, val: usize) -> Result<(), Self::Error>;
    fn tri_state_in(&mut self);
    fn undefine_in(&mut self) -> Result<(), Self::Error>;
}

pub trait BusOutput {
    type Error;
    fn drive_out(&mut self, val: usize) -> Result<(), Self::Error>;
    fn tri_state_out(&mut self);
    fn undefine_out(&mut self) -> Result<(), Self::Error>;
}
