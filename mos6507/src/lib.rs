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

pub struct CPU {
    res: bool,
    rdy: bool,
    a: usize,
    d: usize,
    rw: bool,
    phi0: bool,
    phi2: bool,
}

impl CPU {
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

    pub fn drv_res(&mut self, state: bool) {
        self.res = state;
    }

    pub fn drv_rdy(&mut self, state: bool) {
        self.rdy = state;
    }

    pub fn rd_a(&self) -> usize {
        self.a
    }

    pub fn rd_a_bit(&self, bit: usize) -> Result<bool, CPUError> {
        if bit >= A_SIZE {
            return Err(CPUError::InvalidBit(bit));
        }

        Ok(get_bit_of_usize(self.a, bit))
    }

    pub fn rd_d(&self) -> usize {
        self.d
    }

    pub fn rd_d_bit(&self, bit: usize) -> Result<bool, CPUError> {
        if bit >= D_SIZE {
            return Err(CPUError::InvalidBit(bit));
        }

        Ok(get_bit_of_usize(self.d, bit))
    }

    pub fn drv_d(&mut self, val: usize) -> Result<(), CPUError> {
        if val_exceeds_bit_count(val, D_SIZE) {
            return Err(CPUError::ValueTooLarge(val));
        }

        self.d = val;
        Ok(())
    }

    pub fn drv_d_wrap(&mut self, val: usize) {
        self.d = get_low_bits_of_usize(val, D_SIZE);
    }

    pub fn drv_d_bit(&mut self, bit: usize, state: bool) -> Result<(), CPUError> {
        if bit >= D_SIZE {
            return Err(CPUError::InvalidBit(bit));
        }

        self.d = set_bit_of_usize(self.d, bit, state);
        Ok(())
    }

    pub fn rd_rw(&self) -> bool {
        self.rw
    }

    pub fn drv_phi0(&mut self, state: bool) {
        self.phi0 = state;
    }

    pub fn rd_phi2(&self) -> bool {
        self.phi2
    }
}
