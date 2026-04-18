mod control;
mod error;
mod helpers;
mod pins;
mod ram;
mod registers;

pub use emutils::pin::{BusInputUI, PinError, PinInputUI, PinSignal};
pub use error::RiotError;

use crate::{pins::Pins, ram::Ram, registers::Registers};
use emutils::pin::{BusOutput, PinOutput};
use paste::paste;

macro_rules! create_pin_input {
    ($name:ident) => {
        pub fn $name(&self) -> &impl PinInputUI {
            &self.pin.$name
        }

        paste! {
            pub fn [<$name _mut>](&mut self) -> &mut impl PinInputUI {
                &mut self.pin.$name
            }
        }
    };
}

macro_rules! create_bus_input {
    ($name:ident) => {
        pub fn $name(&self) -> &impl BusInputUI {
            &self.pin.$name
        }

        paste! {
            pub fn [<$name _mut>](&mut self) -> &mut impl BusInputUI {
                &mut self.pin.$name
            }
        }
    };
}

pub struct Riot {
    pin: Pins,
    reg: Registers,
    ram: Ram,
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

    pub fn tick(&mut self) -> Result<(), RiotError> {
        let instructions = self.possible_instructions();
        self.execute_possible_instructions(instructions)
    }

    pub fn release_db(&mut self) {
        self.pin
            .db
            .iter_out_mut()
            .for_each(|pin| pin.add_high_z_out(true));
    }

    create_bus_input!(a);
    create_bus_input!(db);
    create_bus_input!(pa);
    create_bus_input!(pb);
    create_pin_input!(res);
    create_pin_input!(phi2);
    create_pin_input!(cs1);
    create_pin_input!(cs2);
    create_pin_input!(rw);
    create_pin_input!(rs);
    create_pin_input!(irq);
}
