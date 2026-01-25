use crate::{
    RiotError,
    data::{pins::Pins, ram::Ram, registers::Registers},
};

pub struct Riot {
    pub pin: Pins,
    pub(super) reg: Registers,
    pub(super) ram: Ram,
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

    pub fn pulse_phi2(&mut self) -> Result<(), RiotError> {
        self.tick()
    }
}
