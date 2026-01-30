use crate::{Bus, Riot, RiotError, SinglePin};

impl Riot {
    pub fn reset_pulse(&mut self) -> Result<(), RiotError> {
        self.pin.res.drive_in(false)?;
        self.pulse_phi2()?;
        Ok(())
    }

    pub fn select(&mut self) -> Result<(), RiotError> {
        self.pin.cs1.drive_in(true)?;
        self.pin.cs2.drive_in(false)
    }

    fn general_pulse(&mut self) -> Result<(), RiotError> {
        self.pin.res.drive_in(true)?;
        self.select()?;
        self.pulse_phi2()
    }

    fn general_ram_pulse(&mut self, rw: bool, address: usize) -> Result<(), RiotError> {
        self.pin.rs.drive_in(false)?;
        self.pin.rw.drive_in(rw)?;
        self.pin.a.drive_value_in(address)?;
        self.general_pulse()
    }

    pub fn write_ram_pulse(&mut self, address: usize, data: usize) -> Result<(), RiotError> {
        self.pin.db.drive_value_in(data)?;
        self.general_ram_pulse(false, address)
    }

    pub fn read_ram_pulse(&mut self, address: usize) -> Result<usize, RiotError> {
        self.general_ram_pulse(true, address)?;
        self.pin.db.read()
    }

    fn general_io_pulse(&mut self, a0: bool, a1: bool, rw: bool) -> Result<(), RiotError> {
        self.pin.rs.drive_in(true)?;
        self.pin.rw.drive_in(rw)?;
        self.pin.a.drive_in_bit(2, false)?;
        self.pin.a.drive_in_bit(0, a0)?;
        self.pin.a.drive_in_bit(1, a1)?;
        self.general_pulse()
    }

    pub fn write_ora_pulse(&mut self, data: usize) -> Result<(), RiotError> {
        self.pin.db.drive_value_in(data)?;
        self.general_io_pulse(false, false, false)
    }

    pub fn read_ora_pulse(&mut self) -> Result<usize, RiotError> {
        self.general_io_pulse(false, false, true)?;
        self.pin.db.read()
    }

    pub fn write_orb_pulse(&mut self, data: usize) -> Result<(), RiotError> {
        self.pin.db.drive_value_in(data)?;
        self.general_io_pulse(false, true, false)
    }

    pub fn read_orb_pulse(&mut self) -> Result<usize, RiotError> {
        self.general_io_pulse(false, true, true)?;
        self.pin.db.read()
    }

    pub fn write_ddra_pulse(&mut self, data: usize) -> Result<(), RiotError> {
        self.pin.db.drive_value_in(data)?;
        self.general_io_pulse(true, false, false)
    }

    pub fn read_ddra_pulse(&mut self) -> Result<usize, RiotError> {
        self.general_io_pulse(true, false, true)?;
        self.pin.db.read()
    }

    pub fn write_ddrb_pulse(&mut self, data: usize) -> Result<(), RiotError> {
        self.pin.db.drive_value_in(data)?;
        self.general_io_pulse(true, true, false)
    }

    pub fn read_ddrb_pulse(&mut self) -> Result<usize, RiotError> {
        self.general_io_pulse(true, true, true)?;
        self.pin.db.read()
    }

    // Add timer control methods here.

    pub fn read_interrupt_flag_pulse(&mut self) -> Result<usize, RiotError> {
        self.pin.rs.drive_in(true)?;
        self.pin.rw.drive_in(true)?;
        self.pin.a.drive_in_bit(2, true)?;
        self.pin.a.drive_in_bit(0, true)?;
        self.general_pulse()?;
        self.pin.db.read()
    }

    pub fn write_edc_pulse(
        &mut self,
        enable_irq: bool,
        use_pos_edge: bool,
    ) -> Result<(), RiotError> {
        self.pin.rs.drive_in(true)?;
        self.pin.rw.drive_in(false)?;
        self.pin.a.drive_in_bit(4, false)?;
        self.pin.a.drive_in_bit(2, true)?;
        self.pin.a.drive_in_bit(1, enable_irq)?;
        self.pin.a.drive_in_bit(0, use_pos_edge)?;
        self.general_pulse()
    }
}
