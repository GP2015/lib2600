use crate::{Riot, RiotError};

impl Riot {
    pub(super) fn write_ram(&mut self) -> Result<(), RiotError> {
        let addr = self.pin.a.read()?;
        let byte = self.pin.db.read()?;
        self.ram.write_byte(addr, byte);
        Ok(())
    }

    pub(super) fn read_ram(&mut self) -> Result<(), RiotError> {
        let addr = self.pin.a.read()?;
        let byte = self.ram.read_byte(addr)?;
        self.pin.db.drive_value_out(byte)?;
        Ok(())
    }
}
