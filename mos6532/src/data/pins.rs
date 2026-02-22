use crate::{Riot, RiotError};
use emu_utils::pin::{
    BusCore, BusInterface, ContentionPin, InputPin, SinglePinCore, SinglePinInterface, StandardBus,
};
use paste::paste;

type InputPinType = InputPin;
type OutputPinType = ContentionPin;
type AddressBusPinType = InputPin;
type DataBusPinType = ContentionPin;
type AddressBusType = StandardBus<AddressBusPinType>;
type DataBusType = StandardBus<DataBusPinType>;

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

macro_rules! create_pin_input {
    ($fn_name:ident, $pin_type:ident) => {
        pub fn $fn_name(&self) -> SinglePinInterface<'_, RiotError, $pin_type, false> {
            self.pin.$fn_name.interface()
        }

        paste! {
            pub fn [<$fn_name _mut>](&mut self) -> SinglePinInterface<'_, RiotError, $pin_type, true> {
                self.pin.$fn_name.interface_mut()
            }
        }
    };
}

macro_rules! create_bus_input {
    ($fn_name:ident, $bus_type:ident, $pin_type:ident) => {
        pub fn $fn_name(&self) -> BusInterface<'_, $bus_type, RiotError, $pin_type, false> {
            self.pin.$fn_name.interface()
        }

        paste! {
            pub fn [<$fn_name _mut>](&mut self) -> BusInterface<'_, $bus_type, RiotError, $pin_type, true> {
                self.pin.$fn_name.interface_mut()
            }
        }
    };
}

macro_rules! create_pin_output {
    ($fn_name:ident, $pin_type:ident) => {
        paste! {
            pub(crate) fn [<$fn_name _out>](&mut self) -> &mut $pin_type {
                &mut self.pin.$fn_name
            }
        }
    };
}

macro_rules! create_bus_output {
    ($fn_name:ident, $bus_type:ident) => {
        paste! {
            pub(crate) fn [<$fn_name _out>](&mut self) -> &mut $bus_type {
                &mut self.pin.$fn_name
            }
        }
    };
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

impl Riot {
    create_bus_input!(a, AddressBusType, AddressBusPinType);
    create_bus_input!(db, DataBusType, DataBusPinType);
    create_bus_input!(pa, DataBusType, DataBusPinType);
    create_bus_input!(pb, DataBusType, DataBusPinType);
    create_pin_input!(res, InputPinType);
    create_pin_input!(phi2, InputPinType);
    create_pin_input!(cs1, InputPinType);
    create_pin_input!(cs2, InputPinType);
    create_pin_input!(rw, InputPinType);
    create_pin_input!(rs, InputPinType);
    create_pin_input!(irq, OutputPinType);

    create_bus_output!(db, DataBusType);
    create_bus_output!(pa, DataBusType);
    create_bus_output!(pb, DataBusType);
    create_pin_output!(irq, OutputPinType);
}
