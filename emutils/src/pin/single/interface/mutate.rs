use crate::pin::PinSignal;

pub trait PinInputUIMutate {
    type ErrType;

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
