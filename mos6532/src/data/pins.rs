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
    pub db: TwoWayBusType,
    pub pa: TwoWayBusType,
    pub pb: TwoWayBusType,
    pub res: InputPinType,
    pub cs1: InputPinType,
    pub cs2: InputPinType,
    pub rs: InputPinType,
    pub rw: InputPinType,
    pub irq: OutputPinType,
}

impl Pins {
    pub(crate) fn new() -> Self {
        Self {
            a: AddressBusType::new(),
            db: TwoWayBusType::new(String::from("DB")),
            pa: TwoWayBusType::new(String::from("PA")),
            pb: TwoWayBusType::new(String::from("PB")),
            res: InputPinType::new(String::from("/RES")),
            cs1: InputPinType::new(String::from("CS1")),
            cs2: InputPinType::new(String::from("/CS2")),
            rw: InputPinType::new(String::from("R/W")),
            rs: InputPinType::new(String::from("/RS")),
            irq: OutputPinType::new(String::from("/IRQ")),
        }
    }
}
