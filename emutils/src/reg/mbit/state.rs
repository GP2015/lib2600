use crate::{bit, reg::BitRegState};
use itertools::Itertools;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct MBitRegState<const SIZE: usize> {
    pub bit_states: [BitRegState; SIZE],
}

impl<const SIZE: usize> MBitRegState<SIZE> {
    #[must_use]
    pub const fn new(bit_states: [BitRegState; SIZE]) -> Self {
        Self { bit_states }
    }

    #[must_use]
    pub const fn size(&self) -> usize {
        SIZE
    }

    #[must_use]
    pub const fn bit_state<const BIT: usize>(&self) -> BitRegState {
        const { assert!(BIT < SIZE) }
        #[allow(clippy::indexing_slicing)]
        self.bit_states[BIT]
    }

    #[must_use]
    pub fn try_bit_state(&self, bit: usize) -> Option<BitRegState> {
        self.bit_states.get(bit).copied()
    }

    pub fn iter(&self) -> impl Iterator<Item = BitRegState> {
        self.bit_states.iter().copied()
    }

    #[must_use]
    pub fn read(&self) -> Option<usize> {
        bit::some_bits_to_usize(self.bit_states.iter().map(|bit| bit.collapsed()))
    }

    pub fn iter_possible_reads(&self) -> impl Iterator<Item = usize> {
        self.bit_states
            .iter()
            .map(|line_state| line_state.possible_reads().iter().copied())
            .multi_cartesian_product()
            .map(|bits| bit::bits_to_usize(bits.into_iter()))
    }

    #[must_use]
    pub fn could_be_val(&self, val: usize) -> bool {
        for (bit, bit_reg) in self.iter().enumerate() {
            if !bit_reg.is_possible((val >> bit) & 1 == 1) {
                return false;
            }
        }
        true
    }

    #[must_use]
    pub fn could_be_diff(&self, val: usize) -> bool {
        for (bit, bit_reg) in self.iter().enumerate() {
            if bit_reg.is_possible((val >> bit) & 1 == 0) {
                return true;
            }
        }
        false
    }

    #[must_use]
    pub fn could_be_val_diff(&self, val: usize) -> (bool, bool) {
        (self.could_be_val(val), self.could_be_diff(val))
    }

    #[must_use]
    pub fn decremented(&self) -> Self {
        let mut res = self.clone();
        let mut must_carry = true;

        for bit in &mut res.bit_states {
            match bit.low_high_possible() {
                (false, false) => unreachable!(),
                (false, true) => {
                    bit.low = true;

                    if must_carry {
                        bit.high = false;
                    }

                    break;
                }
                (true, false) => {
                    bit.high = true;

                    if must_carry {
                        bit.low = false;
                    }
                }
                (true, true) => {
                    if must_carry {
                        bit.high = false;
                    }

                    must_carry = false;
                }
            }
        }

        res
    }
}
