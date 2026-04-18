use crate::{
    pin::{PinError, PinSignal},
    reg::BitRegister,
};

pub trait PinCore {
    fn new(name: String) -> Self;
    fn post_tick_update(&mut self);
}

pub trait PinInputUI {
    fn name(&self) -> &str;
    fn signal_possible(&self, signal: PinSignal) -> bool;
    fn prev_signal_possible(&self, signal: PinSignal) -> bool;
    fn add_signal_in(&mut self, signal: PinSignal, only_possible: bool) -> Result<(), PinError>;
    fn remove_signal_in(&mut self, signal: PinSignal);

    fn high_possible(&self) -> bool {
        self.signal_possible(PinSignal::High)
    }

    fn low_possible(&self) -> bool {
        self.signal_possible(PinSignal::Low)
    }

    fn high_z_possible(&self) -> bool {
        self.signal_possible(PinSignal::HighZ)
    }

    fn prev_high_possible(&self) -> bool {
        self.prev_signal_possible(PinSignal::High)
    }

    fn prev_low_possible(&self) -> bool {
        self.prev_signal_possible(PinSignal::Low)
    }

    fn prev_high_z_possible(&self) -> bool {
        self.prev_signal_possible(PinSignal::HighZ)
    }

    fn could_read_high(&self) -> bool {
        self.high_possible() | self.high_z_possible()
    }

    fn could_read_low(&self) -> bool {
        self.low_possible() | self.high_z_possible()
    }

    fn collapsed(&self) -> Option<PinSignal> {
        match (
            self.high_possible(),
            self.low_possible(),
            self.high_z_possible(),
        ) {
            (true, false, false) => Some(PinSignal::High),
            (false, true, false) => Some(PinSignal::Low),
            (false, false, true) => Some(PinSignal::HighZ),
            _ => None,
        }
    }

    fn prev_collapsed(&self) -> Option<PinSignal> {
        match (
            self.prev_high_possible(),
            self.prev_low_possible(),
            self.prev_high_z_possible(),
        ) {
            (true, false, false) => Some(PinSignal::High),
            (false, true, false) => Some(PinSignal::Low),
            (false, false, true) => Some(PinSignal::HighZ),
            _ => None,
        }
    }

    fn possible_signals(&self) -> Vec<PinSignal> {
        [
            (self.high_possible(), PinSignal::High),
            (self.low_possible(), PinSignal::Low),
            (self.high_z_possible(), PinSignal::HighZ),
        ]
        .into_iter()
        .filter_map(|(state, signal)| state.then_some(signal))
        .collect()
    }

    fn prev_possible_signals(&self) -> Vec<PinSignal> {
        [
            (self.prev_high_possible(), PinSignal::High),
            (self.prev_low_possible(), PinSignal::Low),
            (self.prev_high_z_possible(), PinSignal::HighZ),
        ]
        .into_iter()
        .filter_map(|(state, signal)| state.then_some(signal))
        .collect()
    }

    fn possible_reads(&self) -> Vec<bool> {
        match (
            self.high_possible(),
            self.low_possible(),
            self.high_z_possible(),
        ) {
            (false, false, false) => Vec::new(),
            (false, true, false) => vec![false],
            (true, false, false) => vec![true],
            (true, true, false) | (_, _, true) => vec![true, false],
        }
    }

    fn prev_possible_reads(&self) -> Vec<bool> {
        match (
            self.prev_high_possible(),
            self.prev_low_possible(),
            self.prev_high_z_possible(),
        ) {
            (false, false, false) => Vec::new(),
            (false, true, false) => vec![false],
            (true, false, false) => vec![true],
            (true, true, false) | (_, _, true) => vec![true, false],
        }
    }

    fn add_high_in(&mut self, only_possible: bool) -> Result<(), PinError> {
        self.add_signal_in(PinSignal::High, only_possible)
    }

    fn add_low_in(&mut self, only_possible: bool) -> Result<(), PinError> {
        self.add_signal_in(PinSignal::Low, only_possible)
    }

    fn add_high_z_in(&mut self, only_possible: bool) {
        self.add_signal_in(PinSignal::HighZ, only_possible)
            .expect("setting high impedance in cannot cause a short-circuit");
    }

    fn remove_high_in(&mut self) {
        self.remove_signal_in(PinSignal::High);
    }

    fn remove_low_in(&mut self) {
        self.remove_signal_in(PinSignal::Low);
    }

    fn remove_high_z_in(&mut self) {
        self.remove_signal_in(PinSignal::HighZ);
    }

    fn add_drive_in(&mut self, bool_signal: bool, only_possible: bool) -> Result<(), PinError> {
        if bool_signal {
            self.add_high_in(only_possible)
        } else {
            self.add_low_in(only_possible)
        }
    }

    fn remove_drive_in(&mut self, bool_signal: bool) {
        if bool_signal {
            self.remove_high_in();
        } else {
            self.remove_low_in();
        }
    }

    fn set_all_in(&mut self, high: bool, low: bool, high_z: bool) -> Result<(), PinError> {
        if high {
            self.add_high_in(false)?;
        } else {
            self.remove_high_in();
        }

        if low {
            self.add_low_in(false)?;
        } else {
            self.remove_low_in();
        }

        if high_z {
            self.add_high_z_in(false);
        } else {
            self.remove_high_z_in();
        }

        Ok(())
    }

    fn set_in_to_prev(&mut self) -> Result<(), PinError> {
        self.set_all_in(
            self.high_possible(),
            self.low_possible(),
            self.high_z_possible(),
        )
    }
}

pub trait PinOutput {
    fn add_signal_out(&mut self, signal: PinSignal, only_possible: bool) -> Result<(), PinError>;
    fn remove_signal_out(&mut self, signal: PinSignal);

    fn add_high_out(&mut self, only_possible: bool) -> Result<(), PinError> {
        self.add_signal_out(PinSignal::High, only_possible)
    }

    fn add_low_out(&mut self, only_possible: bool) -> Result<(), PinError> {
        self.add_signal_out(PinSignal::Low, only_possible)
    }

    fn add_high_z_out(&mut self, only_possible: bool) {
        self.add_signal_out(PinSignal::HighZ, only_possible)
            .expect("setting high impedance in cannot cause a short-circuit");
    }

    fn remove_high_out(&mut self) {
        self.remove_signal_out(PinSignal::High);
    }

    fn remove_low_out(&mut self) {
        self.remove_signal_out(PinSignal::Low);
    }

    fn remove_high_z_out(&mut self) {
        self.remove_signal_out(PinSignal::HighZ);
    }

    fn add_drive_out(&mut self, bool_signal: bool, only_possible: bool) -> Result<(), PinError> {
        if bool_signal {
            self.add_high_out(only_possible)
        } else {
            self.add_low_out(only_possible)
        }
    }

    fn remove_drive_out(&mut self, bool_signal: bool) {
        if bool_signal {
            self.remove_high_out();
        } else {
            self.remove_low_out();
        }
    }

    fn set_all_out(&mut self, high: bool, low: bool, high_z: bool) -> Result<(), PinError> {
        if high {
            self.add_high_out(false)?;
        } else {
            self.remove_high_out();
        }

        if low {
            self.add_low_out(false)?;
        } else {
            self.remove_low_out();
        }

        if high_z {
            self.add_high_z_out(false);
        } else {
            self.remove_high_z_out();
        }

        Ok(())
    }

    fn output_from_pin<P>(&mut self, pin: &P, only_possible: bool) -> Result<(), PinError>
    where
        P: PinInputUI,
    {
        if only_possible {
            self.set_all_out(
                pin.high_possible(),
                pin.low_possible(),
                pin.high_z_possible(),
            )?;
        } else {
            if pin.high_possible() {
                self.add_high_out(false)?;
            }

            if pin.low_possible() {
                self.add_low_out(false)?;
            }

            if pin.high_z_possible() {
                self.add_high_z_out(false);
            }
        }
        Ok(())
    }

    fn output_from_reg(&mut self, reg: &BitRegister, only_possible: bool) -> Result<(), PinError> {
        if only_possible {
            self.set_all_out(reg.high_possible(), reg.low_possible(), false)?;
        } else {
            if reg.high_possible() {
                self.add_high_out(false)?;
            }

            if reg.low_possible() {
                self.add_low_out(false)?;
            }
        }
        Ok(())
    }
}
