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

    #[test]
    fn read_write_byte() {
        let mut ram = Ram::new();
        ram.write_byte(0, 0x67);
        ram.write_byte(127, 0x89);
        assert_eq!(ram.read_byte(0).unwrap(), 0x67);
        assert_eq!(ram.read_byte(127).unwrap(), 0x89);
    }

    #[test]
    fn read_uninitialised_byte() {
        let mut ram = Ram::new();
        ram.write_byte(23, 0x67);
        assert!(ram.read_byte(45).is_err());
    }
}
