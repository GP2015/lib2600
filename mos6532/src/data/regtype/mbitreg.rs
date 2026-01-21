use crate::error::RiotError;

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
        val & ((1 << bit_count) - 1)
    }

    pub fn read(&self) -> Result<usize, RiotError> {
        let mut combined = 0;

        for bit in (0..self.size).rev() {
            let Some(val) = self.bits[bit] else {
                return Err(RiotError::UninitialisedMBitRegBit {
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

    pub fn read_bit(&self, bit: usize) -> Result<bool, RiotError> {
        if bit >= self.size {
            return Err(RiotError::MBitRegBitOutOfRange {
                reg_name: self.name.clone(),
                bit,
                reg_size: self.size,
            });
        }

        let Some(val) = self.bits[bit] else {
            return Err(RiotError::UninitialisedMBitRegBit {
                reg_name: self.name.clone(),
                bit,
                reg_size: self.size,
            });
        };

        Ok(val)
    }

    pub fn write(&mut self, val: usize) -> Result<(), RiotError> {
        if Self::usize_exceeds_bit_count(val, self.size) {
            return Err(RiotError::MBitRegDriveValueTooLarge {
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

    pub fn write_wrap(&mut self, val: usize) {
        self.write(Self::get_low_bits_of_usize(val, self.size))
            .unwrap();
    }

    pub fn write_bit(&mut self, bit: usize, state: bool) -> Result<(), RiotError> {
        if bit >= self.size {
            return Err(RiotError::MBitRegBitOutOfRange {
                reg_name: self.name.clone(),
                bit,
                reg_size: self.size,
            });
        }

        self.bits[bit] = Some(state);
        Ok(())
    }

    pub fn is_written(&self) -> bool {
        for bit in 0..self.size {
            if self.bits[bit].is_none() {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::{fixture, rstest};

    #[test]
    fn get_bit_of_usize() {
        assert!(MBitReg::get_bit_of_usize(0b101, 0));
        assert!(!MBitReg::get_bit_of_usize(0b101, 1));
        assert!(!MBitReg::get_bit_of_usize(0b101, 7));
    }

    #[test]
    fn usize_exceeds_bit_count() {
        assert!(MBitReg::usize_exceeds_bit_count(0b1011, 3));
        assert!(!MBitReg::usize_exceeds_bit_count(0b1011, 4));
        assert!(!MBitReg::usize_exceeds_bit_count(0b1011, 5));
    }

    #[test]
    fn get_low_bits_of_usize() {
        assert_eq!(MBitReg::get_low_bits_of_usize(0b1011, 0), 0);
        assert_eq!(MBitReg::get_low_bits_of_usize(0b1011, 1), 1);
        assert_eq!(MBitReg::get_low_bits_of_usize(0b1011, 2), 0b11);
        assert_eq!(MBitReg::get_low_bits_of_usize(0b1011, 3), 0b11);
        assert_eq!(MBitReg::get_low_bits_of_usize(0b1011, 7), 0b1011);
    }

    #[fixture]
    fn reg() -> MBitReg {
        MBitReg::new(8, String::new())
    }

    #[rstest]
    fn read(mut reg: MBitReg) {
        reg.write(0x67).unwrap();
        assert_eq!(reg.read().unwrap(), 0x67);
    }

    #[rstest]
    fn read_uninitialised(mut reg: MBitReg) {
        for i in 0..7 {
            reg.write_bit(i, true).unwrap();
        }
        assert!(reg.read().is_err());

        reg.write_bit(7, true).unwrap();
        assert_eq!(reg.read().unwrap(), 0b11111111);
    }

    #[rstest]
    fn read_bits(mut reg: MBitReg) {
        reg.write(0b11010110).unwrap();
        assert!(!reg.read_bit(0).unwrap());
        assert!(reg.read_bit(4).unwrap());
        assert!(reg.read_bit(8).is_err());
    }

    #[rstest]
    fn read_uninitialised_bits(mut reg: MBitReg) {
        assert!(reg.read_bit(6).is_err());
        reg.write_bit(6, true).unwrap();
        assert!(reg.read_bit(6).unwrap());
    }

    #[rstest]
    fn write(mut reg: MBitReg) {
        assert!(reg.write(0x67).is_ok());
        assert!(reg.write(0x678).is_err());
    }

    #[rstest]
    fn write_wrapped(mut reg: MBitReg) {
        reg.write_wrap(0x567);
        assert_eq!(reg.read().unwrap(), 0x67);
        reg.write_wrap(0x89);
        assert_eq!(reg.read().unwrap(), 0x89);
    }

    #[rstest]
    fn write_bits(mut reg: MBitReg) {
        reg.write(0b11010110).unwrap();
        reg.write_bit(0, false).unwrap();
        assert_eq!(reg.read().unwrap(), 0b11010110);
        reg.write_bit(1, false).unwrap();
        assert_eq!(reg.read().unwrap(), 0b11010100);
        reg.write_bit(2, true).unwrap();
        assert_eq!(reg.read().unwrap(), 0b11010100);
        reg.write_bit(3, true).unwrap();
        assert_eq!(reg.read().unwrap(), 0b11011100);
        assert!(reg.write_bit(8, true).is_err());
    }

    #[rstest]
    fn is_written(mut reg: MBitReg) {
        assert!(!reg.is_written());

        for i in 0..7 {
            reg.write_bit(i, true).unwrap();
        }
        assert!(!reg.is_written());

        reg.write_bit(7, true).unwrap();
        assert!(reg.is_written());
    }
}
