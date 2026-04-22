use emutils::pin::{BusCore, ContentionPin, InputPin, PinCore, StandardBus};

pub(crate) type InputPinType = InputPin;
pub(crate) type OutputPinType = ContentionPin;
pub(crate) type AddressBusType = StandardBus<InputPinType>;
pub(crate) type DataBusType = StandardBus<OutputPinType>;

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
            a: AddressBusType::new("A", 7),
            db: DataBusType::new("DB", 8),
            pa: DataBusType::new("PA", 8),
            pb: DataBusType::new("PB", 8),
            res: InputPinType::new("/RES"),
            phi2: InputPinType::new("PHI2"),
            cs1: InputPinType::new("CS1"),
            cs2: InputPinType::new("/CS2"),
            rw: InputPinType::new("R/W"),
            rs: InputPinType::new("/RS"),
            irq: OutputPinType::new("/IRQ"),
        }
    }
}
