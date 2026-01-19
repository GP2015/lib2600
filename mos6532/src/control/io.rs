use crate::{Riot, RiotError};

const ATYPE: bool = false;
const BTYPE: bool = true;

impl Riot {
    pub(super) fn write_ddra(&mut self) -> Result<(), RiotError> {
        self.write_ddr(ATYPE)
    }

    pub(super) fn write_ddrb(&mut self) -> Result<(), RiotError> {
        self.write_ddr(BTYPE)
    }

    fn write_ddr(&mut self, reg: bool) -> Result<(), RiotError> {
        let byte = self.buf.db.read()?;

        match reg {
            ATYPE => &mut self.reg.ddra,
            BTYPE => &mut self.reg.ddrb,
        }
        .write(byte)
        .unwrap();

        self.update_peripheral(reg)?;

        Ok(())
    }

    pub(super) fn read_ddra(&mut self) -> Result<(), RiotError> {
        self.read_ddr(ATYPE)
    }

    pub(super) fn read_ddrb(&mut self) -> Result<(), RiotError> {
        self.read_ddr(BTYPE)
    }

    fn read_ddr(&mut self, reg: bool) -> Result<(), RiotError> {
        let byte = match reg {
            ATYPE => &self.reg.ddra,
            BTYPE => &self.reg.ddrb,
        }
        .read()?;

        self.buf.db.write(byte).unwrap();
        Ok(())
    }

    pub(super) fn write_ora(&mut self) -> Result<(), RiotError> {
        self.write_or(ATYPE)
    }

    pub(super) fn write_orb(&mut self) -> Result<(), RiotError> {
        self.write_or(BTYPE)
    }

    fn write_or(&mut self, reg: bool) -> Result<(), RiotError> {
        let byte = self.buf.db.read()?;

        match reg {
            ATYPE => &mut self.reg.ora,
            BTYPE => &mut self.reg.orb,
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

    pub(super) fn update_peripheral(&mut self, peripheral: bool) -> Result<(), RiotError> {
        for bit in 0..8 {
            match peripheral {
                ATYPE => {
                    if self.reg.ddra.read_bit(bit)? {
                        let state = self.reg.ora.read_bit(bit)?;
                        self.buf.pa.write_bit(bit, state).unwrap();
                    }
                }

                BTYPE => {
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
