use super::*;
use anyhow::{Result, anyhow};

const ROM_SIZE: usize = 4096;
const CHIP_ENABLE_LINE: usize = 12;

pub struct Mapper4K {
    rom: Vec<u8>,
}

impl UseAsMapper for Mapper4K {
    fn new(program: Vec<u8>) -> Result<Self> {
        if program.len() > ROM_SIZE {
            return Err(anyhow!("Program supplied is too large."));
        }

        Ok(Self { rom: program })
    }

    fn tick(&mut self, address_bus: &mut Bus, data_bus: &mut Bus) {
        if address_bus.get_line(CHIP_ENABLE_LINE).unwrap() {
            let addr = address_bus.get_combined();
            let data = self.rom[addr % ROM_SIZE];
            data_bus.set_combined(data as usize);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_objects(program: Vec<u8>) -> (Bus, Bus, Mapper4K) {
        let address_bus = Bus::new(13);
        let data_bus = Bus::new(8);
        let cart = Mapper4K::new(program).unwrap();
        (address_bus, data_bus, cart)
    }

    #[test]
    fn test_m4k_no_read() {
        let mut program = vec![0; ROM_SIZE];
        program[0x67] = 0x89;

        let (mut address_bus, mut data_bus, mut cart) = create_test_objects(program);

        address_bus.set_combined(0x0067);
        cart.tick(&mut address_bus, &mut data_bus);

        assert_eq!(data_bus.get_combined(), 0x00);
    }

    #[test]
    fn test_m4k_read() {
        let mut program = vec![0; ROM_SIZE];
        program[0x67] = 0x89;

        let (mut address_bus, mut data_bus, mut cart) = create_test_objects(program);

        address_bus.set_combined(0x1067);
        cart.tick(&mut address_bus, &mut data_bus);

        assert_eq!(data_bus.get_combined(), 0x89);
    }
}
