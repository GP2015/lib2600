use crate::{
    RiotError,
    data::{pins::Pins, ram::Ram, registers::Registers},
};
use emu_utils::pin::{BusInterface, BusOutput, SinglePinInterface, SinglePinOutput};
use paste::paste;

macro_rules! create_pin_input {
    ($name:ident, $obj:ident) => {
        pub fn $name(&self) -> &impl $obj<RiotError> {
            &self.pin.$name
        }

        paste! {
            pub fn [<$name _mut>](&mut self) -> &mut impl $obj<RiotError> {
                &mut self.pin.$name
            }
        }
    };
}

macro_rules! create_pin_output {
    ($pin:ident, $obj:ident) => {
        paste! {
            pub(crate) fn [<$pin _out>](&mut self) -> &mut impl $obj<RiotError> {
                &mut self.pin.$pin
            }
        }
    };
}

pub struct Riot {
    pub(crate) pin: Pins,
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
        self.db_out().tri_state_out();
    }

    create_pin_input!(a, BusInterface);
    create_pin_input!(db, BusInterface);
    create_pin_input!(pa, BusInterface);
    create_pin_input!(pb, BusInterface);
    create_pin_input!(res, SinglePinInterface);
    create_pin_input!(phi2, SinglePinInterface);
    create_pin_input!(cs1, SinglePinInterface);
    create_pin_input!(cs2, SinglePinInterface);
    create_pin_input!(rw, SinglePinInterface);
    create_pin_input!(rs, SinglePinInterface);
    create_pin_input!(irq, SinglePinInterface);

    create_pin_output!(db, BusOutput);
    create_pin_output!(pa, BusOutput);
    create_pin_output!(pb, BusOutput);
    create_pin_output!(irq, SinglePinOutput);
}
