use crate::error::RIOTError;

fn get_bit_of_usize(val: usize, bit: usize) -> bool {
    (val >> bit) & 1 == 1
}

fn val_exceeds_bit_count(val: usize, bit_count: usize) -> bool {
    val >> bit_count != 0
}

fn get_low_bits_of_usize(val: usize, bit_count: usize) -> usize {
    val & (1 << bit_count) - 1
}

pub struct Bus {
    bits: Vec<Option<bool>>,
    size: usize,
}

impl Bus {
    pub fn new(size: usize) -> Self {
        Self {
            bits: vec![None; size],
            size,
        }
    }

    pub fn read(&self) -> Result<usize, RIOTError> {
        let mut combined = 0;

        for bit in (0..self.size).rev() {
            let Some(val) = self.bits[bit] else {
                return Err(RIOTError::UninitialisedBusBit(bit));
            };

            combined <<= 1;
            combined |= val as usize;
        }

        Ok(combined)
    }

    pub fn read_bit(&self, bit: usize) -> Result<bool, RIOTError> {
        if bit >= self.size {
            return Err(RIOTError::BusBitOutOfRange(bit));
        }

        let Some(val) = self.bits[bit] else {
            return Err(RIOTError::UninitialisedBusBit(bit));
        };

        Ok(val)
    }

    pub fn drive(&mut self, val: usize) -> Result<(), RIOTError> {
        if val_exceeds_bit_count(val, self.size) {
            return Err(RIOTError::BusDriveValueTooLarge(val));
        }

        for bit in 0..self.size {
            self.bits[bit] = Some(get_bit_of_usize(val, bit))
        }

        Ok(())
    }

    pub fn drive_wrap(&mut self, val: usize) {
        self.drive(get_low_bits_of_usize(val, self.size)).unwrap();
    }

    pub fn drive_bit(&mut self, bit: usize, state: bool) -> Result<(), RIOTError> {
        if bit >= self.size {
            return Err(RIOTError::BusBitOutOfRange(bit));
        }

        self.bits[bit] = Some(state);
        Ok(())
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
    fn read_bus() {
        let mut bus = Bus::new(8);
        bus.drive(0x67).unwrap();
        assert_eq!(bus.read().unwrap(), 0x67);
    }

    #[test]
    fn read_uninitialised_bus() {
        let mut bus = Bus::new(8);

        for i in 0..7 {
            bus.drive_bit(i, true).unwrap();
        }
        assert!(bus.read().is_err());

        bus.drive_bit(7, true).unwrap();
        assert_eq!(bus.read().unwrap(), 0b11111111);
    }

    #[test]
    fn read_bus_bits() {
        let mut bus = Bus::new(8);
        bus.drive(0b11010110).unwrap();
        assert_eq!(bus.read_bit(0).unwrap(), false);
        assert_eq!(bus.read_bit(4).unwrap(), true);
        assert!(bus.read_bit(8).is_err());
    }

    #[test]
    fn read_uninitialised_bus_bits() {
        let mut bus = Bus::new(8);
        assert!(bus.read_bit(6).is_err());
        bus.drive_bit(6, true).unwrap();
        assert_eq!(bus.read_bit(6).unwrap(), true);
    }

    #[test]
    fn drive_bus() {
        let mut bus = Bus::new(8);
        assert!(bus.drive(0x67).is_ok());
        assert!(bus.drive(0x678).is_err());
    }

    #[test]
    fn drive_bus_wrapped() {
        let mut bus = Bus::new(8);
        bus.drive_wrap(0x567);
        assert_eq!(bus.read().unwrap(), 0x67);
        bus.drive_wrap(0x89);
        assert_eq!(bus.read().unwrap(), 0x89);
    }

    #[test]
    fn drive_bus_bits() {
        let mut bus = Bus::new(8);
        bus.drive(0b11010110).unwrap();
        bus.drive_bit(0, false).unwrap();
        assert_eq!(bus.read().unwrap(), 0b11010110);
        bus.drive_bit(1, false).unwrap();
        assert_eq!(bus.read().unwrap(), 0b11010100);
        bus.drive_bit(2, true).unwrap();
        assert_eq!(bus.read().unwrap(), 0b11010100);
        bus.drive_bit(3, true).unwrap();
        assert_eq!(bus.read().unwrap(), 0b11011100);
        assert!(bus.drive_bit(8, true).is_err());
    }
}
