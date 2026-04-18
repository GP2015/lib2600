use crate::{Riot, RiotError};
use emutils::pin::{BusInputUI, PinInputUI};

impl Riot {
    pub fn pulse_phi2(&mut self) -> Result<(), RiotError> {
        self.phi2_mut().add_drive_in(false, true)?;
        self.phi2_mut().add_drive_in(true, true)?;
        self.phi2_mut().add_drive_in(false, true)?;
        Ok(())
    }

    pub fn reset_pulse(&mut self) -> Result<(), RiotError> {
        self.res_mut().add_drive_in(false, true)?;
        self.pulse_phi2()?;
        Ok(())
    }

    pub fn select(&mut self) -> Result<(), RiotError> {
        self.cs1_mut().add_drive_in(true, true)?;
        self.cs2_mut().add_drive_in(false, true)?;
        Ok(())
    }

    fn general_pulse(&mut self) -> Result<(), RiotError> {
        self.res_mut().add_drive_in(true, true)?;
        self.select()?;
        self.pulse_phi2()
    }

    fn general_ram_pulse(&mut self, rw: bool, address: usize) -> Result<(), RiotError> {
        self.rs_mut().add_drive_in(false, true)?;
        self.rw_mut().add_drive_in(rw, true)?;
        self.a_mut().add_drive_in(address, true)?;
        self.general_pulse()
    }

    pub fn write_ram_pulse(&mut self, address: usize, data: usize) -> Result<(), RiotError> {
        self.db_mut().add_drive_in(data, true)?;
        self.general_ram_pulse(false, address)
    }

    pub fn read_ram_pulse(&mut self, address: usize) -> Result<Option<usize>, RiotError> {
        self.general_ram_pulse(true, address)?;
        Ok(self.db().read())
    }

    fn general_io_pulse(&mut self, a0: bool, a1: bool, rw: bool) -> Result<(), RiotError> {
        self.rs_mut().add_drive_in(true, true)?;
        self.rw_mut().add_drive_in(rw, true)?;
        self.a_mut().pin_mut(2)?.add_drive_in(false, true)?;
        self.a_mut().pin_mut(1)?.add_drive_in(a1, true)?;
        self.a_mut().pin_mut(0)?.add_drive_in(a0, true)?;
        self.general_pulse()
    }

    pub fn write_ora_pulse(&mut self, data: usize) -> Result<(), RiotError> {
        self.db_mut().add_drive_in(data, true)?;
        self.general_io_pulse(false, false, false)
    }

    pub fn read_ora_pulse(&mut self) -> Result<Option<usize>, RiotError> {
        self.general_io_pulse(false, false, true)?;
        Ok(self.db().read())
    }

    pub fn write_orb_pulse(&mut self, data: usize) -> Result<(), RiotError> {
        self.db_mut().add_drive_in(data, true)?;
        self.general_io_pulse(false, true, false)
    }

    pub fn read_orb_pulse(&mut self) -> Result<Option<usize>, RiotError> {
        self.general_io_pulse(false, true, true)?;
        Ok(self.db().read())
    }

    pub fn write_ddra_pulse(&mut self, data: usize) -> Result<(), RiotError> {
        self.db_mut().add_drive_in(data, true)?;
        self.general_io_pulse(true, false, false)
    }

    pub fn read_ddra_pulse(&mut self) -> Result<Option<usize>, RiotError> {
        self.general_io_pulse(true, false, true)?;
        Ok(self.db().read())
    }

    pub fn write_ddrb_pulse(&mut self, data: usize) -> Result<(), RiotError> {
        self.db_mut().add_drive_in(data, true)?;
        self.general_io_pulse(true, true, false)
    }

    pub fn read_ddrb_pulse(&mut self) -> Result<Option<usize>, RiotError> {
        self.general_io_pulse(true, true, true)?;
        Ok(self.db().read())
    }

    // Add timer control methods here.

    pub fn read_interrupt_flag_pulse(&mut self) -> Result<Option<usize>, RiotError> {
        self.rs_mut().add_drive_in(true, true)?;
        self.rw_mut().add_drive_in(true, true)?;
        self.a_mut().pin_mut(2)?.add_drive_in(true, true)?;
        self.a_mut().pin_mut(0)?.add_drive_in(true, true)?;
        self.general_pulse()?;
        Ok(self.db().read())
    }

    pub fn write_edc_pulse(
        &mut self,
        enable_irq: bool,
        use_pos_edge: bool,
    ) -> Result<(), RiotError> {
        self.rs_mut().add_drive_in(true, true)?;
        self.rw_mut().add_drive_in(false, true)?;
        self.a_mut().pin_mut(4)?.add_drive_in(false, true)?;
        self.a_mut().pin_mut(2)?.add_drive_in(true, true)?;
        self.a_mut().pin_mut(1)?.add_drive_in(enable_irq, true)?;
        self.a_mut().pin_mut(0)?.add_drive_in(use_pos_edge, true)?;
        self.general_pulse()
    }
}
