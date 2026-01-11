use crate::{Riot, RiotError};

impl Riot {
    pub(super) fn write_ram(&mut self) -> Result<(), RiotError> {
        let addr = self.buf.a.read()?;
        let byte = self.buf.db.read()? as u8;
        self.ram.write_byte(addr, byte);
        Ok(())
    }

    pub(super) fn read_ram(&mut self) -> Result<(), RiotError> {
        let addr = self.buf.a.read()?;
        let byte = self.ram.read_byte(addr)?;
        self.buf.db.write(byte as usize).unwrap();
        Ok(())
    }
}
