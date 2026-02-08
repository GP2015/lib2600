use crate::{
    bit,
    register::{BitRegister, RegisterError},
};

#[derive(Clone)]
pub struct MBitRegister {
    name: String,
    size: usize,
    bits: Vec<BitRegister>,
}

impl MBitRegister {
    pub fn new(size: usize, name: String) -> Self {
        Self {
            size,
            bits: (0..size)
                .map(|bit| BitRegister::new(format!("{} bit {}", name, bit)))
                .collect(),
            name,
        }
    }

    pub fn state(&self) -> Vec<Option<bool>> {
        self.bits.iter().map(|bit| bit.state()).collect()
    }

    pub fn bit_state(&self, bit: usize) -> Result<Option<bool>, RegisterError> {
        if bit >= self.size {
            return Err(RegisterError::BitOutOfRange {
                name: self.name.clone(),
                bit,
                size: self.size,
            });
        }

        Ok(self.bits[bit].state())
    }

    pub fn read(&self) -> Result<usize, RegisterError> {
        let mut combined = 0;

        for bit in (0..self.size).rev() {
            let val = self.bits[bit].read()?;
            combined <<= 1;
            combined |= val as usize;
        }

        Ok(combined)
    }

    pub fn read_bit(&self, bit: usize) -> Result<bool, RegisterError> {
        if bit >= self.size {
            return Err(RegisterError::BitOutOfRange {
                name: self.name.clone(),
                bit,
                size: self.size,
            });
        }

        self.bits[bit].read()
    }

    pub fn write(&mut self, val: usize) -> Result<(), RegisterError> {
        if bit::usize_exceeds_bit_count(val, self.size) {
            return Err(RegisterError::WriteValueTooLarge {
                name: self.name.clone(),
                value: val,
                size: self.size,
            });
        }

        for bit in 0..self.size {
            self.bits[bit].write(bit::get_bit_of_usize(val, bit));
        }

        Ok(())
    }

    pub fn wrapping_write(&mut self, val: usize) {
        self.write(bit::get_low_bits_of_usize(val, self.size))
            .expect("writing only the low bits should not error");
    }

    pub fn undefine(&mut self) {
        for bit in 0..self.size {
            self.bits[bit].undefine();
        }
    }

    pub fn write_bit(&mut self, bit: usize, state: bool) -> Result<(), RegisterError> {
        if bit >= self.size {
            return Err(RegisterError::BitOutOfRange {
                name: self.name.clone(),
                bit,
                size: self.size,
            });
        }

        self.bits[bit].write(state);
        Ok(())
    }

    pub fn undefine_bit(&mut self, bit: usize) -> Result<(), RegisterError> {
        if bit >= self.size {
            return Err(RegisterError::BitOutOfRange {
                name: self.name.clone(),
                bit,
                size: self.size,
            });
        }

        self.bits[bit].undefine();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::{fixture, rstest};

    #[fixture]
    fn reg() -> MBitRegister {
        MBitRegister::new(8, String::new())
    }

    #[rstest]
    fn read(mut reg: MBitRegister) {
        reg.write(0x67).unwrap();
        assert_eq!(reg.read().unwrap(), 0x67);
    }

    #[rstest]
    fn read_uninitialised(mut reg: MBitRegister) {
        for i in 0..7 {
            reg.write_bit(i, true).unwrap();
        }
        assert!(reg.read().is_err());

        reg.write_bit(7, true).unwrap();
        assert_eq!(reg.read().unwrap(), 0b11111111);
    }

    #[rstest]
    fn read_bits(mut reg: MBitRegister) {
        reg.write(0b11010110).unwrap();
        assert!(!reg.read_bit(0).unwrap());
        assert!(reg.read_bit(1).unwrap());
        assert!(!reg.read_bit(3).unwrap());
        assert!(reg.read_bit(4).unwrap());
    }

    #[rstest]
    fn read_uninitialised_bits(mut reg: MBitRegister) {
        assert!(reg.read_bit(6).is_err());
        reg.write_bit(6, true).unwrap();
        assert!(reg.read_bit(6).unwrap());
    }

    #[rstest]
    fn write_bits(mut reg: MBitRegister) {
        reg.write(0b11010110).unwrap();
        reg.write_bit(0, false).unwrap();
        assert_eq!(reg.read().unwrap(), 0b11010110);
        reg.write_bit(1, false).unwrap();
        assert_eq!(reg.read().unwrap(), 0b11010100);
        reg.write_bit(2, true).unwrap();
        assert_eq!(reg.read().unwrap(), 0b11010100);
        reg.write_bit(3, true).unwrap();
        assert_eq!(reg.read().unwrap(), 0b11011100);
    }
}
