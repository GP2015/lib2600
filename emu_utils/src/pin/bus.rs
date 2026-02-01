pub mod contention;
pub mod input;

#[cfg(test)]
mod mock_pin;

use crate::pin::PinState;

pub trait Bus {
    type Error;
    fn state(&self) -> Vec<PinState>;
    fn bit_state(&self, bit: usize) -> Result<PinState, Self::Error>;
    fn read(&self) -> Result<usize, Self::Error>;
    fn read_bit(&self, bit: usize) -> Result<bool, Self::Error>;
    fn drive_value_in(&mut self, val: usize) -> Result<(), Self::Error>;
    fn drive_value_in_wrapped(&mut self, val: usize) -> Result<(), Self::Error>;
    fn tri_state_in(&mut self);
    fn undefine_in(&mut self) -> Result<(), Self::Error>;
    fn signal_in_bit(&mut self, bit: usize, state: PinState) -> Result<(), Self::Error>;
    fn drive_in_bit(&mut self, bit: usize, state: bool) -> Result<(), Self::Error>;
    fn tri_state_in_bit(&mut self, bit: usize) -> Result<(), Self::Error>;
    fn undefine_in_bit(&mut self, bit: usize) -> Result<(), Self::Error>;
}

pub trait BusOutput {
    type Error;
    fn drive_value_out(&mut self, val: usize) -> Result<(), Self::Error>;
    fn tri_state_out(&mut self);
    fn undefine_out(&mut self) -> Result<(), Self::Error>;
    fn signal_out_bit(&mut self, bit: usize, state: PinState) -> Result<(), Self::Error>;
    fn drive_out_bit(&mut self, bit: usize, state: bool) -> Result<(), Self::Error>;
    fn tri_state_out_bit(&mut self, bit: usize) -> Result<(), Self::Error>;
    fn undefine_out_bit(&mut self, bit: usize) -> Result<(), Self::Error>;
}
