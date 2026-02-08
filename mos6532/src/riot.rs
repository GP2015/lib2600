use crate::{
    Bus, RiotError, SinglePin,
    data::{pins::Pins, ram::Ram, registers::Registers},
};
use emu_utils::pin::{BusOutput, SinglePinOutput};
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
            pub(crate) fn [<$pin _out>](&self) -> &impl $obj<RiotError> {
                &self.pin.$pin
            }

            pub(crate) fn [<$pin _out_mut>](&mut self) -> &mut impl $obj<RiotError> {
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
            pin: Pins::new(Box::new(Riot::callback_res), Box::new(Riot::callback_phi2)),
            reg: Registers::new(),
            ram: Ram::new(),
        }
    }

    pub fn release_db(&mut self) {
        self.db_out_mut()
            .tri_state_out()
            .expect("tri-stating the data bus shouldn't error");
    }

    create_pin_input!(a, Bus);
    create_pin_input!(db, Bus);
    create_pin_input!(pa, Bus);
    create_pin_input!(pb, Bus);
    create_pin_input!(res, SinglePin);
    create_pin_input!(phi2, SinglePin);
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
