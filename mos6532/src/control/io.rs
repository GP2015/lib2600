use crate::{Riot, RiotError, data::AOrB};

impl Riot {
    pub(super) fn write_ddr(&mut self, reg: AOrB) -> Result<(), RiotError> {
        let byte = self.buf.db.read()?;

        match reg {
            AOrB::A => &mut self.reg.ddra,
            AOrB::B => &mut self.reg.ddrb,
        }
        .write(byte)
        .unwrap();

        self.update_peripheral(reg)?;

        Ok(())
    }

    pub(super) fn read_ddr(&mut self, reg: AOrB) -> Result<(), RiotError> {
        let byte = match reg {
            AOrB::A => &self.reg.ddra,
            AOrB::B => &self.reg.ddrb,
        }
        .read()?;

        self.buf.db.write(byte).unwrap();
        Ok(())
    }

    pub(super) fn write_or(&mut self, reg: AOrB) -> Result<(), RiotError> {
        let byte = self.buf.db.read()?;

        match reg {
            AOrB::A => &mut self.reg.ora,
            AOrB::B => &mut self.reg.orb,
        }
        .write(byte)
        .unwrap();

        self.update_peripheral(reg)?;

        Ok(())
    }

    pub(super) fn read_ora(&mut self) -> Result<(), RiotError> {
        let byte = self.buf.pa.read()?;
        self.buf.db.write(byte).unwrap();
        Ok(())
    }

    pub(super) fn read_orb(&mut self) -> Result<(), RiotError> {
        for bit in 0..8 {
            let state = match self.reg.ddrb.read_bit(bit)? {
                true => self.reg.orb.read_bit(bit)?,
                false => self.buf.pb.read_bit(bit)?,
            };
            self.buf.db.write_bit(bit, state).unwrap();
        }

        Ok(())
    }

    pub(super) fn update_peripheral(&mut self, peripheral: AOrB) -> Result<(), RiotError> {
        for bit in 0..8 {
            match peripheral {
                AOrB::A => {
                    if self.reg.ddra.read_bit(bit)? {
                        let state = self.reg.ora.read_bit(bit)?;
                        self.buf.pa.write_bit(bit, state).unwrap();
                    }
                }

                AOrB::B => {
                    if self.reg.ddrb.read_bit(bit)? {
                        let state = self.reg.orb.read_bit(bit)?;
                        self.buf.pb.write_bit(bit, state).unwrap();
                    }
                }
            };
        }

        Ok(())
    }
}
