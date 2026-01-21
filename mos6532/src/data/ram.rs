use crate::RiotError;

const RAM_SIZE: usize = 128;

pub struct Ram {
    bytes: [Option<u8>; RAM_SIZE],
}

impl Ram {
    pub fn new() -> Self {
        Self {
            bytes: [None; RAM_SIZE],
        }
    }

    pub fn write_byte(&mut self, address: usize, byte: u8) {
        self.bytes[address] = Some(byte);
    }

    pub fn read_byte(&self, address: usize) -> Result<u8, RiotError> {
        match self.bytes[address] {
            Some(byte) => Ok(byte),
            None => Err(RiotError::UninitialisedRAMByte { address }),
        }
    }

    pub fn reset(&mut self) {
        self.bytes = [None; RAM_SIZE];
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
    fn read_write_byte(mut ram: Ram) {
        ram.write_byte(0, 0x67);
        ram.write_byte(127, 0x89);
        assert_eq!(ram.read_byte(0).unwrap(), 0x67);
        assert_eq!(ram.read_byte(127).unwrap(), 0x89);
    }

    #[rstest]
    fn overwrite_byte(mut ram: Ram) {
        ram.write_byte(23, 0x67);
        ram.write_byte(23, 0x89);
        assert_eq!(ram.read_byte(23).unwrap(), 0x89);
    }

    #[rstest]
    fn read_uninitialised_byte(mut ram: Ram) {
        ram.write_byte(23, 0x67);
        assert!(ram.read_byte(45).is_err());
    }

    #[rstest]
    fn reset_byte(mut ram: Ram) {
        ram.write_byte(23, 0x67);
        ram.reset();
        assert!(ram.read_byte(23).is_err());
    }
}
