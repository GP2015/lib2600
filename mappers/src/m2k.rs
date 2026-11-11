use crate::{Mapper, MapperDrive};
use emucore::{BitStringState, BusOutput, Combine, LineSignal};

#[derive(Debug)]
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

        MapperDrive {
            a: a_drv,
            db: db_drv,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::array;
    use rstest::{fixture, rstest};

    #[fixture]
    fn mapper() -> M2K {
        #[allow(clippy::cast_possible_truncation)]
        M2K::new(array::from_fn(|bit| bit as u8))
    }

    // #[rstest]
    // fn no_read(mut mapper: M2K) {
    //     let a = 0.into();
    //     let drv = mapper.tick(a, BitStringState::default());
    //     assert_eq!(drv.db, [LineSignal::HighZ; _].into());
    // }

    // #[rstest]
    // fn read(mut mapper: M2K) {
    //     let a = 0x1067.into();
    //     let drv = mapper.tick(a, BitStringState::default());
    //     let val = drv.db.read().unwrap().value().unwrap();
    //     assert_eq!(val, 0x67);
    // }

    // #[rstest]
    // fn read_wrapped(mut mapper: M2K) {
    //     let a = 0x1867.into();
    //     let drv = mapper.tick(a, BitStringState::default());
    //     let val = drv.db.read().unwrap().value().unwrap();
    //     assert_eq!(val, 0x67);
    // }
}
