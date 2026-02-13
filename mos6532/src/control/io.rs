use crate::{Riot, RiotError};
use emu_utils::pin::{BusInterface, BusOutput, SinglePinInterface, SinglePinOutput};

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
        let byte = self.db().read()?;

        match reg {
            ATYPE => &mut self.reg.ddra,
            BTYPE => &mut self.reg.ddrb,
        }
        .write(byte)?;

        self.update_peripheral(reg)
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

        self.db_out().drive_out(byte)?;
        Ok(())
    }

    pub(super) fn write_ora(&mut self) -> Result<(), RiotError> {
        self.write_or(ATYPE)
    }

    pub(super) fn write_orb(&mut self) -> Result<(), RiotError> {
        self.write_or(BTYPE)
    }

    fn write_or(&mut self, reg: bool) -> Result<(), RiotError> {
        let byte = self.db().read()?;

        match reg {
            ATYPE => &mut self.reg.ora,
            BTYPE => &mut self.reg.orb,
        }
        .write(byte)?;

        self.update_peripheral(reg)
    }

    pub(super) fn read_ora(&mut self) -> Result<(), RiotError> {
        let byte = self.pa().read()?;
        self.db_out().drive_out(byte)?;
        Ok(())
    }

    pub(super) fn read_orb(&mut self) -> Result<(), RiotError> {
        for bit in 0..8 {
            let state = match self.reg.ddrb.read_bit(bit)? {
                true => self.reg.orb.read_bit(bit)?,
                false => self.pb().pin(bit)?.read()?,
            };
            self.db_out().pin_out(bit)?.drive_out(state)?;
        }

        Ok(())
    }

    fn update_peripheral(&mut self, peripheral: bool) -> Result<(), RiotError> {
        for bit in 0..8 {
            match peripheral {
                ATYPE => {
                    if self.reg.ddra.read_bit(bit)? {
                        let state = self.reg.ora.read_bit(bit)?;
                        self.pa_out().pin_out(bit)?.drive_out(state)?;
                    }
                }
                BTYPE => {
                    if self.reg.ddrb.read_bit(bit)? {
                        let state = self.reg.orb.read_bit(bit)?;
                        self.pb_out().pin_out(bit)?.drive_out(state)?;
                    }
                }
            };
        }

        Ok(())
    }
}
