use crate::{RIOT, RIOTError};

impl RIOT {
    pub(crate) fn tick(&mut self) -> Result<(), RIOTError> {
        if !self.pin.res.read()? {
            self.reset();
            return Ok(());
        }

        if !self.pin.cs1.read()? || self.pin.cs2.read()? {
            return Ok(());
        }

        match self.pin.rs.read()? {
            false => self.ram_control()?,
            true => (),
        }

        Ok(())
    }

    fn reset(&mut self) {
        self.pin.irq.reset();
        self.reg.ddra = Some(0);
        self.reg.ddrb = Some(0);
        self.reg.ora = Some(0);
        self.reg.orb = Some(0);
    }

    fn ram_control(&mut self) -> Result<(), RIOTError> {
        let addr = self.pin.a.read()?;

        match self.pin.rw.read()? {
            true => {
                let byte = self.ram.read_byte(addr)?;
                self.pin.db.drive(byte as usize).unwrap();
            }
            false => {
                let byte = self.pin.db.read()? as u8;
                self.ram.write_byte(addr, byte);
            }
        }

        Ok(())
    }
}
