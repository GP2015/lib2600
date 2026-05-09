use emutils::reg::MBitReg;
use std::array;

const RAM_SIZE: usize = 128;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Ram {
    bytes: [MBitReg<8>; RAM_SIZE],
}

impl Ram {
    pub fn new() -> Self {
        Self {
            bytes: array::from_fn(|i| MBitReg::new(format!("RAM byte {i:x}"), true, true)),
        }
    }

    pub const fn byte_mut(&mut self, address: u8) -> &mut MBitReg<8> {
        &mut self.bytes[address as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::{fixture, rstest};

    #[fixture]
    fn ram() -> Ram {
        Ram::new()
    }

    #[rstest]
    fn read_uninitialised_byte(mut ram: Ram) {
        for bit in ram.byte_mut(67).iter_mut() {
            assert_eq!(bit.state().possible_reads(), [true, false]);
        }
    }
}
