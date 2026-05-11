use crate::common::{
    read::{multi::MultiRead, single::SingleRead},
    reg::single::BitReg,
};
use delegate::delegate;
use std::array;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct MBitReg<const SIZE: usize> {
    bits: [BitReg; SIZE],
}

impl<const SIZE: usize> MBitReg<SIZE> {
    pub fn new(initial_each: SingleRead) -> Self {
        Self {
            bits: array::from_fn(|_| BitReg::new(initial_each)),
        }
    }

    pub fn bit<const BIT: usize>(&self) -> &BitReg {
        const { assert!(BIT < SIZE) }
        &self.bits[BIT]
    }

    pub fn read(&self) -> MultiRead<SIZE> {
        MultiRead::new(array::from_fn(|bit| self.bits[bit].read()))
    }

    pub fn set_to_read(&mut self, read: MultiRead<SIZE>) {
        for (bit_reg, single_read) in self.bits.iter_mut().zip(read.iter()) {
            bit_reg.set_to_read(single_read);
        }
    }

    delegate! {
        to self.bits {
            #[call(get)]
            pub fn try_bit(&self, bit: usize) -> Option<&BitReg>;
            pub fn iter(&self) -> impl Iterator<Item = &BitReg>;
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use rstest::{fixture, rstest};

//     const REG_NAME: &str = "reg";
//     const REG_SIZE: usize = 4;

//     #[fixture]
//     fn reg() -> MBitReg<REG_SIZE> {
//         MBitReg::new(REG_NAME, true, true)
//     }

//     #[rstest]
//     fn new_correct_names(reg: MBitReg<REG_SIZE>) {
//         for (bit, bit_reg) in reg.iter().enumerate() {
//             assert_eq!(bit_reg.name(), format!("{REG_NAME} bit {bit}"));
//         }
//     }

//     #[rstest]
//     fn name(reg: MBitReg<REG_SIZE>) {
//         assert_eq!(reg.name(), REG_NAME);
//     }

//     #[rstest]
//     #[case(0, [false, false, false, false])]
//     #[case(0b101, [true, false, true, false])]
//     #[case(0b1011, [true, true, false, true])]
//     #[case(0b11011, [true, true, false, true])]
//     fn add_only_possible(
//         mut reg: MBitReg<REG_SIZE>,
//         #[values(true, false)] initial: bool,
//         #[case] val: usize,
//         #[case] bits: [bool; REG_SIZE],
//     ) {
//         reg.iter_mut().for_each(|r| r.add(initial));
//         reg.add(val);
//         for (bit_reg, &b) in reg.iter().zip(bits.iter()) {
//             assert_eq!(bit_reg.state().collapsed(), Some(b));
//         }
//     }

//     #[rstest]
//     #[case(0, [false, false, false, false])]
//     #[case(0b101, [true, false, true, false])]
//     #[case(0b1011, [true, true, false, true])]
//     #[case(0b11011, [true, true, false, true])]
//     fn add_not_only_possible(
//         mut reg: MBitReg<REG_SIZE>,
//         #[values(true, false)] initial: bool,
//         #[case] val: usize,
//         #[case] bits: [bool; REG_SIZE],
//     ) {
//         reg.iter_mut().for_each(|r| r.add(initial));
//         reg.add(val);
//         for (bit_reg, &b) in reg.iter().zip(bits.iter()) {
//             assert!(bit_reg.state().is_possible(initial));
//             assert!(bit_reg.state().is_possible(b));
//         }
//     }
// }
