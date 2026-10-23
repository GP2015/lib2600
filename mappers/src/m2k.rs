use crate::{Mapper, MapperDrive};
use emucore::{BitStringState, BusOutput, Combine, LineSignal};

pub struct M2K {
    rom: [u8; 2048],
}

impl M2K {
    #[must_use]
    pub const fn new(rom: [u8; 2048]) -> Self {
        Self { rom }
    }
}

impl Mapper for M2K {
    fn tick(&mut self, a: BitStringState<13>, _: BitStringState<8>) -> MapperDrive {
        let a_drv = [LineSignal::HighZ; _].into();

        let db_drv = Combine::mux(
            a[12].as_cond(),
            &|| BusOutput::from([LineSignal::HighZ; _]),
            &|| {
                BusOutput::from(
                    a.section::<11>(0)
                        .iter_possible_uints()
                        .map(|addr| u16::from(self.rom[usize::from(addr)]).into())
                        .reduce(|acc: BitStringState<_>, byte| acc.combine_with(&byte))
                        .expect("BitState will always have at least one possible read"),
                )
            },
        );

        MapperDrive { a_drv, db_drv }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     fn create_valid_objects(program: Vec<u8>) -> (Bus, Bus, Mapper2K) {
//         let address_bus = Bus::new(13);
//         let data_bus = Bus::new(8);
//         let cart = Mapper2K::new(program).unwrap();
//         (address_bus, data_bus, cart)
//     }

//     #[test]
//     fn invalid_program() {
//         let program = vec![0; 5];
//         let Err(_) = Mapper2K::new(program) else {
//             panic!("Provided program is invalid, so cartridge creation should have failed.");
//         };
//     }

//     #[test]
//     fn no_read() {
//         let mut program = vec![0; ROM_SIZE];
//         program[0x67] = 0x89;

//         let (mut address_bus, mut data_bus, mut cart) = create_valid_objects(program);

//         address_bus.set_combined(0x0067);
//         cart.tick(&mut address_bus, &mut data_bus);

//         assert_eq!(data_bus.get_combined(), 0x00);
//     }

//     #[test]
//     fn read() {
//         let mut program = vec![0; ROM_SIZE];
//         program[0x67] = 0x89;

//         let (mut address_bus, mut data_bus, mut cart) = create_valid_objects(program);

//         address_bus.set_combined(0x1067);
//         cart.tick(&mut address_bus, &mut data_bus);

//         assert_eq!(data_bus.get_combined(), 0x89);
//     }

//     #[test]
//     fn read_wrapped() {
//         let mut program = vec![0; ROM_SIZE];
//         program[0x67] = 0x89;

//         let (mut address_bus, mut data_bus, mut cart) = create_valid_objects(program);

//         address_bus.set_combined(0x1867);
//         cart.tick(&mut address_bus, &mut data_bus);

//         assert_eq!(data_bus.get_combined(), 0x89);
//     }
// }
