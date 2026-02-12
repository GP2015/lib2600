pub mod contention;
pub mod core;
pub mod input;

#[cfg(test)]
pub mod mock_pin;

use crate::pin::PinState;

pub trait SinglePinInterface<E> {
    fn state(&self) -> PinState;
    fn prev_state(&self) -> PinState;
    fn state_as_bool(&self) -> Option<bool>;
    fn prev_state_as_bool(&self) -> Option<bool>;
    fn read(&self) -> Result<bool, E>;
    fn read_prev(&self) -> Result<bool, E>;
    fn signal_in(&mut self, state: PinState) -> Result<(), E>;
    fn drive_in(&mut self, state: bool) -> Result<(), E>;
    fn tri_state_in(&mut self);
    fn undefine_in(&mut self) -> Result<(), E>;
}

pub trait SinglePinCore {
    fn new(name: String) -> Self;
    fn post_tick_update(&mut self);
}

pub trait SinglePinOutput<E> {
    fn signal_out(&mut self, state: PinState) -> Result<(), E>;
    fn drive_out(&mut self, state: bool) -> Result<(), E>;
    fn tri_state_out(&mut self);
    fn undefine_out(&mut self) -> Result<(), E>;
}
