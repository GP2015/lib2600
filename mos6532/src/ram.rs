use crate::RIOTError;

pub struct RAM {
    bytes: [Option<u8>; 128],
}

impl RAM {
    pub fn new() -> Self {
        Self { bytes: [None; 128] }
    }

    pub fn write_byte(&mut self, addr: usize, byte: u8) {
        self.bytes[addr] = Some(byte);
    }

    pub fn read_byte(&self, addr: usize) -> Result<u8, RIOTError> {
        match self.bytes[addr] {
            Some(byte) => Ok(byte),
            None => Err(RIOTError::UninitialisedRAMByte { address: addr }),
        }
    }
}
