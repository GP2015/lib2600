use crate::data::{
    pins::{AddressBusType, DataBusType, InputPinType, OutputPinType, Pins},
    ram::Ram,
    registers::Registers,
};
use emu_utils::pin::{
    BusCore, BusMut, BusRef, SinglePinCore, SinglePinMut, SinglePinOutput, SinglePinRef,
};
use paste::paste;

macro_rules! create_pin_input {
    ($fn_name:ident, $pin_type:ident) => {
        pub fn $fn_name(&self) -> SinglePinRef<'_, $pin_type> {
            self.pin.$fn_name.interface()
        }

        paste! {
            pub fn [<$fn_name _mut>](&mut self) -> SinglePinMut<'_, $pin_type> {
                self.pin.$fn_name.interface_mut()
            }
        }
    };
}

macro_rules! create_bus_input {
    ($fn_name:ident, $bus_type:ident, $pin_type:ident) => {
        pub fn $fn_name(&self) -> BusRef<'_, $bus_type, $pin_type> {
            self.pin.$fn_name.interface()
        }

        paste! {
            pub fn [<$fn_name _mut>](&mut self) -> BusMut<'_, $bus_type, $pin_type> {
                self.pin.$fn_name.interface_mut()
            }
        }
    };
}

pub struct Riot {
    pub(crate) pin: Pins,
    pub(crate) reg: Registers,
    pub(crate) ram: Ram,
}

impl Default for Riot {
    fn default() -> Self {
        Self::new()
    }
}

impl Riot {
    pub fn new() -> Self {
        Self {
            pin: Pins::new(),
            reg: Registers::new(),
            ram: Ram::new(),
        }
    }

    pub fn release_db(&mut self) {
        self.pin
            .db
            .iter_mut()
            .for_each(OutputPinType::add_high_z_out);
    }

    create_bus_input!(a, AddressBusType, InputPinType);
    create_bus_input!(db, DataBusType, OutputPinType);
    create_bus_input!(pa, DataBusType, OutputPinType);
    create_bus_input!(pb, DataBusType, OutputPinType);
    create_pin_input!(res, InputPinType);
    create_pin_input!(phi2, InputPinType);
    create_pin_input!(cs1, InputPinType);
    create_pin_input!(cs2, InputPinType);
    create_pin_input!(rw, InputPinType);
    create_pin_input!(rs, InputPinType);
    create_pin_input!(irq, OutputPinType);
}
