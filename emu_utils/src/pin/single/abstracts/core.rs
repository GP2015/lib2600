use crate::pin::{
    PinError, PinSignal,
    single::interfaces::{pinmut::SinglePinMut, pinref::SinglePinRef},
};

pub trait SinglePinCore<'a> {
    fn new(name: String) -> Self;
    fn post_tick_update(&mut self);

    fn interface(&'a self) -> SinglePinRef<'a, Self>
    where
        Self: Sized,
    {
        SinglePinRef::from(self)
    }

    fn interface_mut<E>(&'a mut self) -> SinglePinMut<'a, Self, E>
    where
        Self: Sized,
        E: From<PinError>,
    {
        SinglePinMut::from(self)
    }

    fn name(&self) -> &str;
    fn iter_possible_signals(&self) -> impl Iterator<Item = PinSignal>;
    fn iter_prev_possible_signals(&self) -> impl Iterator<Item = PinSignal>;
    fn possible_signals(&self) -> Vec<PinSignal>;
    fn prev_possible_signals(&self) -> Vec<PinSignal>;
    fn possible_reads(&self) -> Vec<bool>;
    fn prev_possible_reads(&self) -> Vec<bool>;
    fn collapsed(&self) -> Option<PinSignal>;
    fn prev_collapsed(&self) -> Option<PinSignal>;
    fn set_signal_in(&mut self, signal: PinSignal, possible: bool) -> Result<(), PinError>;
    fn set_all_signals_in(&mut self, possible: bool) -> Result<(), PinError>;
    fn set_possible_in_to_prev(&mut self) -> Result<(), PinError>;

    fn set_drive_in(&mut self, bool_signal: bool, possible: bool) -> Result<(), PinError> {
        self.set_signal_in(PinSignal::from_bool(bool_signal), possible)
    }

    fn set_high_z_in(&mut self, possible: bool) {
        self.set_signal_in(PinSignal::HighZ, possible)
            .expect("setting high impedance in should never panic");
    }

    fn add_signal_in(&mut self, signal: PinSignal) -> Result<(), PinError> {
        self.set_signal_in(signal, true)
    }

    fn add_drive_in(&mut self, bool_signal: bool) -> Result<(), PinError> {
        self.set_drive_in(bool_signal, true)
    }

    fn add_high_z_in(&mut self) {
        self.set_high_z_in(true);
    }

    fn add_all_signals_in(&mut self) -> Result<(), PinError> {
        self.set_all_signals_in(true)
    }
}
