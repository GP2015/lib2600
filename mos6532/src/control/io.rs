use crate::{RIOT, RIOTError, data::AOrB};

impl RIOT {
    pub(super) fn write_ddr(&mut self, reg: AOrB) -> Result<(), RIOTError> {
        let byte = self.buf.db.read()?;

        match reg {
            AOrB::A => &mut self.reg.ddra,
            AOrB::B => &mut self.reg.ddrb,
        }
        .drive(byte)
        .unwrap();

        Ok(())
    }

    pub(super) fn read_ddr(&mut self, reg: AOrB) -> Result<(), RIOTError> {
        let byte = match reg {
            AOrB::A => &self.reg.ddra,
            AOrB::B => &self.reg.ddrb,
        }
        .read()?;

        self.buf.db.drive(byte).unwrap();
        Ok(())
    }

    pub(super) fn write_or(&mut self, reg: AOrB) -> Result<(), RIOTError> {
        let byte = self.buf.db.read()?;

        match reg {
            AOrB::A => &mut self.reg.ora,
            AOrB::B => &mut self.reg.orb,
        }
        .drive(byte)
        .unwrap();

        Ok(())
    }

    pub(super) fn read_ora(&mut self) -> Result<(), RIOTError> {
        for bit in 0..8 {
            let state = match self.reg.ddra.read_bit(bit)? {
                true => self.reg.ora.read_bit(bit)?,
                false => self.buf.pa.read_bit(bit)?,
            };
            self.buf.db.drive_bit(bit, state).unwrap();
        }

        Ok(())
    }

    pub(super) fn read_orb(&mut self) -> Result<(), RIOTError> {
        let byte = self.reg.orb.read()?;
        self.buf.db.drive(byte).unwrap();
        Ok(())
    }

    pub(super) fn update_peripheral(&mut self, peripheral: AOrB) -> Result<(), RIOTError> {
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
