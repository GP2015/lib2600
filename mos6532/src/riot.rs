use crate::{
    Bus, RiotError, SinglePin,
    data::{
        pins::{Pins, bus::BusOutput},
        ram::Ram,
        registers::Registers,
    },
};

macro_rules! pin_getter {
    ($name:ident, $obj:ident) => {
        pub fn $name(&mut self) -> &mut impl $obj {
            &mut self.pin.$name
        }
    };
}

pub struct Riot {
    pub pin: Pins,
    pub(crate) reg: Registers,
    pub(crate) ram: Ram,
}

impl Default for Riot {
    fn default() -> Self {
        Self::new()
    }
}

impl Riot {
    pub fn new() -> Self {
        Self {
            pin: Pins::new(),
            reg: Registers::new(),
            ram: Ram::new(),
        }
    }

    pin_getter!(a, Bus);
    pin_getter!(db, Bus);
    pin_getter!(pa, Bus);
    pin_getter!(pb, Bus);
    pin_getter!(res, SinglePin);
    pin_getter!(cs1, SinglePin);
    pin_getter!(cs2, SinglePin);
    pin_getter!(rw, SinglePin);
    pin_getter!(rs, SinglePin);
    pin_getter!(irq, SinglePin);

    pub fn pulse_phi2(&mut self) -> Result<(), RiotError> {
        self.tick()
    }

    pub fn release_db(&mut self) {
        self.pin.db.tristate_out();
    }
}
