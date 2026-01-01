use crate::{bitreg::BitReg, mbitreg::MBitReg};

pub struct Buffers {
    pub a: MBitReg,
    pub pa: MBitReg,
    pub pb: MBitReg,
    pub irq: BitReg,
    pub db: MBitReg,
    pub res: BitReg,
    pub rw: BitReg,
    pub rs: BitReg,
    pub cs2: BitReg,
    pub cs1: BitReg,
}

impl Buffers {
    pub fn new() -> Self {
        Self {
            a: MBitReg::new(7, String::from("A")),
            db: MBitReg::new(8, String::from("DB")),
            pa: MBitReg::new(8, String::from("PA")),
            pb: MBitReg::new(8, String::from("PB")),
            cs1: BitReg::new(String::from("CS1")),
            cs2: BitReg::new(String::from("/CS2")),
            rw: BitReg::new(String::from("R/W")),
            res: BitReg::new(String::from("/RES")),
            rs: BitReg::new(String::from("/RS")),
            irq: BitReg::new(String::from("/IRQ")),
        }
    }
}

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
