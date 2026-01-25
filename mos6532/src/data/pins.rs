use crate::data::pins::{
    abus::AddressBus, cbytebus::ContentionByteBus, cpin::ContentionPin, ipin::InputPin,
};

pub mod abus;
pub mod cbytebus;
pub mod common;
pub mod cpin;
pub mod ipin;

pub struct Pins {
    pub a: AddressBus,
    pub pa: ContentionByteBus,
    pub pb: ContentionByteBus,
    pub irq: ContentionPin,
    pub db: ContentionByteBus,
    pub res: InputPin,
    pub rw: InputPin,
    pub rs: InputPin,
    pub cs2: InputPin,
    pub cs1: InputPin,
}

impl Pins {
    pub(crate) fn new() -> Self {
        Self {
            a: AddressBus::new(),
            db: ContentionByteBus::new(String::from("DB")),
            pa: ContentionByteBus::new(String::from("PA")),
            pb: ContentionByteBus::new(String::from("PB")),
            cs1: InputPin::new(String::from("CS1")),
            cs2: InputPin::new(String::from("/CS2")),
            rw: InputPin::new(String::from("R/W")),
            res: InputPin::new(String::from("/RES")),
            rs: InputPin::new(String::from("/RS")),
            irq: ContentionPin::new(String::from("/IRQ")),
        }
    }
}
