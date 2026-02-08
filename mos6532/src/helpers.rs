use crate::{Bus, Riot, RiotError, SinglePin};

impl Riot {
    pub fn pulse_phi2(&mut self) -> Result<(), RiotError> {
        self.phi2_mut().drive_in(false)?;
        self.phi2_mut().drive_in(true)?;
        self.phi2_mut().drive_in(false)
    }

    pub fn reset_pulse(&mut self) -> Result<(), RiotError> {
        self.res_mut().drive_in(false)?;
        self.pulse_phi2()?;
        Ok(())
    }

    pub fn select(&mut self) -> Result<(), RiotError> {
        self.cs1_mut().drive_in(true)?;
        self.cs2_mut().drive_in(false)
    }

    fn general_pulse(&mut self) -> Result<(), RiotError> {
        self.res_mut().drive_in(true)?;
        self.select()?;
        self.pulse_phi2()
    }

    fn general_ram_pulse(&mut self, rw: bool, address: usize) -> Result<(), RiotError> {
        self.rs_mut().drive_in(false)?;
        self.rw_mut().drive_in(rw)?;
        self.a_mut().drive_in(address)?;
        self.general_pulse()
    }

    pub fn write_ram_pulse(&mut self, address: usize, data: usize) -> Result<(), RiotError> {
        self.db_mut().drive_in(data)?;
        self.general_ram_pulse(false, address)
    }

    pub fn read_ram_pulse(&mut self, address: usize) -> Result<usize, RiotError> {
        self.general_ram_pulse(true, address)?;
        self.db().read()
    }

    fn general_io_pulse(&mut self, a0: bool, a1: bool, rw: bool) -> Result<(), RiotError> {
        self.rs_mut().drive_in(true)?;
        self.rw_mut().drive_in(rw)?;
        self.a_mut().pin_mut(2)?.drive_in(false)?;
        self.a_mut().pin_mut(1)?.drive_in(a1)?;
        self.a_mut().pin_mut(0)?.drive_in(a0)?;
        self.general_pulse()
    }

    pub fn write_ora_pulse(&mut self, data: usize) -> Result<(), RiotError> {
        self.db_mut().drive_in(data)?;
        self.general_io_pulse(false, false, false)
    }

    pub fn read_ora_pulse(&mut self) -> Result<usize, RiotError> {
        self.general_io_pulse(false, false, true)?;
        self.db().read()
    }

    pub fn write_orb_pulse(&mut self, data: usize) -> Result<(), RiotError> {
        self.db_mut().drive_in(data)?;
        self.general_io_pulse(false, true, false)
    }

    pub fn read_orb_pulse(&mut self) -> Result<usize, RiotError> {
        self.general_io_pulse(false, true, true)?;
        self.db().read()
    }

    pub fn write_ddra_pulse(&mut self, data: usize) -> Result<(), RiotError> {
        self.db_mut().drive_in(data)?;
        self.general_io_pulse(true, false, false)
    }

    pub fn read_ddra_pulse(&mut self) -> Result<usize, RiotError> {
        self.general_io_pulse(true, false, true)?;
        self.db().read()
    }

    pub fn write_ddrb_pulse(&mut self, data: usize) -> Result<(), RiotError> {
        self.db_mut().drive_in(data)?;
        self.general_io_pulse(true, true, false)
    }

    pub fn read_ddrb_pulse(&mut self) -> Result<usize, RiotError> {
        self.general_io_pulse(true, true, true)?;
        self.db().read()
    }

    // Add timer control methods here.

    pub fn read_interrupt_flag_pulse(&mut self) -> Result<usize, RiotError> {
        self.rs_mut().drive_in(true)?;
        self.rw_mut().drive_in(true)?;
        self.a_mut().pin_mut(2)?.drive_in(true)?;
        self.a_mut().pin_mut(0)?.drive_in(true)?;
        self.general_pulse()?;
        self.db().read()
    }

    pub fn write_edc_pulse(
        &mut self,
        enable_irq: bool,
        use_pos_edge: bool,
    ) -> Result<(), RiotError> {
        self.rs_mut().drive_in(true)?;
        self.rw_mut().drive_in(false)?;
        self.a_mut().pin_mut(4)?.drive_in(false)?;
        self.a_mut().pin_mut(2)?.drive_in(true)?;
        self.a_mut().pin_mut(1)?.drive_in(enable_irq)?;
        self.a_mut().pin_mut(0)?.drive_in(use_pos_edge)?;
        self.general_pulse()
    }
}
