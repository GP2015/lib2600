pub mod common;
pub mod cpin;
pub mod ipin;

use cpin::ContentionPin;
use ipin::InputPin;

pub struct Pins {
    // pub a: MBitReg,
    // pub pa: MBitReg,
    // pub pb: MBitReg,
    pub irq: ContentionPin,
    // pub db: MBitReg,
    pub res: InputPin,
    pub rw: InputPin,
    pub rs: InputPin,
    pub cs2: InputPin,
    pub cs1: InputPin,
}

impl Default for Pins {
    fn default() -> Self {
        Self::new()
    }
}

impl Pins {
    pub fn new() -> Self {
        Self {
            irq: ContentionPin::new(),
            res: InputPin::new(),
            rw: InputPin::new(),
            rs: InputPin::new(),
            cs2: InputPin::new(),
            cs1: InputPin::new(),
        }
    }
}
