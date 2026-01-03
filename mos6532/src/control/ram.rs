use crate::{RIOT, RIOTError};

impl RIOT {
    pub(super) fn write_ram(&mut self) -> Result<(), RIOTError> {
        let addr = self.buf.a.read()?;
        let byte = self.buf.db.read()? as u8;
        self.ram.write_byte(addr, byte);
        Ok(())
    }

    pub(super) fn read_ram(&mut self) -> Result<(), RIOTError> {
        let addr = self.buf.a.read()?;
        let byte = self.ram.read_byte(addr)?;
        self.buf.db.drive(byte as usize).unwrap();
        Ok(())
    }
}
