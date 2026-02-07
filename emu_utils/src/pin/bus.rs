pub mod standard;

use crate::pin::{PinError, PinState, SinglePin, SinglePinOutput, SinglePinSetup};

pub trait Bus<E: From<PinError>> {
    fn pin(&self, bit: usize) -> Result<&impl SinglePin<E>, E>;
    fn pin_mut(&mut self, bit: usize) -> Result<&mut impl SinglePin<E>, E>;
    fn state(&self) -> Vec<PinState>;
    fn state_as_bool(&self) -> Vec<Option<bool>>;
    fn read(&self) -> Result<usize, E>;
    fn drive_in(&mut self, val: usize) -> Result<(), E>;
    fn wrapping_drive_in(&mut self, val: usize) -> Result<(), E>;
    fn tri_state_in(&mut self) -> Result<(), E>;
    fn undefine_in(&mut self) -> Result<(), E>;
}

pub trait BusSetup<E: From<PinError>> {
    fn new(name: String, size: usize) -> Self;
    fn pin_setup(&self, bit: usize) -> Result<&impl SinglePinSetup<E>, E>;
    fn pin_setup_mut(&mut self, bit: usize) -> Result<&mut impl SinglePinSetup<E>, E>;
}

pub trait BusOutput<E: From<PinError>> {
    fn pin_out(&self, bit: usize) -> Result<&impl SinglePinOutput<E>, E>;
    fn pin_out_mut(&mut self, bit: usize) -> Result<&mut impl SinglePinOutput<E>, E>;
    fn drive_out(&mut self, val: usize) -> Result<(), E>;
    fn tri_state_out(&mut self) -> Result<(), E>;
    fn undefine_out(&mut self) -> Result<(), E>;
}
