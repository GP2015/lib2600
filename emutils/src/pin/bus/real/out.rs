use crate::{
    pin::{BusInputter, PinInputter, PinOutputter},
    reg::MBitRegister,
};

pub trait BusOutputter<'a, P>: BusInputter<'a, P>
where
    P: 'a + PinInputter<'a> + PinOutputter<'a>,
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
        B: BusInputter<'a, P2>,
        P2: 'a + PinInputter<'a>,
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
