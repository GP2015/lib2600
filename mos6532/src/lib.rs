#![warn(clippy::pedantic)]
#![cfg_attr(not(test), warn(clippy::unwrap_used))]
#![cfg_attr(not(test), warn(clippy::expect_used))]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::struct_excessive_bools)]
#![allow(clippy::cast_possible_truncation)]

mod control;
mod error;
mod pins;
mod ram;
mod registers;

pub use error::RiotError;

use crate::{
    pins::{PinConnections, RiotLineInitRefs, RiotLineRefs},
    ram::Ram,
    registers::Registers,
};

pub struct Riot {
    connections: PinConnections,
    reg: Registers,
    ram: Ram,
}

impl Riot {
    #[must_use]
    pub fn new(inits: &mut RiotLineInitRefs) -> Self {
        Self {
            connections: PinConnections::new(inits),
            reg: Registers::new(),
            ram: Ram::new(),
        }
    }

    pub fn tick(&mut self, lines: &mut RiotLineRefs) -> Result<(), RiotError> {
        let instructions = control::possible_instructions(lines)?;
        self.execute_possible_instructions(lines, &instructions)
    }
}
