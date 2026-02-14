pub mod standard;

use crate::pin::{SinglePinInterface, SinglePinOutput};

pub trait BusInterface<E> {
    fn name(&self) -> String;
    fn pin(&self, bit: usize) -> Result<&impl SinglePinInterface<E>, E>;
    fn pin_mut(&mut self, bit: usize) -> Result<&mut impl SinglePinInterface<E>, E>;
    fn read_collapsed(&self) -> Option<usize>;
    fn read_prev_collapsed(&self) -> Option<usize>;
    fn add_possible_drive_in(&mut self, val: usize) -> Result<(), E>;
    fn add_possible_drive_in_wrapping(&mut self, val: usize) -> Result<(), E>;
    fn add_possible_tri_state_in(&mut self);
    fn remove_all_possible_in(&mut self);
}

pub trait BusCore {
    fn new(name: String, size: usize) -> Self;
    fn post_tick_update(&mut self);
}

pub trait BusOutput<E> {
    fn pin_out(&mut self, bit: usize) -> Result<&mut impl SinglePinOutput<E>, E>;
    fn add_possible_drive_out(&mut self, val: usize) -> Result<(), E>;
    fn add_possible_tri_state_out(&mut self);
    fn remove_all_possible_out(&mut self);
}
