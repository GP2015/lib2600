use crate::{
    pin::{BusCore, SinglePinCore, SinglePinOutput},
    register::MBitRegister,
};

pub trait BusOutput<'a, P>: BusCore<'a, P>
where
    P: 'a + SinglePinCore<'a> + SinglePinOutput<'a>,
{
    fn add_drive_out(&mut self, val: usize) -> Result<(), P::ErrType>;
    fn add_drive_out_wrapping(&mut self, val: usize) -> Result<(), P::ErrType>;

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
