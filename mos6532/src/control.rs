mod io;
mod ram;
mod reset;

use crate::{RIOT, RIOTError, data::AOrB};

impl RIOT {
    pub(crate) fn tick(&mut self) -> Result<(), RIOTError> {
        if !self.buf.res.read()? {
            self.reset();
            return Ok(());
        }

        if !self.buf.cs1.read()? || self.buf.cs2.read()? {
            return Ok(());
        }

        let rs = self.buf.rs.read()? as u8;
        let rw = self.buf.rw.read()? as u8;
        let a4 = self.buf.a.read_bit(4)? as u8;
        let a3 = self.buf.a.read_bit(3)? as u8;
        let a2 = self.buf.a.read_bit(2)? as u8;
        let a1 = self.buf.a.read_bit(1)? as u8;
        let a0 = self.buf.a.read_bit(0)? as u8;

        match (rs, rw, a4, a3, a2, a1, a0) {
            (0, 0, _, _, _, _, _) => self.write_ram()?,
            (0, 1, _, _, _, _, _) => self.read_ram()?,
            (1, 0, _, _, 0, 0, 1) => self.write_ddr(AOrB::A)?,
            (1, 1, _, _, 0, 0, 1) => self.read_ddr(AOrB::A)?,
            (1, 0, _, _, 0, 1, 1) => self.write_ddr(AOrB::B)?,
            (1, 1, _, _, 0, 1, 1) => self.read_ddr(AOrB::B)?,
            (1, 0, _, _, 0, 0, 0) => self.write_or(AOrB::A)?,
            (1, 1, _, _, 0, 0, 0) => self.read_ora()?,
            (1, 0, _, _, 0, 1, 0) => self.write_or(AOrB::B)?,
            (1, 1, _, _, 0, 1, 0) => self.read_orb()?,
            _ => (),
        }

        self.update_peripheral(AOrB::A)?;
        self.update_peripheral(AOrB::B)?;

        Ok(())
    }
}
