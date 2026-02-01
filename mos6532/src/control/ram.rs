use crate::{Riot, RiotError};
use emu_utils::pin::{Bus, BusOutput};

impl Riot {
    pub(super) fn write_ram(&mut self) -> Result<(), RiotError> {
        let addr = self.a().read()?;
        let byte = self.db().read()?;
        self.ram.byte(addr).write(byte)?;
        Ok(())
    }

    pub(super) fn read_ram(&mut self) -> Result<(), RiotError> {
        let addr = self.a().read()?;
        let byte = self.ram.byte(addr).read()?;
        self.db_o().drive_out(byte)?;
        Ok(())
    }
}
