pub mod address;
pub mod data;

use crate::{PinState, RiotError};

pub trait Bus {
    fn read(&self) -> Result<usize, RiotError>;
    fn read_bit(&self, bit: usize) -> Result<bool, RiotError>;
    fn state(&self) -> Vec<Option<PinState>>;
    fn bit_state(&self, bit: usize) -> Result<Option<PinState>, RiotError>;
    fn drive_value_in(&mut self, val: usize) -> Result<(), RiotError>;
    fn drive_value_in_wrapped(&mut self, val: usize) -> Result<(), RiotError>;
    fn tristate_in(&mut self);
    fn set_signal_in_bit(&mut self, bit: usize, state: PinState) -> Result<(), RiotError>;
    fn drive_in_bit(&mut self, bit: usize, state: bool) -> Result<(), RiotError>;
    fn tristate_in_bit(&mut self, bit: usize) -> Result<(), RiotError>;
}

pub trait BusOutput {
    fn drive_value_out(&mut self, val: usize) -> Result<(), RiotError>;
    fn tristate_out(&mut self);
    fn set_signal_out_bit(&mut self, bit: usize, state: PinState) -> Result<(), RiotError>;
    fn drive_out_bit(&mut self, bit: usize, state: bool) -> Result<(), RiotError>;
    fn tristate_out_bit(&mut self, bit: usize) -> Result<(), RiotError>;
}
