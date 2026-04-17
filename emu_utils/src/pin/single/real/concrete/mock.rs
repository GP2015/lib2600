use crate::pin::{PinError, PinSignal, SinglePinCore, SinglePinOutput, possible::PossibleSignals};
use delegate::delegate;
use std::{fmt::Debug, marker::PhantomData};

pub struct MockPin<E>
where
    E: From<PinError>,
{
    name: String,
    signals: PossibleSignals,
    prev_signals: PossibleSignals,
    err_type: PhantomData<E>,
}

impl<E> SinglePinCore<'_> for MockPin<E>
where
    E: From<PinError> + Debug,
{
    type ErrType = E;

    fn new(name: String) -> Self {
        Self {
            name,
            signals: PossibleSignals::from(false, false, false),
            prev_signals: PossibleSignals::from(false, false, true),
            err_type: PhantomData,
        }
    }

    fn post_tick_update(&mut self) {
        self.prev_signals = self.signals;
        self.signals.set_all(false, false, false);
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }

    delegate! {
        to self.signals{
            fn signal_possible(&self, signal: PinSignal) -> bool;

            #[call(add_signal)]
            #[expr($; Ok(()))]
            fn add_signal_in(&mut self, signal: PinSignal, only_possible: bool) -> Result<(), Self::ErrType>;

            #[call(remove_signal)]
            fn remove_signal_in(&mut self, signal: PinSignal);
        }

        to self.prev_signals{
            #[call(signal_possible)]
            fn prev_signal_possible(&self, signal: PinSignal) -> bool;
        }
    }
}

impl<E> SinglePinOutput<'_> for MockPin<E>
where
    E: From<PinError> + Debug,
{
    delegate! {
        to self.signals{
            #[call(add_signal)]
            #[expr($; Ok(()))]
            fn add_signal_out(&mut self, signal: PinSignal, only_possible: bool) -> Result<(), Self::ErrType>;

            #[call(remove_signal)]
            fn remove_signal_out(&mut self, signal: PinSignal);
        }
    }
}
