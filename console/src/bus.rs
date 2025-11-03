use anyhow::{Result, anyhow};

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

    pub fn get_line(&self, line: usize) -> Result<bool> {
        if line >= self.number_of_lines {
            return Err(anyhow!(
                "Cannot access non-existent bit {line} of {}-size bus.",
                self.number_of_lines
            ));
        }

        Ok((self.combined_value >> line) & 1 == 1)
    }

    // Returns true if the input value overflowed to fit the bus.
    pub fn set_combined(&mut self, combined_value: usize) -> bool {
        self.combined_value = combined_value % self.value_range_size;
        return combined_value >= self.value_range_size;
    }

    pub fn set_line(&mut self, line: usize, value: bool) -> Result<()> {
        if line >= self.number_of_lines {
            return Err(anyhow!(
                "Cannot write to non-existent line {line} of {}-sized bus.",
                self.number_of_lines
            ));
        }

        self.combined_value = match value {
            true => self.combined_value | (1 << line),
            false => self.combined_value & !(1 << line),
        };

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_bus_combined_value() {
        let mut bus = Bus::new(8);
        bus.set_combined(0x67);
        assert_eq!(bus.get_combined(), 0x67);
    }

    #[test]
    fn test_get_bus_line_value() {
        let mut bus = Bus::new(4);
        bus.set_combined(0b0110);
        assert_eq!(bus.get_line(2).unwrap(), true);
        assert_eq!(bus.get_line(3).unwrap(), false);
    }

    #[test]
    fn test_set_bus_line_value() {
        let mut bus = Bus::new(4);
        bus.set_combined(0b1100);
        bus.set_line(0, false).unwrap();
        bus.set_line(1, true).unwrap();
        bus.set_line(2, false).unwrap();
        bus.set_line(3, true).unwrap();
        assert_eq!(bus.get_combined(), 0b1010);
    }
}
