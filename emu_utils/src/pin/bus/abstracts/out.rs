use crate::pin::PinError;

pub trait BusOutput<P> {
    fn pin_out(&mut self, bit: usize) -> Result<&mut P, PinError>;
    fn add_possible_drive_out(&mut self, val: usize) -> Result<(), PinError>;
}
