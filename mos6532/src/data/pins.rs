use emu_utils::pin::{BusCore, ContentionPin, InputPin, SinglePinCore, StandardBus};

type InputPinType = InputPin;
type OutputPinType = ContentionPin;
type AddressBusType = StandardBus<InputPin>;
type DataBusType = StandardBus<ContentionPin>;

pub struct Pins {
    pub a: AddressBusType,
    pub db: DataBusType,
    pub pa: DataBusType,
    pub pb: DataBusType,
    pub res: InputPinType,
    pub phi2: InputPinType,
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
            phi2: InputPinType::new(String::from("PHI2")),
            cs1: InputPinType::new(String::from("CS1")),
            cs2: InputPinType::new(String::from("/CS2")),
            rw: InputPinType::new(String::from("R/W")),
            rs: InputPinType::new(String::from("/RS")),
            irq: OutputPinType::new(String::from("/IRQ")),
        }
    }
}
