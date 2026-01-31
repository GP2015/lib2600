pub mod contention;
pub mod input;

#[cfg(test)]
mod mock_pin;

use crate::pin::{PinError, PinState};

pub trait Bus {
    fn read(&self) -> Result<usize, PinError>;
    fn read_bit(&self, bit: usize) -> Result<bool, PinError>;
    fn state(&self) -> Vec<Option<PinState>>;
    fn bit_state(&self, bit: usize) -> Result<Option<PinState>, PinError>;
    fn drive_value_in(&mut self, val: usize) -> Result<(), PinError>;
    fn drive_value_in_wrapped(&mut self, val: usize) -> Result<(), PinError>;
    fn tri_state_in(&mut self);
    fn set_signal_in_bit(&mut self, bit: usize, state: PinState) -> Result<(), PinError>;
    fn drive_in_bit(&mut self, bit: usize, state: bool) -> Result<(), PinError>;
    fn tri_state_in_bit(&mut self, bit: usize) -> Result<(), PinError>;
}

pub trait BusOutput {
    fn drive_value_out(&mut self, val: usize) -> Result<(), PinError>;
    fn tri_state_out(&mut self);
    fn set_signal_out_bit(&mut self, bit: usize, state: PinState) -> Result<(), PinError>;
    fn drive_out_bit(&mut self, bit: usize, state: bool) -> Result<(), PinError>;
    fn tri_state_out_bit(&mut self, bit: usize) -> Result<(), PinError>;
}
