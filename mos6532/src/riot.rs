use crate::{Pins, data::registers::Registers};

pub struct Riot {
    pub pin: Pins,
    reg: Registers,
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
        }
    }
}
