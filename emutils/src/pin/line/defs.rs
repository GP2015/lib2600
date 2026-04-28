use delegate::delegate;

use crate::{
    pin::{PinError, PinSignal},
    reg::BitRegister,
};

pub trait PinCore {
    fn new<S: Into<String>>(name: S) -> Self;
    fn post_tick_update(&mut self);
}

pub trait PinQuery {
    fn name(&self) -> &str;
    fn signal_possible_when(&self, signal: PinSignal, prev: bool) -> bool;

    fn could_read_high_when(&self, prev: bool) -> bool {
        self.high_possible_when(prev) | self.high_z_possible_when(prev)
    }

    fn could_read_low_when(&self, prev: bool) -> bool {
        self.low_possible_when(prev) | self.high_z_possible_when(prev)
    }

    fn collapsed_when(&self, prev: bool) -> Option<PinSignal> {
        match (
            self.high_possible_when(prev),
            self.low_possible_when(prev),
            self.high_z_possible_when(prev),
        ) {
            (true, false, false) => Some(PinSignal::High),
            (false, true, false) => Some(PinSignal::Low),
            (false, false, true) => Some(PinSignal::HighZ),
            _ => None,
        }
    }

    fn read_when(&self, prev: bool) -> Option<bool> {
        self.collapsed_when(prev)
            .and_then(|signal| signal.as_bool())
    }

    fn iter_possible_signals_when(&self, prev: bool) -> impl Iterator<Item = PinSignal> {
        [
            (self.high_possible_when(prev), PinSignal::High),
            (self.low_possible_when(prev), PinSignal::Low),
            (self.high_z_possible_when(prev), PinSignal::HighZ),
        ]
        .into_iter()
        .filter_map(|(state, signal)| state.then_some(signal))
    }

    fn possible_reads_when(&self, prev: bool) -> &'static [bool] {
        match (
            self.high_possible_when(prev),
            self.low_possible_when(prev),
            self.high_z_possible_when(prev),
        ) {
            (false, false, false) => &[],
            (false, true, false) => &[false],
            (true, false, false) => &[true],
            (true, true, false) | (_, _, true) => &[true, false],
        }
    }

    delegate! {
        to self {
            #[call(signal_possible_when)]
            fn signal_possible(&self, signal: PinSignal, [false]) -> bool;
            #[call(signal_possible_when)]
            fn prev_signal_possible(&self, signal: PinSignal, [true]) -> bool;

            #[call(signal_possible_when)]
            fn high_possible_when(&self, [PinSignal::High], prev: bool) -> bool;
            #[call(signal_possible_when)]
            fn low_possible_when(&self, [PinSignal::Low], prev: bool) -> bool;
            #[call(signal_possible_when)]
            fn high_z_possible_when(&self, [PinSignal::HighZ], prev: bool) -> bool;

            #[call(signal_possible_when)]
            fn high_possible(&self, [PinSignal::High], [false]) -> bool;
            #[call(signal_possible_when)]
            fn low_possible(&self, [PinSignal::Low], [false]) -> bool;
            #[call(signal_possible_when)]
            fn high_z_possible(&self, [PinSignal::HighZ], [false]) -> bool;

            #[call(signal_possible_when)]
            fn prev_high_possible(&self, [PinSignal::High], [true]) -> bool;
            #[call(signal_possible_when)]
            fn prev_low_possible(&self, [PinSignal::Low], [true]) -> bool;
            #[call(signal_possible_when)]
            fn prev_high_z_possible(&self, [PinSignal::HighZ], [true]) -> bool;

            #[call(could_read_high_when)]
            fn could_read_high(&self, [false]) -> bool;
            #[call(could_read_high_when)]
            fn prev_could_read_high(&self, [true]) -> bool;

            #[call(could_read_low_when)]
            fn could_read_low(&self, [false]) -> bool;
            #[call(could_read_low_when)]
            fn prev_could_read_low(&self, [true]) -> bool;

            #[call(collapsed_when)]
            fn collapsed(&self, [false]) -> Option<PinSignal>;
            #[call(collapsed_when)]
            fn prev_collapsed(&self, [true]) -> Option<PinSignal>;

            #[call(read_when)]
            fn read(&self, [false]) -> Option<bool>;
            #[call(read_when)]
            fn prev_read(&self, [true]) -> Option<bool>;

            #[call(iter_possible_signals_when)]
            fn iter_possible_signals(&self, [false]) -> impl Iterator<Item = PinSignal>;
            #[call(iter_possible_signals_when)]
            fn iter_prev_possible_signals(&self, [true]) -> impl Iterator<Item = PinSignal>;

            #[call(possible_reads_when)]
            fn possible_reads(&self, [false]) -> &'static [bool];
            #[call(possible_reads_when)]
            fn prev_possible_reads(&self, [true]) -> &'static [bool];
        }
    }
}

pub trait PinInput {
    fn add_signal(&mut self, signal: PinSignal, only_possible: bool) -> Result<(), PinError>;
    fn remove_signal(&mut self, signal: PinSignal);

    fn add_drive(&mut self, bool_signal: bool, only_possible: bool) -> Result<(), PinError> {
        if bool_signal {
            self.add_high(only_possible)
        } else {
            self.add_low(only_possible)
        }
    }

    fn remove_drive(&mut self, bool_signal: bool) {
        if bool_signal {
            self.remove_high();
        } else {
            self.remove_low();
        }
    }

    fn set_all(&mut self, high: bool, low: bool, high_z: bool) -> Result<(), PinError> {
        if high {
            self.add_high(false)?;
        } else {
            self.remove_high();
        }

        if low {
            self.add_low(false)?;
        } else {
            self.remove_low();
        }

        if high_z {
            self.add_high_z(false);
        } else {
            self.remove_high_z();
        }

        Ok(())
    }

    fn copy_from_prev(&mut self) -> Result<(), PinError> {
        self.set_all(
            self.prev_high_possible(),
            self.prev_low_possible(),
            self.prev_high_z_possible(),
        )
    }

    fn copy_from_pin<P: PinQuery>(&mut self, pin: &P, only_possible: bool) -> Result<(), PinError> {
        if only_possible {
            self.set_all_in(
                pin.high_possible(),
                pin.low_possible(),
                pin.high_z_possible(),
            )?;
        } else {
            if pin.high_possible() {
                self.add_high_in(false)?;
            }

            if pin.low_possible() {
                self.add_low_in(false)?;
            }

            if pin.high_z_possible() {
                self.add_high_z_in(false);
            }
        }
        Ok(())
    }

    fn copy_from_reg(&mut self, reg: &BitRegister, only_possible: bool) -> Result<(), PinError> {
        if only_possible {
            self.set_all_in(reg.high_possible(), reg.low_possible(), false)?;
        } else {
            if reg.high_possible() {
                self.add_high_in(false)?;
            }

            if reg.low_possible() {
                self.add_low_in(false)?;
            }
        }
        Ok(())
    }

    delegate! {
        to self {
            #[call(add_signal)]
            fn add_high(&mut self, [PinSignal::High], only_possible: bool) -> Result<(), PinError>;
            #[call(add_signal)]
            fn add_low(&mut self, [PinSignal::Low], only_possible: bool) -> Result<(), PinError>;
            #[call(add_signal)]
            #[unwrap]
            fn add_high_z(&mut self, [PinSignal::HighZ], only_possible: bool);

            #[call(remove_signal)]
            fn remove_high(&mut self, [PinSignal::High]);
            #[call(remove_signal)]
            fn remove_low(&mut self, [PinSignal::Low]);
            #[call(remove_signal)]
            fn remove_high_z(&mut self, [PinSignal::HighZ]);
        }
    }
}
