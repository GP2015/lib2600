pub mod contention;
pub mod input;

#[cfg(test)]
mod mock_pin;

use crate::pin::PinState;

pub trait Bus {
    type Error;
    fn read(&self) -> Result<usize, Self::Error>;
    fn read_bit(&self, bit: usize) -> Result<bool, Self::Error>;
    fn state(&self) -> Vec<Option<PinState>>;
    fn bit_state(&self, bit: usize) -> Result<Option<PinState>, Self::Error>;
    fn drive_value_in(&mut self, val: usize) -> Result<(), Self::Error>;
    fn drive_value_in_wrapped(&mut self, val: usize) -> Result<(), Self::Error>;
    fn tri_state_in(&mut self);
    fn set_signal_in_bit(&mut self, bit: usize, state: PinState) -> Result<(), Self::Error>;
    fn drive_in_bit(&mut self, bit: usize, state: bool) -> Result<(), Self::Error>;
    fn tri_state_in_bit(&mut self, bit: usize) -> Result<(), Self::Error>;
}

pub trait BusOutput {
    type Error;
    fn drive_value_out(&mut self, val: usize) -> Result<(), Self::Error>;
    fn tri_state_out(&mut self);
    fn set_signal_out_bit(&mut self, bit: usize, state: PinState) -> Result<(), Self::Error>;
    fn drive_out_bit(&mut self, bit: usize, state: bool) -> Result<(), Self::Error>;
    fn tri_state_out_bit(&mut self, bit: usize) -> Result<(), Self::Error>;
}
