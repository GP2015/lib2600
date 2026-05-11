use crate::common::{bit, read::single::SingleRead};
use itertools::Itertools;

pub struct MultiRead<const SIZE: usize> {
    inner: [SingleRead; SIZE],
}

impl<const SIZE: usize> MultiRead<SIZE> {
    pub fn new(reads: [SingleRead; SIZE]) -> Self {
        Self { inner: reads }
    }

    pub fn bit<const BIT: usize>(&self) -> SingleRead {
        const { assert!(BIT < SIZE) }
        self.inner[BIT]
    }

    pub fn try_bit(&self, bit: usize) -> Option<SingleRead> {
        self.inner.get(bit).copied()
    }

    pub fn iter(&self) -> impl Iterator<Item = SingleRead> {
        self.inner.iter().copied()
    }

    pub fn iter_possible_reads(&self) -> impl Iterator<Item = usize> {
        self.inner
            .iter()
            .map(|line_state| line_state.possible_reads().iter().copied())
            .multi_cartesian_product()
            .map(|bits| bit::bits_to_usize(bits.into_iter()))
    }

    pub fn could_be_val(&self, val: usize) -> bool {
        for (bit, bit_reg) in self.iter().enumerate() {
            if !bit_reg.could_read((val >> bit) & 1 == 1) {
                return false;
            }
        }
        true
    }

    pub fn could_be_diff(&self, val: usize) -> bool {
        for (bit, bit_reg) in self.iter().enumerate() {
            if bit_reg.could_read((val >> bit) & 1 == 0) {
                return true;
            }
        }
        false
    }

    pub fn could_be_val_diff(&self, val: usize) -> (bool, bool) {
        (self.could_be_val(val), self.could_be_diff(val))
    }
}
