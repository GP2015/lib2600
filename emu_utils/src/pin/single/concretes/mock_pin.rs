use crate::pin::{PinError, PinSignal, SinglePinCore, SinglePinOutput, possible::PossibleSignals};
use delegate::delegate;
use std::marker::PhantomData;

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
    E: From<PinError>,
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
        self.signals.set_all(false);
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }

    delegate! {
        to self.signals {
            #[call(iter_all_enabled)]
            fn iter_possible_signals(&self) -> impl Iterator<Item = PinSignal>;

            #[call(all_enabled)]
            fn possible_signals(&self) -> Vec<PinSignal>;

            #[call(possible_reads)]
            fn possible_reads(&self) -> Vec<bool>;

            fn collapsed(&self) -> Option<PinSignal>;
        }

        to self.prev_signals {
            #[call(iter_all_enabled)]
            fn iter_prev_possible_signals(&self) -> impl Iterator<Item = PinSignal>;

            #[call(all_enabled)]
            fn prev_possible_signals(&self) -> Vec<PinSignal>;

            #[call(possible_reads)]
            fn prev_possible_reads(&self) -> Vec<bool>;

            #[call(collapsed)]
            fn prev_collapsed(&self) -> Option<PinSignal>;
        }

        #[expr($; Ok(()))]
        to self.signals {
            #[call(set_signal)]
            fn set_signal_in(&mut self, signal: PinSignal, possible: bool) -> Result<(), Self::ErrType>;

            #[call(set_all)]
            fn set_all_signals_in(&mut self, possible: bool) -> Result<(), Self::ErrType>;
        }
    }

    fn set_in_to_prev(&mut self) -> Result<(), Self::ErrType> {
        self.signals = self.prev_signals;
        Ok(())
    }
}

impl<E> SinglePinOutput<'_> for MockPin<E>
where
    E: From<PinError>,
{
    delegate! {
        #[expr($; Ok(()))]
        to self.signals {
            #[call(set_signal)]
            fn set_signal_out(&mut self, signal: PinSignal, possible: bool) -> Result<(), Self::ErrType>;

            #[call(set_all)]
            fn set_all_signals_out(&mut self, possible: bool) -> Result<(), Self::ErrType>;
        }
    }

    fn set_out_to_prev(&mut self) -> Result<(), Self::ErrType> {
        self.signals = self.prev_signals;
        Ok(())
    }
}
