mod bitreg;
mod common;
mod mbitreg;

use crate::data::registers::mbitreg::MBitReg;

pub struct Registers {
    ddra: MBitReg,
    ddrb: MBitReg,
    ora: MBitReg,
    orb: MBitReg,
}

impl Registers {
    pub fn new() -> Self {
        Self {
            ddra: MBitReg::new(8),
            ddrb: MBitReg::new(8),
            ora: MBitReg::new(8),
            orb: MBitReg::new(8),
        }
    }
}
