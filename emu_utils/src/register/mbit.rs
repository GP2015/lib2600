use itertools::Itertools;

use crate::{
    bit,
    pin::{BusCore, SinglePinCore},
    register::{BitRegister, RegisterError},
};

#[derive(Clone)]
pub struct MBitRegister {
    name: String,
    bits: Box<[BitRegister]>,
}

impl MBitRegister {
    fn check_bit_in_range(&self, bit: usize) -> Result<(), RegisterError> {
        if bit >= self.size() {
            Err(RegisterError::BitOutOfRange {
                name: self.name.clone(),
                bit,
                size: self.size(),
            })
        } else {
            Ok(())
        }
    }

    fn check_write_val_valid(&self, val: usize) -> Result<(), RegisterError> {
        if bit::usize_exceeds_bit_count(val, self.size()) {
            Err(RegisterError::WriteValueTooLarge {
                name: self.name.clone(),
                value: val,
                size: self.size(),
            })
        } else {
            Ok(())
        }
    }

    fn collapsed_as_usize(collapsed: &[Option<bool>]) -> Option<usize> {
        let mut combined = 0;
        for &bit in collapsed.iter().rev() {
            let b = bit?;
            combined = (combined << 1) | usize::from(b);
        }
        Some(combined)
    }

    fn bools_as_usize(bools: &[bool]) -> usize {
        let mut combined = 0;
        for b in bools.iter().rev().map(|&b| usize::from(b)) {
            combined = (combined << 1) | b;
        }
        combined
    }

    #[must_use]
    pub fn new(size: usize, name: String) -> Self {
        Self {
            bits: (0..size)
                .map(|bit| BitRegister::new(format!("{name} bit {bit}")))
                .collect(),
            name,
        }
    }

    #[must_use]
    pub fn size(&self) -> usize {
        self.bits.len()
    }

    pub fn bit(&self, bit: usize) -> Result<&BitRegister, RegisterError> {
        self.check_bit_in_range(bit)?;
        Ok(&self.bits[bit])
    }

    pub fn bit_mut(&mut self, bit: usize) -> Result<&mut BitRegister, RegisterError> {
        self.check_bit_in_range(bit)?;
        Ok(&mut self.bits[bit])
    }

    pub fn iter(&self) -> impl Iterator<Item = &BitRegister> {
        self.bits.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut BitRegister> {
        self.bits.iter_mut()
    }

    pub fn read(&self) -> Option<usize> {
        let collapsed = self
            .bits
            .iter()
            .map(BitRegister::collapsed)
            .collect::<Vec<Option<bool>>>();
        Self::collapsed_as_usize(&collapsed)
    }

    pub fn iter_possible_reads(&self) -> impl Iterator<Item = usize> {
        self.bits
            .iter()
            .map(BitRegister::possible_reads)
            .multi_cartesian_product()
            .map(|bools| Self::bools_as_usize(&bools))
    }

    pub fn add_wrapping(&mut self, val: usize, only_possible: bool) {
        for (bit, bitreg) in self.bits.iter_mut().enumerate() {
            bitreg.add(bit::get_bit_of_usize(val, bit), only_possible);
        }
    }

    pub fn add(&mut self, val: usize, only_possible: bool) -> Result<(), RegisterError> {
        self.check_write_val_valid(val)?;
        self.add_wrapping(val, only_possible);
        Ok(())
    }

    pub fn input_from_bus<'a, B, P>(&mut self, bus: &'a B, only_possible: bool)
    where
        B: BusCore<'a, P>,
        P: 'a + SinglePinCore<'a>,
    {
        for (reg, pin) in self.iter_mut().zip(bus.iter()) {
            reg.input_from_pin(pin, only_possible);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::fixture;

    #[fixture]
    fn reg() -> MBitRegister {
        MBitRegister::new(8, String::new())
    }
}
