mod instructions;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum CPUError {
    #[error("the specified bit ({0}) does not exist")]
    InvalidBit(usize),

    #[error("the provided value ({0}) cannot fit in the bus")]
    ValueTooLarge(usize),
}

const A_SIZE: usize = 13;
const D_SIZE: usize = 8;

fn get_bit_of_usize(val: usize, bit: usize) -> bool {
    (val >> bit) & 1 == 1
}

fn set_bit_of_usize(val: usize, bit: usize, state: bool) -> usize {
    match state {
        true => val | (1 << bit),
        false => val & !(1 << bit),
    }
}

fn val_exceeds_bit_count(val: usize, bit_count: usize) -> bool {
    val >> bit_count != 0
}

fn get_low_bits_of_usize(val: usize, bit_count: usize) -> usize {
    val & (1 << bit_count) - 1
}

struct Pins {
    res: bool,
    rdy: bool,
    a: usize,
    d: usize,
    rw: bool,
    phi0: bool,
    phi2: bool,
}

impl Pins {
    pub fn new() -> Self {
        Self {
            res: false,
            rdy: false,
            a: 0,
            d: 0,
            rw: false,
            phi0: false,
            phi2: false,
        }
    }
}

struct Registers {
    a: u8,
    x: u8,
    y: u8,
    pc: u16,
    sp: u8,
    sr: u8,
}

impl Registers {
    pub fn new() -> Self {
        Self {
            a: 0,
            x: 0,
            y: 0,
            pc: 0,
            sp: 0,
            sr: 0,
        }
    }
}

pub struct CPU {
    pin: Pins,
    reg: Registers,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            pin: Pins::new(),
            reg: Registers::new(),
        }
    }

    pub fn drv_res(&mut self, state: bool) {
        self.pin.res = state;
    }

    pub fn drv_rdy(&mut self, state: bool) {
        self.pin.rdy = state;
    }

    pub fn rd_a(&self) -> usize {
        self.pin.a
    }

    pub fn rd_a_bit(&self, bit: usize) -> Result<bool, CPUError> {
        if bit >= A_SIZE {
            return Err(CPUError::InvalidBit(bit));
        }

        Ok(get_bit_of_usize(self.pin.a, bit))
    }

    pub fn rd_d(&self) -> usize {
        self.pin.d
    }

    pub fn rd_d_bit(&self, bit: usize) -> Result<bool, CPUError> {
        if bit >= D_SIZE {
            return Err(CPUError::InvalidBit(bit));
        }

        Ok(get_bit_of_usize(self.pin.d, bit))
    }

    pub fn drv_d(&mut self, val: usize) -> Result<(), CPUError> {
        if val_exceeds_bit_count(val, D_SIZE) {
            return Err(CPUError::ValueTooLarge(val));
        }

        self.pin.d = val;
        Ok(())
    }

    pub fn drv_d_wrap(&mut self, val: usize) {
        self.pin.d = get_low_bits_of_usize(val, D_SIZE);
    }

    pub fn drv_d_bit(&mut self, bit: usize, state: bool) -> Result<(), CPUError> {
        if bit >= D_SIZE {
            return Err(CPUError::InvalidBit(bit));
        }

        self.pin.d = set_bit_of_usize(self.pin.d, bit, state);
        Ok(())
    }

    pub fn rd_rw(&self) -> bool {
        self.pin.rw
    }

    pub fn drv_phi0(&mut self, state: bool) {
        self.pin.phi0 = state;
    }

    pub fn rd_phi2(&self) -> bool {
        self.pin.phi2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_bit_of_usize_test() {
        assert_eq!(get_bit_of_usize(0b101, 0), true);
        assert_eq!(get_bit_of_usize(0b101, 1), false);
        assert_eq!(get_bit_of_usize(0b101, 7), false);
    }

    #[test]
    fn set_bit_of_usize_test() {
        assert_eq!(set_bit_of_usize(0b101, 0, false), 0b100);
        assert_eq!(set_bit_of_usize(0b101, 0, true), 0b101);
        assert_eq!(set_bit_of_usize(0b101, 1, false), 0b101);
        assert_eq!(set_bit_of_usize(0b101, 1, true), 0b111);
        assert_eq!(set_bit_of_usize(0b101, 4, false), 0b101);
        assert_eq!(set_bit_of_usize(0b101, 4, true), 0b10101);
    }

    #[test]
    fn val_exceeds_bit_count_test() {
        assert_eq!(val_exceeds_bit_count(0b1011, 3), true);
        assert_eq!(val_exceeds_bit_count(0b1011, 4), false);
        assert_eq!(val_exceeds_bit_count(0b1011, 5), false);
    }

    #[test]
    fn get_low_bits_of_usize_test() {
        assert_eq!(get_low_bits_of_usize(0b1011, 0), 0);
        assert_eq!(get_low_bits_of_usize(0b1011, 1), 1);
        assert_eq!(get_low_bits_of_usize(0b1011, 2), 0b11);
        assert_eq!(get_low_bits_of_usize(0b1011, 3), 0b11);
        assert_eq!(get_low_bits_of_usize(0b1011, 7), 0b1011);
    }

    #[test]
    fn drive_reset_pin() {
        let mut cpu = CPU::new();
        cpu.drv_res(true);
        assert_eq!(cpu.pin.res, true);
        cpu.drv_res(false);
        assert_eq!(cpu.pin.res, false);
    }

    #[test]
    fn drive_ready_pin() {
        let mut cpu = CPU::new();
        cpu.drv_rdy(true);
        assert_eq!(cpu.pin.rdy, true);
        cpu.drv_rdy(false);
        assert_eq!(cpu.pin.rdy, false);
    }

    #[test]
    fn read_address_bus() {
        let mut cpu = CPU::new();
        cpu.pin.a = 0x67;
        assert_eq!(cpu.rd_a(), 0x67);
    }

    #[test]
    fn read_address_bus_bits() {
        let mut cpu = CPU::new();
        cpu.pin.a = 0b11010110;
        assert_eq!(cpu.rd_a_bit(0).unwrap(), false);
        assert_eq!(cpu.rd_a_bit(4).unwrap(), true);
        assert!(cpu.rd_a_bit(13).is_err());
    }

    #[test]
    fn drive_data_bus() {
        let mut cpu = CPU::new();
        assert!(cpu.drv_d(0x67).is_ok());
        assert!(cpu.drv_d(0x678).is_err());
    }

    #[test]
    fn read_data_bus() {
        let mut cpu = CPU::new();
        cpu.drv_d(0x67).unwrap();
        assert_eq!(cpu.rd_d(), 0x67);
    }

    #[test]
    fn read_data_bus_bits() {
        let mut cpu = CPU::new();
        cpu.drv_d(0b11010110).unwrap();
        assert_eq!(cpu.rd_d_bit(0).unwrap(), false);
        assert_eq!(cpu.rd_d_bit(4).unwrap(), true);
        assert!(cpu.rd_d_bit(8).is_err());
    }

    #[test]
    fn drive_data_bus_bits() {
        let mut cpu = CPU::new();
        cpu.drv_d(0b11010110).unwrap();
        cpu.drv_d_bit(0, false).unwrap();
        assert_eq!(cpu.rd_d(), 0b11010110);
        cpu.drv_d_bit(1, false).unwrap();
        assert_eq!(cpu.rd_d(), 0b11010100);
        cpu.drv_d_bit(2, true).unwrap();
        assert_eq!(cpu.rd_d(), 0b11010100);
        cpu.drv_d_bit(3, true).unwrap();
        assert_eq!(cpu.rd_d(), 0b11011100);
        assert!(cpu.drv_d_bit(8, true).is_err());
    }

    #[test]
    fn drive_data_bus_wrapped() {
        let mut cpu = CPU::new();
        cpu.drv_d_wrap(0x567);
        assert_eq!(cpu.rd_d(), 0x67);
        cpu.drv_d_wrap(0x89);
        assert_eq!(cpu.rd_d(), 0x89);
    }

    #[test]
    fn read_rw_pin() {
        let mut cpu = CPU::new();
        cpu.pin.rw = true;
        assert_eq!(cpu.rd_rw(), true);
        cpu.pin.rw = false;
        assert_eq!(cpu.rd_rw(), false);
    }

    #[test]
    fn drive_phi0_pin() {
        let mut cpu = CPU::new();
        cpu.drv_phi0(true);
        assert_eq!(cpu.pin.phi0, true);
        cpu.drv_phi0(false);
        assert_eq!(cpu.pin.phi0, false);
    }

    #[test]
    fn read_phi2_pin() {
        let mut cpu = CPU::new();
        cpu.pin.phi2 = true;
        assert_eq!(cpu.rd_phi2(), true);
        cpu.pin.phi2 = false;
        assert_eq!(cpu.rd_phi2(), false);
    }
}
