use crate::{
    bit,
    pin::BusInputUI,
    reg::{BitRegister, RegisterError},
};
use delegate::delegate;
use itertools::Itertools;

#[derive(Clone)]
pub struct MBitRegister {
    name: String,
    bits: Box<[BitRegister]>,
}

impl MBitRegister {
    fn check_bit_in_range(&self, bit: usize) -> Result<(), RegisterError> {
        if bit >= self.size() {
            return Err(RegisterError::BitOutOfRange {
                name: self.name.clone(),
                bit,
                size: self.size(),
            });
        }
        Ok(())
    }

    fn check_write_val_valid(&self, val: usize) -> Result<(), RegisterError> {
        if bit::usize_exceeds_bit_count(val, self.size()) {
            return Err(RegisterError::WriteValueTooLarge {
                name: self.name.clone(),
                value: val,
                size: self.size(),
            });
        }
        Ok(())
    }

    #[must_use]
    pub fn new<S: Into<String>>(name: S, size: usize) -> Self {
        let name = name.into();
        Self {
            bits: (0..size)
                .map(|bit| BitRegister::new(format!("{name} bit {bit}")))
                .collect(),
            name,
        }
    }

    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn bit(&self, bit: usize) -> Result<&BitRegister, RegisterError> {
        self.check_bit_in_range(bit)?;
        Ok(&self.bits[bit])
    }

    pub fn bit_mut(&mut self, bit: usize) -> Result<&mut BitRegister, RegisterError> {
        self.check_bit_in_range(bit)?;
        Ok(&mut self.bits[bit])
    }

    #[must_use]
    pub fn read(&self) -> Option<usize> {
        bit::some_bits_to_usize(self.bits.iter().map(BitRegister::collapsed))
    }

    pub fn iter_possible_reads(&self) -> impl Iterator<Item = usize> {
        self.bits
            .iter()
            .map(BitRegister::possible_reads)
            .multi_cartesian_product()
            .map(|bits| bit::bits_to_usize(bits.into_iter().copied()))
    }

    pub fn add_wrapping(&mut self, val: usize, only_possible: bool) {
        for (bit, reg) in self.bits.iter_mut().enumerate() {
            reg.add(bit::bit_of_usize(val, bit), only_possible);
        }
    }

    pub fn add(&mut self, val: usize, only_possible: bool) -> Result<(), RegisterError> {
        self.check_write_val_valid(val)?;
        self.add_wrapping(val, only_possible);
        Ok(())
    }

    pub fn input_from_bus(&mut self, bus: &impl BusInputUI, only_possible: bool) {
        for (reg, pin) in self.iter_mut().zip(bus.iter()) {
            reg.input_from_pin(pin, only_possible);
        }
    }

    delegate! {
        to self.bits {
            #[must_use]
            #[call(len)]
            pub fn size(&self) -> usize;
            pub fn iter(&self) -> impl Iterator<Item = &BitRegister>;
            pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut BitRegister>;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::{fixture, rstest};

    const REG_NAME: &str = "reg";
    const REG_SIZE: usize = 4;

    #[fixture]
    fn reg() -> MBitRegister {
        MBitRegister::new(REG_NAME, REG_SIZE)
    }

    #[rstest]
    fn new_correct_size(reg: MBitRegister) {
        assert_eq!(reg.size(), REG_SIZE);
    }

    #[rstest]
    fn new_correct_names(reg: MBitRegister) {
        for bit in 0..REG_SIZE {
            assert_eq!(
                reg.bit(bit).unwrap().name(),
                format!("{REG_NAME} bit {bit}")
            );
        }
    }

    #[rstest]
    fn name(reg: MBitRegister) {
        assert_eq!(reg.name(), REG_NAME);
    }

    #[rstest]
    fn valid_bit(mut reg: MBitRegister) {
        for bit in 0..REG_SIZE {
            assert!(reg.bit(bit).is_ok());
            assert!(reg.bit_mut(bit).is_ok());
        }
    }

    #[rstest]
    fn invalid_bit(mut reg: MBitRegister, #[values(REG_SIZE, REG_SIZE + 1)] bit: usize) {
        let e = RegisterError::BitOutOfRange {
            name: reg.name().to_string(),
            bit,
            size: REG_SIZE,
        };

        assert_eq!(reg.bit(bit).err().unwrap(), e);
        assert_eq!(reg.bit_mut(bit).err().unwrap(), e);
    }

    #[rstest]
    #[case([false, false, false, false], 0)]
    #[case([false, true, false, false], 0b10)]
    #[case([true, true, false, true], 0b1011)]
    fn read_success(mut reg: MBitRegister, #[case] bits: [bool; REG_SIZE], #[case] val: usize) {
        for (bit_reg, &b) in reg.iter_mut().zip(bits.iter()) {
            bit_reg.add(b, true);
        }
        assert_eq!(reg.read(), Some(val));
    }

    #[rstest]
    fn read_failure(mut reg: MBitRegister, #[values(true, false)] initial: bool) {
        reg.iter_mut().for_each(|r| r.add(initial, true));
        reg.bit_mut(2).unwrap().add(!initial, false);
        assert_eq!(reg.read(), None);
    }

    // Iter possible reads
    // Iter possible reads
    // Iter possible reads
    // Iter possible reads
    // Iter possible reads

    #[rstest]
    #[case(0, [false, false, false, false])]
    #[case(0b1011, [true, true, false, true])]
    #[case(0b11011, [true, true, false, true])]
    fn add_wrapping_only_possible(
        mut reg: MBitRegister,
        #[values(true, false)] initial: bool,
        #[case] val: usize,
        #[case] bits: [bool; REG_SIZE],
    ) {
        reg.iter_mut().for_each(|r| r.add(initial, true));
        reg.add_wrapping(val, true);
        for (bit_reg, &b) in reg.iter().zip(bits.iter()) {
            assert_eq!(bit_reg.collapsed(), Some(b));
        }
    }

    #[rstest]
    #[case(0, [false, false, false, false])]
    #[case(0b1011, [true, true, false, true])]
    #[case(0b11011, [true, true, false, true])]
    fn add_wrapping_not_only_possible(
        mut reg: MBitRegister,
        #[values(true, false)] initial: bool,
        #[case] val: usize,
        #[case] bits: [bool; REG_SIZE],
    ) {
        reg.iter_mut().for_each(|r| r.add(initial, true));
        reg.add_wrapping(val, false);
        for (bit_reg, &b) in reg.iter().zip(bits.iter()) {
            assert!(bit_reg.is_possible(initial));
            assert!(bit_reg.is_possible(b));
        }
    }
}
