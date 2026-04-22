#![warn(clippy::pedantic)]
#![cfg_attr(not(test), warn(clippy::unwrap_used))]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::struct_excessive_bools)]
#![allow(clippy::cast_possible_truncation)]

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

macro_rules! pin_inputs {
    ($($name:ident),* $(,)?) => {
        $(
            pub fn $name(&self) -> &impl PinInputUI {
                &self.pin.$name
            }

            paste! {
                pub fn [<$name _mut>](&mut self) -> &mut impl PinInputUI {
                    &mut self.pin.$name
                }
            }
        )*
    };
}

macro_rules! bus_inputs {
    ($($name:ident),* $(,)?) => {
        $(
            pub fn $name(&self) -> &impl BusInputUI {
                &self.pin.$name
            }

            paste! {
                pub fn [<$name _mut>](&mut self) -> &mut impl BusInputUI {
                    &mut self.pin.$name
                }
            }
        )*
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
    #[must_use]
    pub fn new() -> Self {
        Self {
            pin: Pins::new(),
            reg: Registers::new(),
            ram: Ram::new(),
        }
    }

    pub fn tick(&mut self) -> Result<(), RiotError> {
        let instructions = self.possible_instructions();
        self.execute_possible_instructions(&instructions)
    }

    pub fn release_db(&mut self) {
        self.pin
            .db
            .iter_out_mut()
            .for_each(|pin| pin.add_high_z_out(true));
    }

    bus_inputs!(a, db, pa, pb);
    pin_inputs!(res, phi2, cs1, cs2, rw, rs, irq);
}
