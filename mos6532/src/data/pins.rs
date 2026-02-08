use emu_utils::pin::{BusSetup, CallbackFn, ContentionPin, InputPin, SinglePinSetup, StandardBus};

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
    pub phi2: InputPinType,
    pub cs1: InputPinType,
    pub cs2: InputPinType,
    pub rs: InputPinType,
    pub rw: InputPinType,
    pub irq: OutputPinType,
}

impl Pins {
    pub(crate) fn new(
        res_callback: Box<CallbackFn<RiotError>>,
        phi2_callback: Box<CallbackFn<RiotError>>,
    ) -> Self {
        let mut pins = Self {
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
        };

        pins.res.assign_callback(res_callback);
        pins.phi2.assign_callback(phi2_callback);

        pins
    }
}
