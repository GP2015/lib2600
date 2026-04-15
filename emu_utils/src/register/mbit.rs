use crate::{
    bit,
    register::{BitRegister, RegisterError},
};

#[derive(Clone)]
pub struct MBitRegister {
    name: String,
    bits: Box<[BitRegister]>,
}

impl MBitRegister {
    fn check_bit_in_range(&self, bit: usize) -> Result<(), RegisterError> {
        if bit >= self.size() {
            Err(RegisterError::BitOutOfRange {
                name: self.name.clone(),
                bit,
                size: self.size(),
            })
        } else {
            Ok(())
        }
    }

    fn check_write_val_valid(&self, val: usize) -> Result<(), RegisterError> {
        if bit::usize_exceeds_bit_count(val, self.size()) {
            Err(RegisterError::WriteValueTooLarge {
                name: self.name.clone(),
                value: val,
                size: self.size(),
            })
        } else {
            Ok(())
        }
    }

    #[must_use]
    pub fn new(size: usize, name: String) -> Self {
        Self {
            bits: (0..size)
                .map(|bit| BitRegister::new(format!("{name} bit {bit}")))
                .collect(),
            name,
        }
    }

    #[must_use]
    pub fn size(&self) -> usize {
        self.bits.len()
    }

    #[must_use]
    pub fn state(&self) -> Vec<Option<bool>> {
        self.bits.iter().map(BitRegister::state).collect()
    }

    pub fn bit_state(&self, bit: usize) -> Result<Option<bool>, RegisterError> {
        self.check_bit_in_range(bit)?;
        Ok(self.bits[bit].state())
    }

    pub fn read(&self) -> Result<usize, RegisterError> {
        let mut combined = 0;

        for bit_reg in self.bits.iter().rev() {
            let val = bit_reg.read()?;
            combined <<= 1;
            combined |= usize::from(val);
        }

        Ok(combined)
    }

    pub fn read_bit(&self, bit: usize) -> Result<bool, RegisterError> {
        self.check_bit_in_range(bit)?;
        self.bits[bit].read()
    }

    pub fn write(&mut self, val: usize) -> Result<(), RegisterError> {
        self.check_write_val_valid(val)?;
        for (bit, bit_reg) in self.bits.iter_mut().enumerate() {
            bit_reg.write(bit::get_bit_of_usize(val, bit));
        }
        Ok(())
    }

    pub fn wrapping_write(&mut self, val: usize) {
        for (bit, bit_reg) in self.bits.iter_mut().enumerate() {
            bit_reg.write(bit::get_bit_of_usize(val, bit));
        }
    }

    pub fn undefine(&mut self) {
        for bit_reg in &mut self.bits {
            bit_reg.undefine();
        }
    }

    pub fn write_bit(&mut self, bit: usize, state: bool) -> Result<(), RegisterError> {
        self.check_bit_in_range(bit)?;
        self.bits[bit].write(state);
        Ok(())
    }

    pub fn undefine_bit(&mut self, bit: usize) -> Result<(), RegisterError> {
        self.check_bit_in_range(bit)?;
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
        assert_eq!(reg.read().unwrap(), 0b1111_1111);
    }

    #[rstest]
    fn read_bits(mut reg: MBitRegister) {
        reg.write(0b1101_0110).unwrap();
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
        reg.write(0b1101_0110).unwrap();
        reg.write_bit(0, false).unwrap();
        assert_eq!(reg.read().unwrap(), 0b1101_0110);
        reg.write_bit(1, false).unwrap();
        assert_eq!(reg.read().unwrap(), 0b1101_0100);
        reg.write_bit(2, true).unwrap();
        assert_eq!(reg.read().unwrap(), 0b1101_0100);
        reg.write_bit(3, true).unwrap();
        assert_eq!(reg.read().unwrap(), 0b1101_1100);
    }
}
