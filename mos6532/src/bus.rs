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
    pub fn new(size: usize) -> Self {
        Self { val: 0, size }
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
    fn drive_bus() {
        let mut bus = Bus::new(8);
        assert!(bus.drive(0x67).is_ok());
        assert!(bus.drive(0x678).is_err());
    }

    #[test]
    fn read_bus() {
        let mut bus = Bus::new(8);
        bus.drive(0x67).unwrap();
        assert_eq!(bus.read(), 0x67);
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
    fn drive_bus_bits() {
        let mut bus = Bus::new(8);
        bus.drive(0b11010110).unwrap();
        bus.drive_bit(0, false).unwrap();
        assert_eq!(bus.read(), 0b11010110);
        bus.drive_bit(1, false).unwrap();
        assert_eq!(bus.read(), 0b11010100);
        bus.drive_bit(2, true).unwrap();
        assert_eq!(bus.read(), 0b11010100);
        bus.drive_bit(3, true).unwrap();
        assert_eq!(bus.read(), 0b11011100);
        assert!(bus.drive_bit(8, true).is_err());
    }

    #[test]
    fn drive_bus_wrapped() {
        let mut bus = Bus::new(8);
        bus.drive_wrap(0x567);
        assert_eq!(bus.read(), 0x67);
        bus.drive_wrap(0x89);
        assert_eq!(bus.read(), 0x89);
    }
}
