use crate::{Riot, RiotError};

/// Some extra helper methods to help reduce boilerplate.
/// Instead of setting the pins individually for each operation,
/// you can call these methods and they will handle the pins for you.
///
/// These methods are simply abstractions over the core methods.
/// They do not provide any extra functionality.
impl Riot {
    pub fn reset_pulse(&mut self) -> Result<(), RiotError> {
        self.write_res(false);
        self.pulse_phi2()?;
        self.write_res(true);
        Ok(())
    }

    pub fn select(&mut self) {
        self.write_cs1(true);
        self.write_cs2(false);
    }

    pub fn deselected_pulse(&mut self) -> Result<(), RiotError> {
        self.write_cs1(false);
        self.pulse_phi2()
    }

    fn general_pulse(&mut self) -> Result<(), RiotError> {
        self.write_res(true);
        self.select();
        self.pulse_phi2()
    }

    fn general_ram_pulse(&mut self, rw: bool, address: usize) -> Result<(), RiotError> {
        self.write_rw(rw);
        self.write_rs(false);
        self.write_a(address)?;
        self.general_pulse()
    }

    pub fn write_ram_pulse(&mut self, address: usize, data: u8) -> Result<(), RiotError> {
        self.write_db(data);
        self.general_ram_pulse(false, address)
    }

    pub fn read_ram_pulse(&mut self, address: usize) -> Result<u8, RiotError> {
        self.general_ram_pulse(true, address)?;
        self.read_db()
    }

    fn general_io_pulse(&mut self, a0: bool, a1: bool, rw: bool) -> Result<(), RiotError> {
        self.write_a_bit(0, a0).unwrap();
        self.write_a_bit(1, a1).unwrap();
        self.write_rw(rw);
        self.write_rs(true);
        self.write_a_bit(2, false).unwrap();
        self.general_pulse()
    }

    pub fn write_ora_pulse(&mut self, data: u8) -> Result<(), RiotError> {
        self.write_db(data);
        self.general_io_pulse(false, false, false)
    }

    pub fn read_ora_pulse(&mut self) -> Result<u8, RiotError> {
        self.general_io_pulse(false, false, true)?;
        self.read_db()
    }

    pub fn write_orb_pulse(&mut self, data: u8) -> Result<(), RiotError> {
        self.write_db(data);
        self.general_io_pulse(false, true, false)
    }

    pub fn read_orb_pulse(&mut self) -> Result<u8, RiotError> {
        self.general_io_pulse(false, true, true)?;
        self.read_db()
    }

    pub fn write_ddra_pulse(&mut self, data: u8) -> Result<(), RiotError> {
        self.write_db(data);
        self.general_io_pulse(true, false, false)
    }

    pub fn read_ddra_pulse(&mut self) -> Result<u8, RiotError> {
        self.general_io_pulse(true, false, true)?;
        self.read_db()
    }

    pub fn write_ddrb_pulse(&mut self, data: u8) -> Result<(), RiotError> {
        self.write_db(data);
        self.general_io_pulse(true, true, false)
    }

    pub fn read_ddrb_pulse(&mut self) -> Result<u8, RiotError> {
        self.general_io_pulse(true, true, true)?;
        self.read_db()
    }

    // Add timer control methods here.

    pub fn read_interrupt_flag_pulse(&mut self) -> Result<u8, RiotError> {
        self.write_rs(true);
        self.write_rw(true);
        self.write_a_bit(2, true).unwrap();
        self.write_a_bit(0, true).unwrap();
        self.general_pulse()?;
        self.read_db()
    }

    pub fn write_edc_pulse(
        &mut self,
        enable_irq: bool,
        use_pos_edge: bool,
    ) -> Result<(), RiotError> {
        self.write_rs(true);
        self.write_rw(false);
        self.write_a_bit(4, false).unwrap();
        self.write_a_bit(2, true).unwrap();
        self.write_a_bit(1, enable_irq).unwrap();
        self.write_a_bit(0, use_pos_edge).unwrap();
        self.general_pulse()
    }
}
