use emutils::reg::MBitRegister;
use std::array;

const RAM_SIZE: usize = 128;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Ram {
    bytes: [MBitRegister<8>; RAM_SIZE],
}

impl Ram {
    pub fn new() -> Self {
        Self {
            bytes: array::from_fn(|i| MBitRegister::new(format!("RAM byte {i:x}"), true, true)),
        }
    }

    pub const fn byte(&self, address: u8) -> &MBitRegister<8> {
        #[allow(clippy::indexing_slicing)]
        &self.bytes[address as usize]
    }

    pub const fn byte_mut(&mut self, address: u8) -> &mut MBitRegister<8> {
        #[allow(clippy::indexing_slicing)]
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
    fn read_uninitialised_byte(ram: Ram) {
        for bit in ram.byte(67).iter() {
            assert_eq!(bit.state().possible_reads(), [true, false]);
        }
    }
}
