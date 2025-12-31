use crate::{RIOT, RIOTError};

enum AOrB {
    A,
    B,
}

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

    fn reset(&mut self) {
        self.buf.irq.reset();
        self.reg.ddra.drive(0).unwrap();
        self.reg.ddrb.drive(0).unwrap();
        self.reg.ora.drive(0).unwrap();
        self.reg.orb.drive(0).unwrap();
    }

    fn write_ram(&mut self) -> Result<(), RIOTError> {
        let addr = self.buf.a.read()?;
        let byte = self.buf.db.read()? as u8;
        self.ram.write_byte(addr, byte);
        Ok(())
    }

    fn read_ram(&mut self) -> Result<(), RIOTError> {
        let addr = self.buf.a.read()?;
        let byte = self.ram.read_byte(addr)?;
        self.buf.db.drive(byte as usize).unwrap();
        Ok(())
    }

    fn write_ddr(&mut self, reg: AOrB) -> Result<(), RIOTError> {
        let byte = self.buf.db.read()?;

        match reg {
            AOrB::A => &mut self.reg.ddra,
            AOrB::B => &mut self.reg.ddrb,
        }
        .drive(byte)
        .unwrap();

        Ok(())
    }

    fn read_ddr(&mut self, reg: AOrB) -> Result<(), RIOTError> {
        let byte = match reg {
            AOrB::A => &self.reg.ddra,
            AOrB::B => &self.reg.ddrb,
        }
        .read()?;

        self.buf.db.drive(byte).unwrap();
        Ok(())
    }

    fn write_or(&mut self, reg: AOrB) -> Result<(), RIOTError> {
        let byte = self.buf.db.read()?;

        match reg {
            AOrB::A => &mut self.reg.ora,
            AOrB::B => &mut self.reg.orb,
        }
        .drive(byte)
        .unwrap();

        Ok(())
    }

    fn read_ora(&mut self) -> Result<(), RIOTError> {
        for bit in 0..8 {
            let state = match self.reg.ddra.read_bit(bit)? {
                true => self.reg.ora.read_bit(bit)?,
                false => self.buf.pa.read_bit(bit)?,
            };
            self.buf.db.drive_bit(bit, state).unwrap();
        }

        Ok(())
    }

    fn read_orb(&mut self) -> Result<(), RIOTError> {
        let byte = self.reg.orb.read()?;
        self.buf.db.drive(byte).unwrap();
        Ok(())
    }

    fn update_peripheral(&mut self, peripheral: AOrB) -> Result<(), RIOTError> {
        for bit in 0..8 {
            match peripheral {
                AOrB::A => {
                    if self.reg.ddra.read_bit(bit)? {
                        let state = self.reg.ora.read_bit(bit)?;
                        self.buf.pa.drive_bit(bit, state).unwrap();
                    }
                }

                AOrB::B => {
                    if self.reg.ddrb.read_bit(bit)? {
                        let state = self.reg.orb.read_bit(bit)?;
                        self.buf.pb.drive_bit(bit, state).unwrap();
                    }
                }
            };
        }

        Ok(())
    }
}
