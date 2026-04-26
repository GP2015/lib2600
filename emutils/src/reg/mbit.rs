use crate::{
    bit,
    pin::BusInputUI,
    reg::{BitRegister, RegisterError},
};
use delegate::delegate;
use itertools::Itertools;

#[derive(Clone)]
pub struct MBitRegister {
    name: String,
    bits: Box<[BitRegister]>,
}

impl MBitRegister {
    fn check_bit_in_range(&self, bit: usize) -> Result<(), RegisterError> {
        if bit >= self.size() {
            return Err(RegisterError::BitOutOfRange {
                name: self.name.clone(),
                bit,
                size: self.size(),
            });
        }
        Ok(())
    }

    fn check_write_val_valid(&self, val: usize) -> Result<(), RegisterError> {
        if bit::usize_exceeds_bit_count(val, self.size()) {
            return Err(RegisterError::WriteValueTooLarge {
                name: self.name.clone(),
                value: val,
                size: self.size(),
            });
        }
        Ok(())
    }

    #[must_use]
    pub fn new<S: Into<String>>(name: S, size: usize) -> Self {
        let name = name.into();
        Self {
            bits: (0..size)
                .map(|bit| BitRegister::new(format!("{name} bit {bit}")))
                .collect(),
            name,
        }
    }

    pub fn bit(&self, bit: usize) -> Result<&BitRegister, RegisterError> {
        self.check_bit_in_range(bit)?;
        Ok(&self.bits[bit])
    }

    pub fn bit_mut(&mut self, bit: usize) -> Result<&mut BitRegister, RegisterError> {
        self.check_bit_in_range(bit)?;
        Ok(&mut self.bits[bit])
    }

    #[must_use]
    pub fn read(&self) -> Option<usize> {
        bit::some_bits_to_usize(self.bits.iter().map(BitRegister::collapsed))
    }

    pub fn iter_possible_reads(&self) -> impl Iterator<Item = usize> {
        self.bits
            .iter()
            .map(BitRegister::possible_reads)
            .multi_cartesian_product()
            .map(|bits| bit::bits_to_usize(bits.into_iter().copied()))
    }

    pub fn add_wrapping(&mut self, val: usize, only_possible: bool) {
        for (bit, reg) in self.bits.iter_mut().enumerate() {
            reg.add(bit::bit_of_usize(val, bit), only_possible);
        }
    }

    pub fn add(&mut self, val: usize, only_possible: bool) -> Result<(), RegisterError> {
        self.check_write_val_valid(val)?;
        self.add_wrapping(val, only_possible);
        Ok(())
    }

    pub fn input_from_bus(&mut self, bus: &impl BusInputUI, only_possible: bool) {
        for (reg, pin) in self.iter_mut().zip(bus.iter()) {
            reg.input_from_pin(pin, only_possible);
        }
    }

    delegate! {
        to self.bits {
            #[must_use]
            #[call(len)]
            pub fn size(&self) -> usize;
            pub fn iter(&self) -> impl Iterator<Item = &BitRegister>;
            pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut BitRegister>;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::fixture;

    const REG_NAME: &str = "reg";

    #[fixture]
    fn reg() -> MBitRegister {
        MBitRegister::new(REG_NAME, 8)
    }
}
