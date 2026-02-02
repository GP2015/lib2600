use emu_utils::pin::{ContentionPin, InputPin, SinglePinNew, StandardBus};

use crate::RiotError;

type InputPinType = InputPin<RiotError>;
type OutputPinType = ContentionPin<RiotError>;
type AddressBusType = StandardBus<InputPin<RiotError>>;
type DataBusType = StandardBus<ContentionPin<RiotError>>;

pub struct Pins {
    pub a: AddressBusType,
    pub db: DataBusType,
    pub pa: DataBusType,
    pub pb: DataBusType,
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
            a: AddressBusType::new(String::from("A"), 7),
            db: DataBusType::new(String::from("DB"), 8),
            pa: DataBusType::new(String::from("PA"), 8),
            pb: DataBusType::new(String::from("PB"), 8),
            res: InputPinType::new(String::from("/RES")),
            cs1: InputPinType::new(String::from("CS1")),
            cs2: InputPinType::new(String::from("/CS2")),
            rw: InputPinType::new(String::from("R/W")),
            rs: InputPinType::new(String::from("/RS")),
            irq: OutputPinType::new(String::from("/IRQ")),
        }
    }
}
