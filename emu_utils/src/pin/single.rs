pub mod contention;
pub mod core;
pub mod input;

#[cfg(test)]
pub mod mock_pin;

use crate::pin::{PinError, PinState, callback::CallbackFn};
use delegate::delegate;

pub trait SinglePinSetup<O> {
    fn new(name: String) -> Self;
    fn assign_callback(&mut self, callback: Box<CallbackFn<O>>);
}

pub trait SinglePinInput {
    fn state(&self) -> PinState;
    fn state_as_bool(&self) -> Option<bool>;
    fn read(&self) -> Result<bool, PinError>;
    fn signal_in(&mut self, state: PinState) -> Result<(), PinError>;
    fn drive_in(&mut self, state: bool) -> Result<(), PinError>;
    fn tri_state_in(&mut self) -> Result<(), PinError>;
    fn undefine_in(&mut self) -> Result<(), PinError>;
}

pub trait SinglePinOutput {
    fn signal_out(&mut self, state: PinState) -> Result<(), PinError>;
    fn drive_out(&mut self, state: bool) -> Result<(), PinError>;
    fn tri_state_out(&mut self) -> Result<(), PinError>;
    fn undefine_out(&mut self) -> Result<(), PinError>;
}

struct SinglePin<'a, O, E> {
    pin: &'a mut dyn SinglePinInput,
    obj: &'a mut O,
    err_type: std::marker::PhantomData<E>,
}

impl<'a, O, E: From<PinError>> SinglePin<'a, O, E> {
    delegate! {
        to self.pin{
            fn state(&self) -> PinState;
            fn state_as_bool(&self) -> Option<bool>;
            fn read(&self) -> Result<bool, PinError>;
            fn signal_in(&mut self, state: PinState) -> Result<(), PinError>;
            fn drive_in(&mut self, state: bool) -> Result<(), PinError>;
            fn tri_state_in(&mut self) -> Result<(), PinError>;
            fn undefine_in(&mut self) -> Result<(), PinError>;
        }
    }
}
