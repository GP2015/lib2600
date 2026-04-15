use crate::pin::{BusCore, SinglePinCore};

pub trait BusOutput<'a, P>: BusCore<'a, P>
where
    P: 'a + SinglePinCore<'a>,
{
    fn pin_out(&mut self, bit: usize) -> Result<&mut P, P::ErrType>;
    fn add_drive_out(&mut self, val: usize) -> Result<(), P::ErrType>;
    fn add_drive_out_wrapping(&mut self, val: usize) -> Result<(), P::ErrType>;
}
