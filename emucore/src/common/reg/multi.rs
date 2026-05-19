use core::{
    array,
    ops::{Index, IndexMut},
};

use crate::common::{read::multi::MultiRead, reg::single::BitReg};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct MBitReg<const SIZE: usize> {
    bits: [BitReg; SIZE],
}

impl<const SIZE: usize> MBitReg<SIZE> {
    pub fn read(&self) -> MultiRead<SIZE> {
        self.bits.each_ref().map(BitReg::read)
    }

    pub fn set_to_read(&mut self, reads: &MultiRead<SIZE>) {
        for (bit_reg, &single_read) in self.bits.iter_mut().zip(reads.iter()) {
            bit_reg.set_to_read(single_read);
        }
    }
}

impl<const SIZE: usize> Index<usize> for MBitReg<SIZE> {
    type Output = BitReg;
    fn index(&self, index: usize) -> &Self::Output {
        #[expect(clippy::indexing_slicing)]
        &self.bits[index]
    }
}

impl<const SIZE: usize> IndexMut<usize> for MBitReg<SIZE> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        #[expect(clippy::indexing_slicing)]
        &mut self.bits[index]
    }
}

impl<const SIZE: usize> From<MultiRead<SIZE>> for MBitReg<SIZE> {
    fn from(reads: MultiRead<SIZE>) -> Self {
        Self {
            bits: reads.each_ref().map(|&read| BitReg::from(read)),
        }
    }
}

impl<const SIZE: usize> From<u16> for MBitReg<SIZE> {
    fn from(value: u16) -> Self {
        Self {
            bits: array::from_fn(|bit| BitReg::from(value >> bit & 1 == 1)),
        }
    }
}
