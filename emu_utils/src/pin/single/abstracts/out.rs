use crate::pin::{PinSignal, SinglePinCore};

pub trait SinglePinOutput<'a>: SinglePinCore<'a> {
    fn set_signal_out(&mut self, signal: PinSignal, possible: bool) -> Result<(), Self::ErrType>;
    fn set_all_signals_out(&mut self, possible: bool) -> Result<(), Self::ErrType>;
    fn set_out_to_prev(&mut self) -> Result<(), Self::ErrType>;

    fn set_drive_out(&mut self, bool_signal: bool, possible: bool) -> Result<(), Self::ErrType> {
        self.set_signal_out(PinSignal::from_bool(bool_signal), possible)
    }

    fn set_high_z_out(&mut self, possible: bool) {
        self.set_signal_out(PinSignal::HighZ, possible)
            .unwrap_or_else(|_| panic!("setting high impedance out should never panic"));
    }

    fn add_signal_out(&mut self, signal: PinSignal) -> Result<(), Self::ErrType> {
        self.set_signal_out(signal, true)
    }

    fn add_drive_out(&mut self, bool_signal: bool) -> Result<(), Self::ErrType> {
        self.set_drive_out(bool_signal, true)
    }

    fn add_high_z_out(&mut self) {
        self.set_high_z_out(true);
    }

    fn add_all_signals_out(&mut self) -> Result<(), Self::ErrType> {
        self.set_all_signals_out(true)
    }
}
