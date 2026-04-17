use crate::pin::PinSignal;

trait SinglePinMut {
    type ErrType;

    fn name(&self) -> &str;
    fn signal_possible(&self, signal: PinSignal) -> bool;
    fn high_possible(&self) -> bool;
    fn low_possible(&self) -> bool;
    fn high_z_possible(&self) -> bool;
    fn prev_signal_possible(&self, signal: PinSignal) -> bool;
    fn prev_high_possible(&self) -> bool;
    fn prev_low_possible(&self) -> bool;
    fn prev_high_z_possible(&self) -> bool;
    fn could_read_high(&self) -> bool;
    fn could_read_low(&self) -> bool;
    fn collapsed(&self) -> Option<PinSignal>;
    fn prev_collapsed(&self) -> Option<PinSignal>;
    fn possible_signals(&self) -> Vec<PinSignal>;
    fn prev_possible_signals(&self) -> Vec<PinSignal>;
    fn possible_reads(&self) -> Vec<bool>;
    fn prev_possible_reads(&self) -> Vec<bool>;

    fn add_signal_in(
        &mut self,
        signal: PinSignal,
        only_possible: bool,
    ) -> Result<(), Self::ErrType>;
    fn add_high_in(&mut self, only_possible: bool) -> Result<(), Self::ErrType>;
    fn add_low_in(&mut self, only_possible: bool) -> Result<(), Self::ErrType>;
    fn add_high_z_in(&mut self, only_possible: bool);
    fn remove_signal_in(&mut self, signal: PinSignal);
    fn remove_high_in(&mut self);
    fn remove_low_in(&mut self);
    fn remove_high_z_in(&mut self);
    fn add_drive_in(&mut self, bool_signal: bool, only_possible: bool)
    -> Result<(), Self::ErrType>;
    fn remove_drive_in(&mut self, bool_signal: bool);
    fn set_all_in(&mut self, high: bool, low: bool, high_z: bool) -> Result<(), Self::ErrType>;
    fn set_in_to_prev(&mut self) -> Result<(), Self::ErrType>;
}
