use crate::common::read::{multi::MultiRead, single::SingleRead};

pub type BitReg = SingleRead;
pub type MBitReg<const SIZE: usize> = MultiRead<SIZE>;
