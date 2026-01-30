pub mod bus;
pub mod single;
pub mod state;

use crate::data::pins::{
    bus::{Bus, address::AddressBus, data::ContentionByteBus},
    single::{SinglePin, SinglePinNew, contention::ContentionPin, input::InputPin},
};

type InputPinType = InputPin;
type OutputPinType = ContentionPin;
type AddressBusType = AddressBus<InputPin>;
type TwoWayBusType = ContentionByteBus<ContentionPin>;

macro_rules! pin_getter {
    ($name:ident, $obj:ident) => {
        pub fn $name(&mut self) -> &mut impl $obj {
            &mut self.$name
        }
    };
}

pub struct Pins {
    pub(crate) a: AddressBusType,
    pub(crate) db: TwoWayBusType,
    pub(crate) pa: TwoWayBusType,
    pub(crate) pb: TwoWayBusType,
    pub(crate) res: InputPinType,
    pub(crate) cs1: InputPinType,
    pub(crate) cs2: InputPinType,
    pub(crate) rs: InputPinType,
    pub(crate) rw: InputPinType,
    pub(crate) irq: OutputPinType,
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

    pin_getter!(a, Bus);
    pin_getter!(db, Bus);
    pin_getter!(pa, Bus);
    pin_getter!(pb, Bus);
    pin_getter!(res, SinglePin);
    pin_getter!(cs1, SinglePin);
    pin_getter!(cs2, SinglePin);
    pin_getter!(rw, SinglePin);
    pin_getter!(rs, SinglePin);
    pin_getter!(irq, SinglePin);
}
