use crate::error::RIOTError;

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

pub struct Bus {
    val: usize,
    size: usize,
}

impl Bus {
    pub fn new(val: usize, size: usize) -> Self {
        Self { val, size }
    }

    pub fn read(&self) -> usize {
        self.val
    }

    pub fn read_bit(&self, bit: usize) -> Result<bool, RIOTError> {
        if bit >= self.size {
            return Err(RIOTError::InvalidBit(bit));
        }

        Ok(get_bit_of_usize(self.val, bit))
    }

    pub fn drive(&mut self, val: usize) -> Result<(), RIOTError> {
        if val_exceeds_bit_count(val, self.size) {
            return Err(RIOTError::ValueTooLarge(val));
        }

        self.val = val;
        Ok(())
    }

    pub fn drive_wrap(&mut self, val: usize) {
        self.val = get_low_bits_of_usize(val, self.size);
    }

    pub fn drive_bit(&mut self, bit: usize, state: bool) -> Result<(), RIOTError> {
        if bit >= self.size {
            return Err(RIOTError::InvalidBit(bit));
        }

        self.val = set_bit_of_usize(self.val, bit, state);
        Ok(())
    }
}
