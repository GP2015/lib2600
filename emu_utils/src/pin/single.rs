pub mod contention;
pub mod core;
pub mod input;

#[cfg(test)]
pub mod mock_pin;

use crate::pin::{PinSignal, state::PinState};

pub trait SinglePinInterface<E> {
    fn state(&self) -> PinState;
    fn prev_state(&self) -> PinState;
    fn add_signal_in(&mut self, state: PinSignal) -> Result<(), E>;
    fn add_drive_in(&mut self, state: bool) -> Result<(), E>;
    fn add_tri_state_in(&mut self);
}

pub trait SinglePinCore {
    fn new(name: String) -> Self;
    fn post_tick_update(&mut self);
}

pub trait SinglePinOutput<E> {
    fn add_signal_out(&mut self, state: PinSignal) -> Result<(), E>;
    fn add_drive_out(&mut self, state: bool) -> Result<(), E>;
    fn add_tri_state_out(&mut self);
}
