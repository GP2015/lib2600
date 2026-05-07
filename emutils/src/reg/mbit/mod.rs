pub mod state;

use crate::{
    bit,
    line::BusState,
    reg::{BitRegister, MBitRegisterState, RegisterError},
};
use delegate::delegate;
use std::array;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct MBitRegister<const SIZE: usize> {
    name: String,
    bits: [BitRegister; SIZE],
}

impl<const SIZE: usize> MBitRegister<SIZE> {
    #[must_use]
    pub fn new<S: Into<String>>(name: S, low: bool, high: bool) -> Self {
        let name = name.into();
        Self {
            bits: array::from_fn(|bit| BitRegister::new(format!("{name} bit {bit}"), low, high)),
            name,
        }
    }

    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[must_use]
    pub const fn bit<const BIT: usize>(&self) -> &BitRegister {
        const { assert!(BIT < SIZE) }

        #[allow(clippy::indexing_slicing)]
        &self.bits[BIT]
    }

    #[must_use]
    pub const fn bit_mut<const BIT: usize>(&mut self) -> &mut BitRegister {
        const { assert!(BIT < SIZE) }

        #[allow(clippy::indexing_slicing)]
        &mut self.bits[BIT]
    }

    pub fn try_bit(&self, bit: usize) -> Result<&BitRegister, RegisterError> {
        self.bits
            .get(bit)
            .ok_or_else(|| RegisterError::BitOutOfRange {
                name: self.name.clone(),
                bit,
                size: SIZE,
            })
    }

    pub fn try_bit_mut(&mut self, bit: usize) -> Result<&mut BitRegister, RegisterError> {
        self.bits
            .get_mut(bit)
            .ok_or_else(|| RegisterError::BitOutOfRange {
                name: self.name.clone(),
                bit,
                size: SIZE,
            })
    }

    #[must_use]
    pub fn state(&self) -> MBitRegisterState<SIZE> {
        #[allow(clippy::indexing_slicing)]
        MBitRegisterState::new(array::from_fn(|bit| self.bits[bit].state()))
    }

    pub fn add_wrapping(&mut self, val: usize) {
        for (bit, reg) in self.bits.iter_mut().enumerate() {
            reg.add(bit::bit_of_usize(val, bit));
        }
    }

    pub fn add(&mut self, val: usize) -> Result<(), RegisterError> {
        if bit::usize_exceeds_bit_count(val, SIZE) {
            return Err(RegisterError::WriteValueTooLarge {
                name: self.name.clone(),
                value: val,
                size: SIZE,
            });
        }

        self.add_wrapping(val);
        Ok(())
    }

    pub fn remove_all(&mut self) {
        for reg in self.iter_mut() {
            reg.remove_all();
        }
    }

    pub fn copy_from_bus_state(&mut self, bus: &BusState<SIZE>) {
        for (reg, line) in self.iter_mut().zip(bus.iter()) {
            reg.copy_from_line_state(&line);
        }
    }

    pub fn copy_from_reg_state(&mut self, other: &MBitRegisterState<SIZE>) {
        for (reg, other_reg) in self.iter_mut().zip(other.iter()) {
            reg.copy_from_reg_state(&other_reg);
        }
    }

    delegate! {
        to self.bits {
            pub fn iter(&self) -> impl Iterator<Item = &BitRegister>;
            pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut BitRegister>;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::{fixture, rstest};

    const REG_NAME: &str = "reg";
    const REG_SIZE: usize = 4;

    #[fixture]
    fn reg() -> MBitRegister<REG_SIZE> {
        MBitRegister::new(REG_NAME, true, true)
    }

    #[rstest]
    fn new_correct_size(reg: MBitRegister<REG_SIZE>) {
        assert_eq!(reg.state().size(), REG_SIZE);
    }

    #[rstest]
    fn new_correct_names(reg: MBitRegister<REG_SIZE>) {
        for (bit, bit_reg) in reg.iter().enumerate() {
            assert_eq!(bit_reg.name(), format!("{REG_NAME} bit {bit}"));
        }
    }

    #[rstest]
    fn name(reg: MBitRegister<REG_SIZE>) {
        assert_eq!(reg.name(), REG_NAME);
    }

    #[rstest]
    #[case(0, [false, false, false, false])]
    #[case(0b101, [true, false, true, false])]
    #[case(0b1011, [true, true, false, true])]
    #[case(0b11011, [true, true, false, true])]
    fn add_wrapping_only_possible(
        mut reg: MBitRegister<REG_SIZE>,
        #[values(true, false)] initial: bool,
        #[case] val: usize,
        #[case] bits: [bool; REG_SIZE],
    ) {
        reg.iter_mut().for_each(|r| r.add(initial));
        reg.add_wrapping(val);
        for (bit_reg, &b) in reg.iter().zip(bits.iter()) {
            assert_eq!(bit_reg.state().collapsed(), Some(b));
        }
    }

    #[rstest]
    #[case(0, [false, false, false, false])]
    #[case(0b101, [true, false, true, false])]
    #[case(0b1011, [true, true, false, true])]
    #[case(0b11011, [true, true, false, true])]
    fn add_wrapping_not_only_possible(
        mut reg: MBitRegister<REG_SIZE>,
        #[values(true, false)] initial: bool,
        #[case] val: usize,
        #[case] bits: [bool; REG_SIZE],
    ) {
        reg.iter_mut().for_each(|r| r.add(initial));
        reg.add_wrapping(val);
        for (bit_reg, &b) in reg.iter().zip(bits.iter()) {
            assert!(bit_reg.state().is_possible(initial));
            assert!(bit_reg.state().is_possible(b));
        }
    }

    #[rstest]
    fn add_success(mut reg: MBitRegister<REG_SIZE>, #[values(0, 0b100, 0b1011)] val: usize) {
        assert!(reg.add(val).is_ok());
    }

    #[rstest]
    fn add_failure(mut reg: MBitRegister<REG_SIZE>, #[values(0b10000, 0b11011)] val: usize) {
        let e = RegisterError::WriteValueTooLarge {
            name: reg.name().to_string(),
            value: val,
            size: reg.state().size(),
        };
        assert_eq!(reg.add(val).err().unwrap(), e);
    }
}
