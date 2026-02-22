pub mod contention;
pub mod input;
#[cfg(test)]
pub mod mock_pin;

use delegate::delegate;

use crate::pin::{PinError, PinSignal, obj_ref::ObjRef};
use std::marker::PhantomData;

pub trait SinglePinCore {
    fn new(name: String) -> Self;
    fn post_tick_update(&mut self);

    fn interface<E>(&self) -> SinglePinInterface<'_, E, Self, false>
    where
        Self: Sized;

    fn interface_mut<E>(&mut self) -> SinglePinInterface<'_, E, Self, true>
    where
        Self: Sized;

    fn name(&self) -> &str;
    fn iter_possible_signals(&self) -> impl Iterator<Item = PinSignal>;
    fn iter_prev_possible_signals(&self) -> impl Iterator<Item = PinSignal>;
    fn possible_signals(&self) -> Vec<PinSignal>;
    fn prev_possible_signals(&self) -> Vec<PinSignal>;
    fn collapsed(&self) -> Option<PinSignal>;
    fn prev_collapsed(&self) -> Option<PinSignal>;
    fn set_signal_in(&mut self, signal: PinSignal, possible: bool) -> Result<(), PinError>;
    fn set_possible_in_to_prev(&mut self) -> Result<(), PinError>;

    fn set_drive_in(&mut self, bool_signal: bool, possible: bool) -> Result<(), PinError> {
        self.set_signal_in(PinSignal::from_bool(bool_signal), possible)
    }

    fn set_tri_state_in(&mut self, possible: bool) {
        self.set_signal_in(PinSignal::TriState, possible).unwrap();
    }

    fn set_all_signals_in(&mut self, possible: bool) -> Result<(), PinError> {
        for signal in [PinSignal::High, PinSignal::Low, PinSignal::TriState] {
            self.set_signal_in(signal, possible)?;
        }
        Ok(())
    }

    fn add_signal_in(&mut self, signal: PinSignal) -> Result<(), PinError> {
        self.set_signal_in(signal, true)
    }

    fn add_drive_in(&mut self, bool_signal: bool) -> Result<(), PinError> {
        self.set_drive_in(bool_signal, true)
    }

    fn add_tri_state_in(&mut self) {
        self.set_tri_state_in(true);
    }

    fn add_all_signals_in(&mut self) -> Result<(), PinError> {
        self.set_all_signals_in(true)
    }
}

pub trait SinglePinOutput {
    fn set_signal_out(&mut self, signal: PinSignal, possible: bool) -> Result<(), PinError>;
    fn set_possible_out_to_prev(&mut self) -> Result<(), PinError>;

    fn set_drive_out(&mut self, bool_signal: bool, possible: bool) -> Result<(), PinError> {
        self.set_signal_out(PinSignal::from_bool(bool_signal), possible)
    }

    fn set_tri_state_out(&mut self, possible: bool) {
        self.set_signal_out(PinSignal::TriState, possible).unwrap();
    }

    fn set_all_signals_out(&mut self, possible: bool) -> Result<(), PinError> {
        for signal in [PinSignal::High, PinSignal::Low, PinSignal::TriState] {
            self.set_signal_out(signal, possible)?;
        }
        Ok(())
    }

    fn add_signal_out(&mut self, signal: PinSignal) -> Result<(), PinError> {
        self.set_signal_out(signal, true)
    }

    fn add_drive_out(&mut self, bool_signal: bool) -> Result<(), PinError> {
        self.set_drive_out(bool_signal, true)
    }

    fn add_tri_state_out(&mut self) {
        self.set_tri_state_out(true);
    }

    fn add_all_signals_out(&mut self) -> Result<(), PinError> {
        self.set_all_signals_out(true)
    }
}

pub struct SinglePinInterface<'a, E, P, const M: bool> {
    inner: ObjRef<'a, P>,
    err_type: PhantomData<E>,
}

impl<'a, E, P> SinglePinInterface<'a, E, P, false> {
    pub fn from_ref(pin: &'a P) -> Self {
        Self {
            inner: ObjRef::Immutable(pin),
            err_type: PhantomData,
        }
    }
}

impl<'a, E, P> SinglePinInterface<'a, E, P, true> {
    pub fn from_mut(pin: &'a mut P) -> Self {
        Self {
            inner: ObjRef::Mutable(pin),
            err_type: PhantomData,
        }
    }
}

impl<'a, E, P, const M: bool> SinglePinInterface<'a, E, P, M>
where
    P: SinglePinCore,
{
    delegate! {
        to self.inner.as_ref(){
            pub fn name(&self) -> &str;
            pub fn iter_possible_signals(&self) -> impl Iterator<Item = PinSignal>;
            pub fn iter_prev_possible_signals(&self) -> impl Iterator<Item = PinSignal>;
            pub fn possible_signals(&self) -> Vec<PinSignal>;
            pub fn prev_possible_signals(&self) -> Vec<PinSignal>;
            pub fn collapsed(&self) -> Option<PinSignal>;
            pub fn prev_collapsed(&self) -> Option<PinSignal>;
        }
    }
}

impl<'a, E, P> SinglePinInterface<'a, E, P, true>
where
    E: From<PinError>,
    P: SinglePinCore,
{
    delegate! {
        #[expr($.map_err(E::from))]
        to self.inner.as_mut().unwrap(){
            pub fn set_signal_in(&mut self, signal: PinSignal, possible: bool) -> Result<(), E>;
            pub fn set_possible_in_to_prev(&mut self) -> Result<(), E>;

            pub fn set_drive_in(&mut self, bool_signal: bool, possible: bool) -> Result<(), E>;
            pub fn set_all_signals_in(&mut self, possible: bool) -> Result<(), E>;
            pub fn add_signal_in(&mut self, signal: PinSignal) -> Result<(), E>;
            pub fn add_drive_in(&mut self, bool_signal: bool) -> Result<(), E>;
            pub fn add_all_signals_in(&mut self) -> Result<(), E>;
        }

        to self.inner.as_mut().unwrap(){
            pub fn set_tri_state_in(&mut self, possible: bool);
            pub fn add_tri_state_in(&mut self);
        }
    }
}
