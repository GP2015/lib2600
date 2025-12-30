use crate::error::RIOTError;

pub struct MBitReg {
    name: String,
    bits: Vec<Option<bool>>,
    size: usize,
}

impl MBitReg {
    pub fn new(size: usize, name: String) -> Self {
        Self {
            name,
            bits: vec![None; size],
            size,
        }
    }

    fn get_bit_of_usize(val: usize, bit: usize) -> bool {
        (val >> bit) & 1 == 1
    }

    fn usize_exceeds_bit_count(val: usize, bit_count: usize) -> bool {
        val >> bit_count != 0
    }

    fn get_low_bits_of_usize(val: usize, bit_count: usize) -> usize {
        val & (1 << bit_count) - 1
    }

    pub fn read(&self) -> Result<usize, RIOTError> {
        let mut combined = 0;

        for bit in (0..self.size).rev() {
            let Some(val) = self.bits[bit] else {
                return Err(RIOTError::UninitialisedMBitRegBit {
                    reg_name: self.name.clone(),
                    bit,
                    reg_size: self.size,
                });
            };

            combined <<= 1;
            combined |= val as usize;
        }

        Ok(combined)
    }

    pub fn read_bit(&self, bit: usize) -> Result<bool, RIOTError> {
        if bit >= self.size {
            return Err(RIOTError::MBitRegBitOutOfRange {
                reg_name: self.name.clone(),
                bit,
                reg_size: self.size,
            });
        }

        let Some(val) = self.bits[bit] else {
            return Err(RIOTError::UninitialisedMBitRegBit {
                reg_name: self.name.clone(),
                bit,
                reg_size: self.size,
            });
        };

        Ok(val)
    }

    pub fn drive(&mut self, val: usize) -> Result<(), RIOTError> {
        if Self::usize_exceeds_bit_count(val, self.size) {
            return Err(RIOTError::MBitRegDriveValueTooLarge {
                reg_name: self.name.clone(),
                value: val,
                reg_size: self.size,
            });
        }

        for bit in 0..self.size {
            self.bits[bit] = Some(Self::get_bit_of_usize(val, bit))
        }

        Ok(())
    }

    pub fn drive_wrap(&mut self, val: usize) {
        self.drive(Self::get_low_bits_of_usize(val, self.size))
            .unwrap();
    }

    pub fn drive_bit(&mut self, bit: usize, state: bool) -> Result<(), RIOTError> {
        if bit >= self.size {
            return Err(RIOTError::MBitRegBitOutOfRange {
                reg_name: self.name.clone(),
                bit,
                reg_size: self.size,
            });
        }

        self.bits[bit] = Some(state);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_bit_of_usize() {
        assert_eq!(MBitReg::get_bit_of_usize(0b101, 0), true);
        assert_eq!(MBitReg::get_bit_of_usize(0b101, 1), false);
        assert_eq!(MBitReg::get_bit_of_usize(0b101, 7), false);
    }

    #[test]
    fn usize_exceeds_bit_count() {
        assert_eq!(MBitReg::usize_exceeds_bit_count(0b1011, 3), true);
        assert_eq!(MBitReg::usize_exceeds_bit_count(0b1011, 4), false);
        assert_eq!(MBitReg::usize_exceeds_bit_count(0b1011, 5), false);
    }

    #[test]
    fn get_low_bits_of_usize() {
        assert_eq!(MBitReg::get_low_bits_of_usize(0b1011, 0), 0);
        assert_eq!(MBitReg::get_low_bits_of_usize(0b1011, 1), 1);
        assert_eq!(MBitReg::get_low_bits_of_usize(0b1011, 2), 0b11);
        assert_eq!(MBitReg::get_low_bits_of_usize(0b1011, 3), 0b11);
        assert_eq!(MBitReg::get_low_bits_of_usize(0b1011, 7), 0b1011);
    }

    #[test]
    fn read() {
        let mut val = MBitReg::new(8, String::new());
        val.drive(0x67).unwrap();
        assert_eq!(val.read().unwrap(), 0x67);
    }

    #[test]
    fn read_uninitialised() {
        let mut val = MBitReg::new(8, String::new());

        for i in 0..7 {
            val.drive_bit(i, true).unwrap();
        }
        assert!(val.read().is_err());

        val.drive_bit(7, true).unwrap();
        assert_eq!(val.read().unwrap(), 0b11111111);
    }

    #[test]
    fn read_bits() {
        let mut val = MBitReg::new(8, String::new());
        val.drive(0b11010110).unwrap();
        assert_eq!(val.read_bit(0).unwrap(), false);
        assert_eq!(val.read_bit(4).unwrap(), true);
        assert!(val.read_bit(8).is_err());
    }

    #[test]
    fn read_uninitialised_bits() {
        let mut val = MBitReg::new(8, String::new());
        assert!(val.read_bit(6).is_err());
        val.drive_bit(6, true).unwrap();
        assert_eq!(val.read_bit(6).unwrap(), true);
    }

    #[test]
    fn drive() {
        let mut val = MBitReg::new(8, String::new());
        assert!(val.drive(0x67).is_ok());
        assert!(val.drive(0x678).is_err());
    }

    #[test]
    fn drive_wrapped() {
        let mut val = MBitReg::new(8, String::new());
        val.drive_wrap(0x567);
        assert_eq!(val.read().unwrap(), 0x67);
        val.drive_wrap(0x89);
        assert_eq!(val.read().unwrap(), 0x89);
    }

    #[test]
    fn drive_bits() {
        let mut val = MBitReg::new(8, String::new());
        val.drive(0b11010110).unwrap();
        val.drive_bit(0, false).unwrap();
        assert_eq!(val.read().unwrap(), 0b11010110);
        val.drive_bit(1, false).unwrap();
        assert_eq!(val.read().unwrap(), 0b11010100);
        val.drive_bit(2, true).unwrap();
        assert_eq!(val.read().unwrap(), 0b11010100);
        val.drive_bit(3, true).unwrap();
        assert_eq!(val.read().unwrap(), 0b11011100);
        assert!(val.drive_bit(8, true).is_err());
    }
}
