use crate::{Riot, RiotError};
use emu_utils::pin::{Bus, BusOutput};

impl Riot {
    pub(super) fn write_ram(&mut self) -> Result<(), RiotError> {
        let addr = self.a().read()?;
        let byte = self.db().read()?;
        self.ram.write_byte(addr, byte as u8);
        Ok(())
    }

    pub(super) fn read_ram(&mut self) -> Result<(), RiotError> {
        let addr = self.a().read()?;
        let byte = self.ram.read_byte(addr)?;
        self.db_o().drive_out(byte as usize)?;
        Ok(())
    }
}
