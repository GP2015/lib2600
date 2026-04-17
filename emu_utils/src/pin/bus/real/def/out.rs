use crate::{
    pin::{BusCore, SinglePinCore, SinglePinOutput},
    reg::MBitRegister,
};

pub trait BusOutput<'a, P>: BusCore<'a, P>
where
    P: 'a + SinglePinCore<'a> + SinglePinOutput<'a>,
{
    fn add_drive_out(&mut self, val: usize, only_possible: bool) -> Result<(), P::ErrType>;
    fn add_drive_out_wrapping(&mut self, val: usize, only_possible: bool)
    -> Result<(), P::ErrType>;

    fn output_from_bus<B, P2>(
        &'a mut self,
        bus: &'a B,
        only_possible: bool,
    ) -> Result<(), P::ErrType>
    where
        B: BusCore<'a, P2>,
        P2: 'a + SinglePinCore<'a>,
    {
        for (out_pin, in_pin) in self.iter_mut().zip(bus.iter()) {
            out_pin.output_from_pin(in_pin, only_possible)?;
        }
        Ok(())
    }

    fn output_from_reg(
        &'a mut self,
        reg: &MBitRegister,
        only_possible: bool,
    ) -> Result<(), P::ErrType> {
        for (pin, reg) in self.iter_mut().zip(reg.iter()) {
            pin.output_from_reg(reg, only_possible)?;
        }
        Ok(())
    }
}
