use crate::data::{pins::Pins, ram::Ram, registers::Registers};
use emu_utils::pin::{BusCore, SinglePinOutput};

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
        self.db_out()
            .for_each_pin_mut(|pin| pin.set_tri_state_out(true));
    }
}
