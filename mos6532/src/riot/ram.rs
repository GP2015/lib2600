use emutils::reg::MBitRegister;
use std::array;

const RAM_SIZE: usize = 128;

pub struct Ram {
    bytes: [MBitRegister; RAM_SIZE],
}

impl Ram {
    pub fn new() -> Self {
        Self {
            bytes: array::from_fn(|i| MBitRegister::new(format!("RAM byte {i:x}"), 8)),
        }
    }

    pub fn byte(&self, address: u8) -> &MBitRegister {
        &self.bytes[address as usize]
    }

    pub fn byte_mut(&mut self, address: u8) -> &mut MBitRegister {
        &mut self.bytes[address as usize]
    }

    pub fn reset(&mut self) {
        for byte in &mut self.bytes {
            for reg in byte.iter_mut() {
                reg.set_all(true, true);
            }
        }
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
            assert_eq!(bit.possible_reads(), [true, false]);
        }
    }

    #[rstest]
    fn reset(mut ram: Ram) {
        ram.byte_mut(23).add(0x45, true).unwrap();
        ram.byte_mut(67).add(0x89, true).unwrap();
        ram.reset();

        for bit in ram.byte(23).iter() {
            assert_eq!(bit.possible_reads(), [true, false]);
        }

        for bit in ram.byte(67).iter() {
            assert_eq!(bit.possible_reads(), [true, false]);
        }
    }
}
