use crate::{
    pin::{PinError, PinInputUI, PinOutput},
    reg::MBitRegister,
};

pub trait BusCore {
    fn new(name: String, size: usize) -> Self;
    fn post_tick_update(&mut self);
}

pub trait BusInputUI {
    fn name(&self) -> &str;
    fn size(&self) -> usize;

    fn pin(&self, bit: usize) -> Result<&impl PinInputUI, PinError>;
    fn pin_mut(&mut self, bit: usize) -> Result<&mut impl PinInputUI, PinError>;
    fn iter(&self) -> impl Iterator<Item = &impl PinInputUI>;
    fn iter_mut(&mut self) -> impl Iterator<Item = &mut impl PinInputUI>;
    fn read(&self) -> Option<usize>;
    fn read_prev(&self) -> Option<usize>;
    fn only_one_possible_read(&self) -> bool;
    fn iter_possible_reads(&self) -> impl Iterator<Item = usize>;
    fn iter_prev_possible_reads(&self) -> impl Iterator<Item = usize>;
    fn add_drive_in(&mut self, val: usize, only_possible: bool) -> Result<(), PinError>;
    fn add_drive_in_wrapping(&mut self, val: usize, only_possible: bool) -> Result<(), PinError>;
}

pub trait BusOutput {
    fn pin_out(&self, bit: usize) -> Result<&impl PinOutput, PinError>;
    fn pin_out_mut(&mut self, bit: usize) -> Result<&mut impl PinOutput, PinError>;
    fn iter_out(&self) -> impl Iterator<Item = &impl PinOutput>;
    fn iter_out_mut(&mut self) -> impl Iterator<Item = &mut impl PinOutput>;
    fn add_drive_out(&mut self, val: usize, only_possible: bool) -> Result<(), PinError>;
    fn add_drive_out_wrapping(&mut self, val: usize, only_possible: bool) -> Result<(), PinError>;

    fn output_from_bus(
        &mut self,
        bus: &impl BusInputUI,
        only_possible: bool,
    ) -> Result<(), PinError> {
        for (out_pin, in_pin) in self.iter_out_mut().zip(bus.iter()) {
            out_pin.output_from_pin(in_pin, only_possible)?;
        }
        Ok(())
    }

    fn output_from_reg(&mut self, reg: &MBitRegister, only_possible: bool) -> Result<(), PinError> {
        for (pin, reg) in self.iter_out_mut().zip(reg.iter()) {
            pin.output_from_reg(reg, only_possible)?;
        }
        Ok(())
    }
}
