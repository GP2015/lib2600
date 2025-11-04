pub struct Bus {
    combined_value: usize,
    number_of_lines: usize,
    value_range_size: usize,
}

impl Bus {
    pub fn new(size: usize) -> Self {
        Self {
            combined_value: 0,
            number_of_lines: size,
            value_range_size: usize::pow(2, size as u32),
        }
    }

    pub fn get_combined(&self) -> usize {
        self.combined_value
    }

    pub fn get_line(&self, line: usize) -> bool {
        if line >= self.number_of_lines {
            panic!(
                "Cannot access non-existent bit {line} of {}-size bus.",
                self.number_of_lines
            );
        }

        (self.combined_value >> line) & 1 == 1
    }

    // Returns true if the input value overflowed to fit the bus.
    pub fn set_combined(&mut self, combined_value: usize) -> bool {
        self.combined_value = combined_value % self.value_range_size;
        return combined_value >= self.value_range_size;
    }

    pub fn set_line(&mut self, line: usize, value: bool) {
        if line >= self.number_of_lines {
            panic!(
                "Cannot write to non-existent line {line} of {}-sized bus.",
                self.number_of_lines
            );
        }

        self.combined_value = match value {
            true => self.combined_value | (1 << line),
            false => self.combined_value & !(1 << line),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_combined_value() {
        let mut bus = Bus::new(8);
        bus.set_combined(0x67);
        assert_eq!(bus.get_combined(), 0x67);
    }

    #[test]
    fn get_line_value() {
        let mut bus = Bus::new(4);
        bus.set_combined(0b0110);
        assert_eq!(bus.get_line(2), true);
        assert_eq!(bus.get_line(3), false);
    }

    #[test]
    fn set_line_value() {
        let mut bus = Bus::new(4);
        bus.set_combined(0b1100);
        bus.set_line(0, false);
        bus.set_line(1, true);
        bus.set_line(2, false);
        bus.set_line(3, true);
        assert_eq!(bus.get_combined(), 0b1010);
    }
}
