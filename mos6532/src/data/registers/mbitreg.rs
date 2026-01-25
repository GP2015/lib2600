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

    pub fn read(&self) -> Result<usize, RiotError> {
        let mut combined = 0;

        for bit in (0..self.size).rev() {
            let Some(val) = self.bits[bit] else {
                return Err(RiotError::RegisterBitUninitialised {
                    name: self.name.clone(),
                    bit,
                });
            };

            combined <<= 1;
            combined |= val as usize;
        }

        Ok(combined)
    }

    pub fn read_bit(&self, bit: usize) -> Result<bool, RiotError> {
        let Some(val) = self.bits[bit] else {
            return Err(RiotError::RegisterBitUninitialised {
                name: self.name.clone(),
                bit,
            });
        };

        Ok(val)
    }

    pub fn write(&mut self, val: usize) {
        if cfg!(debug_assertions) && Self::usize_exceeds_bit_count(val, self.size) {
            panic!("writing excessively large value to register should not be possible");
        }

        for bit in 0..self.size {
            self.bits[bit] = Some(Self::get_bit_of_usize(val, bit))
        }
    }

    pub fn write_bit(&mut self, bit: usize, state: bool) -> Result<(), RiotError> {
        self.bits[bit] = Some(state);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::{fixture, rstest};

    #[rstest]
    #[case(0x101, 0, true)]
    #[case(0x101, 1, false)]
    #[case(0x101, 7, false)]
    fn get_bit_of_usize(#[case] val: usize, #[case] bit: usize, #[case] res: bool) {
        assert_eq!(MBitReg::get_bit_of_usize(val, bit), res);
    }

    #[rstest]
    #[case(0b1011, 3, true)]
    #[case(0b1011, 4, false)]
    #[case(0b1011, 5, false)]
    fn usize_exceeds_bit_count(#[case] val: usize, #[case] bit_count: usize, #[case] res: bool) {
        assert_eq!(MBitReg::usize_exceeds_bit_count(val, bit_count), res);
    }

    #[fixture]
    fn reg() -> MBitReg {
        MBitReg::new(8, String::new())
    }

    #[rstest]
    fn read(mut reg: MBitReg) {
        reg.write(0x67);
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
        reg.write(0b11010110);
        assert!(!reg.read_bit(0).unwrap());
        assert!(reg.read_bit(1).unwrap());
        assert!(!reg.read_bit(3).unwrap());
        assert!(reg.read_bit(4).unwrap());
    }

    #[rstest]
    fn read_uninitialised_bits(mut reg: MBitReg) {
        assert!(reg.read_bit(6).is_err());
        reg.write_bit(6, true).unwrap();
        assert!(reg.read_bit(6).unwrap());
    }

    #[rstest]
    fn write_bits(mut reg: MBitReg) {
        reg.write(0b11010110);
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
