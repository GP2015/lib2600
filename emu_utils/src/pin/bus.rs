pub mod standard;

use crate::pin::{PinState, SinglePin, SinglePinOutput};

pub trait Bus<E> {
    fn pin(&self, bit: usize) -> Result<&impl SinglePin<E>, E>;
    fn pin_mut(&mut self, bit: usize) -> Result<&mut impl SinglePin<E>, E>;
    fn state(&self) -> Vec<PinState>;
    fn prev_state(&self) -> Vec<PinState>;
    fn state_as_bool(&self) -> Vec<Option<bool>>;
    fn prev_state_as_bool(&self) -> Vec<Option<bool>>;
    fn read(&self) -> Result<usize, E>;
    fn read_prev(&self) -> Result<usize, E>;
    fn drive_in(&mut self, val: usize) -> Result<(), E>;
    fn wrapping_drive_in(&mut self, val: usize) -> Result<(), E>;
    fn tri_state_in(&mut self);
    fn undefine_in(&mut self) -> Result<(), E>;
}

pub trait BusOutput<E> {
    fn pin_out(&self, bit: usize) -> Result<&impl SinglePinOutput<E>, E>;
    fn pin_out_mut(&mut self, bit: usize) -> Result<&mut impl SinglePinOutput<E>, E>;
    fn drive_out(&mut self, val: usize) -> Result<(), E>;
    fn tri_state_out(&mut self);
    fn undefine_out(&mut self) -> Result<(), E>;
}
