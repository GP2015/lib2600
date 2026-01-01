use crate::data::mbitreg::MBitReg;

pub struct Registers {
    pub ddra: MBitReg,
    pub ddrb: MBitReg,
    pub ora: MBitReg,
    pub orb: MBitReg,
}

impl Registers {
    pub fn new() -> Self {
        Self {
            ddra: MBitReg::new(8, String::from("DDRA")),
            ddrb: MBitReg::new(8, String::from("DDRB")),
            ora: MBitReg::new(8, String::from("ORA")),
            orb: MBitReg::new(8, String::from("ORB")),
        }
    }
}
