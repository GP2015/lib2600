pub mod contention;
pub mod input;
#[cfg(test)]
pub mod mock_pin;

use delegate::delegate;

use crate::pin::{PinError, PinSignal};
use std::marker::PhantomData;

pub trait SinglePinCore {
    fn new(name: String) -> Self;
    fn post_tick_update(&mut self);
    fn name(&self) -> &str;
    fn iter_possible_signals(&self) -> impl Iterator<Item = PinSignal>;
    fn iter_prev_possible_signals(&self) -> impl Iterator<Item = PinSignal>;
    fn possible_signals(&self) -> Vec<PinSignal>;
    fn prev_possible_signals(&self) -> Vec<PinSignal>;
    fn collapsed(&self) -> Option<PinSignal>;
    fn prev_collapsed(&self) -> Option<PinSignal>;
    fn set_signal_in(&mut self, signal: PinSignal, possible: bool) -> Result<(), PinError>;
    fn set_drive_in(&mut self, bool_signal: bool, possible: bool) -> Result<(), PinError>;
    fn set_tri_state_in(&mut self, possible: bool);
    fn set_all_signals_in(&mut self, possible: bool) -> Result<(), PinError>;
    fn set_possible_in_to_prev(&mut self) -> Result<(), PinError>;
}

pub trait SinglePinOutput {
    fn set_signal_out(&mut self, signal: PinSignal, possible: bool) -> Result<(), PinError>;
    fn set_drive_out(&mut self, bool_signal: bool, possible: bool) -> Result<(), PinError>;
    fn set_tri_state_out(&mut self, possible: bool);
    fn set_all_signals_out(&mut self, possible: bool) -> Result<(), PinError>;
    fn set_possible_out_to_prev(&mut self) -> Result<(), PinError>;
}

trait RefType {}
struct Immutable;
struct Mutable;
impl RefType for Immutable {}
impl RefType for Mutable {}

enum SinglePinRef<'a, P> {
    Immutable(&'a P),
    Mutable(&'a mut P),
}

impl<'a, P> SinglePinRef<'a, P> {
    fn as_ref(&'a self) -> &'a P {
        match self {
            SinglePinRef::Immutable(pin) => pin,
            SinglePinRef::Mutable(pin) => pin,
        }
    }

    fn as_mut(&'a mut self) -> Option<&'a mut P> {
        if let SinglePinRef::Mutable(pin) = self {
            Some(pin)
        } else {
            None
        }
    }
}

pub struct SinglePinInterface<'a, E, M, P> {
    inner: SinglePinRef<'a, P>,
    err_type: PhantomData<E>,
    ref_type: PhantomData<M>,
}

impl<'a, E, P> SinglePinInterface<'a, E, Immutable, P>
where
    E: From<PinError>,
    P: SinglePinCore,
{
    pub fn from_ref(pin: &'a P) -> Self {
        Self {
            inner: SinglePinRef::Immutable(pin),
            err_type: PhantomData,
            ref_type: PhantomData,
        }
    }
}

impl<'a, E, P> SinglePinInterface<'a, E, Mutable, P>
where
    E: From<PinError>,
    P: SinglePinCore,
{
    pub fn from_mut(pin: &'a mut P) -> Self {
        Self {
            inner: SinglePinRef::Mutable(pin),
            err_type: PhantomData,
            ref_type: PhantomData,
        }
    }
}

impl<'a, E, M, P> SinglePinInterface<'a, E, M, P>
where
    E: From<PinError>,
    P: SinglePinCore,
    M: RefType,
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

impl<'a, E, P> SinglePinInterface<'a, E, Mutable, P>
where
    E: From<PinError>,
    P: SinglePinCore,
{
    delegate! {
        to self.inner.as_mut().unwrap(){
            pub fn set_tri_state_in(&mut self, possible: bool);
        }

        #[expr($.map_err(E::from))]
        to self.inner.as_mut().unwrap(){
            pub fn set_signal_in(&mut self, signal: PinSignal, possible: bool) -> Result<(), E>;
            pub fn set_drive_in(&mut self, bool_signal: bool, possible: bool) -> Result<(), E>;
            pub fn set_all_signals_in(&mut self, possible: bool) -> Result<(), E>;
            pub fn set_possible_in_to_prev(&mut self) -> Result<(), E>;
        }
    }
}
