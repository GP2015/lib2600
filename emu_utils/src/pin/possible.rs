use crate::pin::PinSignal;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PossibleSignals {
    pub high: bool,
    pub low: bool,
    pub tri_state: bool,
}

impl PossibleSignals {
    pub fn from(high: bool, low: bool, tri_state: bool) -> Self {
        Self {
            high,
            low,
            tri_state,
        }
    }

    pub fn set_signal(&mut self, signal: PinSignal, enable: bool) {
        match signal {
            PinSignal::High => self.high = enable,
            PinSignal::Low => self.low = enable,
            PinSignal::TriState => self.tri_state = enable,
        };
    }

    pub fn set_bool_signal(&mut self, bool_signal: bool, enable: bool) {
        if bool_signal {
            self.high = enable;
        } else {
            self.low = enable;
        }
    }

    pub fn set_all(&mut self, enable: bool) {
        self.high = enable;
        self.low = enable;
        self.tri_state = enable;
    }

    pub fn with_signal(mut self, signal: PinSignal, enable: bool) -> Self {
        self.set_signal(signal, enable);
        self
    }

    pub fn with_bool_signal(mut self, bool_signal: bool, enable: bool) -> Self {
        self.set_bool_signal(bool_signal, enable);
        self
    }

    pub fn with_tri_state(mut self, enable: bool) -> Self {
        self.tri_state = enable;
        self
    }

    pub fn with_all(mut self, enable: bool) -> Self {
        self.set_all(enable);
        self
    }

    pub fn all_enabled(&self) -> Vec<PinSignal> {
        [
            (self.high, PinSignal::High),
            (self.low, PinSignal::Low),
            (self.tri_state, PinSignal::TriState),
        ]
        .into_iter()
        .filter_map(|(enabled, signal)| enabled.then_some(signal))
        .collect()
    }

    pub fn collapsed(&self) -> Option<PinSignal> {
        let vec = self.all_enabled();
        if vec.len() == 1 { Some(vec[0]) } else { None }
    }

    pub fn contend_together(first: Self, second: Self) -> Option<Self> {
        let mut result = Self::from(false, false, false);

        for first_signal in first.all_enabled() {
            for second_signal in second.all_enabled() {
                let signal = PinSignal::contend_together(first_signal, second_signal)?;
                result.set_signal(signal, true);
            }
        }

        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    fn from(
        #[values(true, false)] high: bool,
        #[values(true, false)] low: bool,
        #[values(true, false)] tri_state: bool,
    ) {
        let signals = PossibleSignals::from(high, low, tri_state);
        assert_eq!(signals.high, high);
        assert_eq!(signals.low, low);
        assert_eq!(signals.tri_state, tri_state);
    }

    #[rstest]
    fn set_signal(
        #[values(true, false)] high: bool,
        #[values(true, false)] low: bool,
        #[values(true, false)] tri_state: bool,
    ) {
        let mut signals = PossibleSignals::from(high, low, tri_state);
        signals.set_signal(PinSignal::High, true);
        signals.set_signal(PinSignal::Low, false);
        signals.set_signal(PinSignal::TriState, true);
        assert_eq!(signals, PossibleSignals::from(true, false, true));
    }

    #[rstest]
    fn set_all(
        #[values(true, false)] high: bool,
        #[values(true, false)] low: bool,
        #[values(true, false)] tri_state: bool,
        #[values(true, false)] enable: bool,
    ) {
        let mut signals = PossibleSignals::from(high, low, tri_state);
        signals.set_all(enable);
        assert_eq!(signals, PossibleSignals::from(enable, enable, enable));
    }

    #[rstest]
    fn all_enabled() {
        let signals = PossibleSignals::from(true, false, true).all_enabled();
        assert_eq!(signals, vec![PinSignal::High, PinSignal::TriState]);
    }

    #[rstest]
    #[case(true, false, false, PinSignal::High)]
    #[case(false, true, false, PinSignal::Low)]
    #[case(false, false, true, PinSignal::TriState)]
    fn collapsed_success(
        #[case] high: bool,
        #[case] low: bool,
        #[case] tri_state: bool,
        #[case] signal: PinSignal,
    ) {
        let signals = PossibleSignals::from(high, low, tri_state);
        assert_eq!(signals.collapsed().unwrap(), signal);
    }

    #[rstest]
    #[case(false, false, false)]
    #[case(false, true, true)]
    #[case(true, false, true)]
    #[case(true, true, false)]
    #[case(true, true, true)]
    fn collapsed_failure(#[case] high: bool, #[case] low: bool, #[case] tri_state: bool) {
        let signals = PossibleSignals::from(high, low, tri_state);
        assert!(signals.collapsed().is_none());
    }
}
