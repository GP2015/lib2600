pub mod standard;

use crate::pin::{PinError, PinState, SinglePinCore, SinglePinOutput};

pub trait BusCore {
    fn new(name: String, size: usize) -> Self;
    fn pin(&self, bit: usize) -> Result<&impl SinglePinCore, PinError>;
    fn pin_mut(&mut self, bit: usize) -> Result<&mut impl SinglePinCore, PinError>;
    fn state(&self) -> Vec<PinState>;
    fn prev_state(&self) -> Vec<PinState>;
    fn state_as_bool(&self) -> Vec<Option<bool>>;
    fn prev_state_as_bool(&self) -> Vec<Option<bool>>;
    fn read(&self) -> Result<usize, PinError>;
    fn read_prev(&self) -> Result<usize, PinError>;
    fn drive_in(&mut self, val: usize) -> Result<(), PinError>;
    fn wrapping_drive_in(&mut self, val: usize) -> Result<(), PinError>;
    fn tri_state_in(&mut self);
    fn undefine_in(&mut self) -> Result<(), PinError>;
}

pub trait BusOutput {
    fn pin_out(&self, bit: usize) -> Result<&impl SinglePinOutput, PinError>;
    fn pin_out_mut(&mut self, bit: usize) -> Result<&mut impl SinglePinOutput, PinError>;
    fn drive_out(&mut self, val: usize) -> Result<(), PinError>;
    fn tri_state_out(&mut self);
    fn undefine_out(&mut self) -> Result<(), PinError>;
}
