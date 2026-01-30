pub mod bus;
pub mod single;
pub mod state;

use crate::data::pins::{
    bus::{address::AddressBus, data::ContentionByteBus},
    single::{SinglePinNew, contention::ContentionPin, input::InputPin},
};

type InputPinType = InputPin;
type OutputPinType = ContentionPin;
type AddressBusType = AddressBus<InputPin>;
type TwoWayBusType = ContentionByteBus<ContentionPin>;

pub struct Pins {
    pub a: AddressBusType,
    pub pa: TwoWayBusType,
    pub pb: TwoWayBusType,
    pub irq: OutputPinType,
    pub db: TwoWayBusType,
    pub res: InputPinType,
    pub rw: InputPinType,
    pub rs: InputPinType,
    pub cs2: InputPinType,
    pub cs1: InputPinType,
}

impl Pins {
    pub(crate) fn new() -> Self {
        Self {
            a: AddressBusType::new(),
            db: TwoWayBusType::new(String::from("DB")),
            pa: TwoWayBusType::new(String::from("PA")),
            pb: TwoWayBusType::new(String::from("PB")),
            cs1: InputPinType::new(String::from("CS1")),
            cs2: InputPinType::new(String::from("/CS2")),
            rw: InputPinType::new(String::from("R/W")),
            res: InputPinType::new(String::from("/RES")),
            rs: InputPinType::new(String::from("/RS")),
            irq: OutputPinType::new(String::from("/IRQ")),
        }
    }
}
