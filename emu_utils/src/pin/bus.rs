pub mod standard;

use crate::pin::{PinError, SinglePinInterface, SinglePinOutput};

pub trait BusInterface<E: From<PinError>> {
    fn name(&self) -> &str;
    fn pin(&self, bit: usize) -> Result<&impl SinglePinInterface<E>, E>;
    fn pin_mut(&mut self, bit: usize) -> Result<&mut impl SinglePinInterface<E>, E>;
    fn read(&self) -> Option<usize>;
    fn read_prev(&self) -> Option<usize>;
    fn add_possible_drive_in(&mut self, val: usize) -> Result<(), E>;
    fn add_possible_drive_in_wrapping(&mut self, val: usize) -> Result<(), E>;
}

pub trait BusCore {
    fn new(name: String, size: usize) -> Self;
    fn post_tick_update(&mut self);
}

pub trait BusOutput<E: From<PinError>> {
    fn pin_out(&mut self, bit: usize) -> Result<&mut impl SinglePinOutput<E>, E>;
    fn add_possible_drive_out(&mut self, val: usize) -> Result<(), E>;
    fn add_possible_tri_state_out(&mut self);
    fn remove_all_possible_out(&mut self);
    fn set_all_possible_out_to_prev(&mut self) -> Result<(), E>;
}
