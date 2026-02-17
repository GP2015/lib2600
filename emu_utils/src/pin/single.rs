pub mod contention;
pub mod input;
#[cfg(test)]
pub mod mock_pin;

use crate::pin::{PinError, PinSignal};

pub trait SinglePinInterface<E: From<PinError>> {
    fn name(&self) -> &str;
    fn possible_signals(&self) -> Vec<PinSignal>;
    fn prev_possible_signals(&self) -> Vec<PinSignal>;
    fn collapsed(&self) -> Option<PinSignal>;
    fn prev_collapsed(&self) -> Option<PinSignal>;
    fn set_signal_in(&mut self, signal: PinSignal, possible: bool) -> Result<(), E>;
    fn set_drive_in(&mut self, bool_signal: bool, possible: bool) -> Result<(), E>;
    fn set_tri_state_in(&mut self, possible: bool);
    fn set_all_signals_in(&mut self, possible: bool) -> Result<(), E>;
    fn set_possible_in_to_prev(&mut self) -> Result<(), E>;
}

pub trait SinglePinCore {
    fn new(name: String) -> Self;
    fn post_tick_update(&mut self);
}

pub trait SinglePinOutput<E: From<PinError>> {
    fn set_signal_out(&mut self, signal: PinSignal, possible: bool) -> Result<(), E>;
    fn set_drive_out(&mut self, bool_signal: bool, possible: bool) -> Result<(), E>;
    fn set_tri_state_out(&mut self, possible: bool);
    fn set_all_signals_out(&mut self, possible: bool) -> Result<(), E>;
    fn set_possible_out_to_prev(&mut self) -> Result<(), E>;
}
