use crate::bus::Bus;
use crate::pin::Pin;

pub struct Pins {
    pub a: Bus,
    pub pa: Bus,
    pub pb: Bus,
    pub irq: Pin,
    pub db: Bus,
    pub res: Pin,
    pub rw: Pin,
    pub rs: Pin,
    pub cs2: Pin,
    pub cs1: Pin,
}

impl Pins {
    pub fn new() -> Self {
        Self {
            a: Bus::new(7, String::from("A")),
            db: Bus::new(8, String::from("DB")),
            pa: Bus::new(8, String::from("PA")),
            pb: Bus::new(8, String::from("PB")),
            cs1: Pin::new(String::from("CS1")),
            cs2: Pin::new(String::from("/CS2")),
            rw: Pin::new(String::from("R/W")),
            res: Pin::new(String::from("/RES")),
            rs: Pin::new(String::from("/RS")),
            irq: Pin::new(String::from("/IRQ")),
        }
    }
}

pub struct Registers {
    pub ddra: Option<u8>,
    pub ddrb: Option<u8>,
    pub ora: Option<u8>,
    pub orb: Option<u8>,
}

impl Registers {
    pub fn new() -> Self {
        Self {
            ddra: None,
            ddrb: None,
            ora: None,
            orb: None,
        }
    }
}
