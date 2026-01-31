use crate::{
    Bus, RiotError, SinglePin,
    data::{pins::Pins, ram::Ram, registers::Registers},
};
use emu_utils::pin::{BusOutput, SinglePinOutput};
use paste::paste;

macro_rules! create_pin_input {
    ($name:ident, $obj:ident) => {
        pub fn $name(&mut self) -> &mut impl $obj<Error = RiotError> {
            &mut self.pin.$name
        }
    };
}

macro_rules! create_pin_output {
    ($pin:ident, $obj:ident) => {
        paste! {
            pub(crate) fn [<$pin _o>](&mut self) -> &mut impl $obj<Error = RiotError> {
                &mut self.pin.$pin
            }
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

    pub fn release_db(&mut self) {
        self.db_o().tri_state_out();
    }

    create_pin_input!(a, Bus);
    create_pin_input!(db, Bus);
    create_pin_input!(pa, Bus);
    create_pin_input!(pb, Bus);
    create_pin_input!(res, SinglePin);
    create_pin_input!(cs1, SinglePin);
    create_pin_input!(cs2, SinglePin);
    create_pin_input!(rw, SinglePin);
    create_pin_input!(rs, SinglePin);
    create_pin_input!(irq, SinglePin);

    create_pin_output!(db, BusOutput);
    create_pin_output!(pa, BusOutput);
    create_pin_output!(pb, BusOutput);
    create_pin_output!(irq, SinglePinOutput);
}
