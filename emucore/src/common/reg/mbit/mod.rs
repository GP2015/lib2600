pub mod state;

use crate::common::{
    bit,
    line::bus::state::BusState,
    reg::{bit::BitReg, mbit::state::MBitRegState},
};
use delegate::delegate;
use std::array;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct MBitReg<const SIZE: usize> {
    name: String,
    bits: [BitReg; SIZE],
}

impl<const SIZE: usize> MBitReg<SIZE> {
    #[must_use]
    pub fn new<S: Into<String>>(name: S, low: bool, high: bool) -> Self {
        let name = name.into();
        Self {
            bits: array::from_fn(|bit| BitReg::new(format!("{name} bit {bit}"), low, high)),
            name,
        }
    }

    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[must_use]
    pub const fn bit<const BIT: usize>(&self) -> &BitReg {
        const { assert!(BIT < SIZE) }
        &self.bits[BIT]
    }

    #[must_use]
    pub const fn bit_mut<const BIT: usize>(&mut self) -> &mut BitReg {
        const { assert!(BIT < SIZE) }
        &mut self.bits[BIT]
    }

    pub const fn try_bit(&self, bit: usize) -> &BitReg {
        &self.bits[bit]
    }

    pub const fn try_bit_mut(&mut self, bit: usize) -> &mut BitReg {
        &mut self.bits[bit]
    }

    #[must_use]
    pub fn state(&self) -> MBitRegState<SIZE> {
        MBitRegState::new(array::from_fn(|bit| self.bits[bit].state()))
    }

    pub fn add(&mut self, val: usize) {
        for (bit, reg) in self.bits.iter_mut().enumerate() {
            reg.add(bit::bit_of_usize(val, bit));
        }
    }

    pub fn remove_all(&mut self) {
        for reg in self.iter_mut() {
            reg.remove_all();
        }
    }

    pub fn copy_from_bus_state(&mut self, bus: &BusState<SIZE>) {
        for (reg, line) in self.iter_mut().zip(bus.iter()) {
            reg.copy_from_line_state(line);
        }
    }

    pub fn copy_from_reg_state(&mut self, other: &MBitRegState<SIZE>) {
        for (reg, other_reg) in self.iter_mut().zip(other.iter()) {
            reg.copy_from_reg_state(other_reg);
        }
    }

    delegate! {
        to self.bits {
            pub fn iter(&self) -> impl Iterator<Item = &BitReg>;
            pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut BitReg>;
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
    fn reg() -> MBitReg<REG_SIZE> {
        MBitReg::new(REG_NAME, true, true)
    }

    #[rstest]
    fn new_correct_names(reg: MBitReg<REG_SIZE>) {
        for (bit, bit_reg) in reg.iter().enumerate() {
            assert_eq!(bit_reg.name(), format!("{REG_NAME} bit {bit}"));
        }
    }

    #[rstest]
    fn name(reg: MBitReg<REG_SIZE>) {
        assert_eq!(reg.name(), REG_NAME);
    }

    #[rstest]
    #[case(0, [false, false, false, false])]
    #[case(0b101, [true, false, true, false])]
    #[case(0b1011, [true, true, false, true])]
    #[case(0b11011, [true, true, false, true])]
    fn add_only_possible(
        mut reg: MBitReg<REG_SIZE>,
        #[values(true, false)] initial: bool,
        #[case] val: usize,
        #[case] bits: [bool; REG_SIZE],
    ) {
        reg.iter_mut().for_each(|r| r.add(initial));
        reg.add(val);
        for (bit_reg, &b) in reg.iter().zip(bits.iter()) {
            assert_eq!(bit_reg.state().collapsed(), Some(b));
        }
    }

    #[rstest]
    #[case(0, [false, false, false, false])]
    #[case(0b101, [true, false, true, false])]
    #[case(0b1011, [true, true, false, true])]
    #[case(0b11011, [true, true, false, true])]
    fn add_not_only_possible(
        mut reg: MBitReg<REG_SIZE>,
        #[values(true, false)] initial: bool,
        #[case] val: usize,
        #[case] bits: [bool; REG_SIZE],
    ) {
        reg.iter_mut().for_each(|r| r.add(initial));
        reg.add(val);
        for (bit_reg, &b) in reg.iter().zip(bits.iter()) {
            assert!(bit_reg.state().is_possible(initial));
            assert!(bit_reg.state().is_possible(b));
        }
    }
}
