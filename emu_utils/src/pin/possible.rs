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

    pub fn set_all(&mut self, enable: bool) {
        self.high = enable;
        self.low = enable;
        self.tri_state = enable;
    }

    pub fn with_signal(mut self, signal: PinSignal, enable: bool) -> Self {
        self.set_signal(signal, enable);
        self
    }

    pub fn iter_all_enabled(&self) -> impl Iterator<Item = PinSignal> {
        [
            (self.high, PinSignal::High),
            (self.low, PinSignal::Low),
            (self.tri_state, PinSignal::TriState),
        ]
        .into_iter()
        .filter_map(|(enabled, signal)| enabled.then_some(signal))
    }

    pub fn all_enabled(&self) -> Vec<PinSignal> {
        self.iter_all_enabled().collect()
    }

    pub fn collapsed(&self) -> Option<PinSignal> {
        let vec = self.all_enabled();
        if vec.len() == 1 { Some(vec[0]) } else { None }
    }

    pub fn contend_together(first: Self, second: Self) -> Option<Self> {
        let first_all_enabled = first.all_enabled();
        let second_all_enabled = second.all_enabled();

        if first_all_enabled.is_empty() || second_all_enabled.is_empty() {
            return None;
        }

        let mut result = Self::from(false, false, false);

        for first_signal in &first_all_enabled {
            for second_signal in &second_all_enabled {
                let signal = PinSignal::contend_together(*first_signal, *second_signal)?;
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
        #[values(true, false)] initial: bool,
        #[values(true, false)] enable: bool,
        #[values(PinSignal::High, PinSignal::Low, PinSignal::TriState)] signal: PinSignal,
    ) {
        let mut signals = PossibleSignals::from(initial, initial, initial);
        signals.set_signal(signal, enable);
        let result: bool = match signal {
            PinSignal::High => signals.high,
            PinSignal::Low => signals.low,
            PinSignal::TriState => signals.tri_state,
        };
        assert_eq!(result, enable);
    }

    #[rstest]
    fn with_signal(
        #[values(true, false)] initial: bool,
        #[values(true, false)] enable: bool,
        #[values(PinSignal::High, PinSignal::Low, PinSignal::TriState)] signal: PinSignal,
    ) {
        let signals = PossibleSignals::from(initial, initial, initial).with_signal(signal, enable);
        let result = match signal {
            PinSignal::High => signals.high,
            PinSignal::Low => signals.low,
            PinSignal::TriState => signals.tri_state,
        };
        assert_eq!(result, enable);
    }

    #[rstest]
    #[case(false, false, false, vec![])]
    #[case(false, false, true, vec![PinSignal::TriState])]
    #[case(false, true, false, vec![PinSignal::Low])]
    #[case(false, true, true, vec![PinSignal::Low, PinSignal::TriState])]
    #[case(true, false, false, vec![PinSignal::High])]
    #[case(true, false, true, vec![PinSignal::High, PinSignal::TriState])]
    #[case(true, true, false, vec![PinSignal::High, PinSignal::Low])]
    #[case(true, true, true, vec![PinSignal::High, PinSignal::Low, PinSignal::TriState])]
    fn all_enabled(
        #[case] high: bool,
        #[case] low: bool,
        #[case] tri_state: bool,
        #[case] res_vec: Vec<PinSignal>,
    ) {
        let signals = PossibleSignals::from(high, low, tri_state).all_enabled();
        assert_eq!(signals, res_vec);
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

    #[rstest]
    fn contend_failure_empty(
        #[values(true, false)] high: bool,
        #[values(true, false)] low: bool,
        #[values(true, false)] tri_state: bool,
    ) {
        let non_empty = PossibleSignals::from(high, low, tri_state);
        let empty = PossibleSignals::from(false, false, false);
        assert!(PossibleSignals::contend_together(non_empty, empty).is_none());
        assert!(PossibleSignals::contend_together(empty, non_empty).is_none());
    }

    #[rstest]
    #[case(false, false, true)]
    #[case(false, true, false)]
    #[case(false, true, true)]
    #[case(true, false, false)]
    #[case(true, false, true)]
    #[case(true, true, false)]
    #[case(true, true, true)]
    fn contend_success_tristate_only(
        #[case] first_high: bool,
        #[case] first_low: bool,
        #[case] first_tri_state: bool,
    ) {
        let first = PossibleSignals::from(first_high, first_low, first_tri_state);
        let second = PossibleSignals::from(false, false, true);
        assert_eq!(
            PossibleSignals::contend_together(first, second).unwrap(),
            PossibleSignals::from(first_high, first_low, first_tri_state)
        );
    }

    #[rstest]
    fn contend_success_contention(
        #[values(true, false)] bool_state: bool,
        #[values(true, false)] first_tri_state: bool,
        #[values(true, false)] second_tri_state: bool,
    ) {
        let first = PossibleSignals::from(bool_state, !bool_state, first_tri_state);
        let second = PossibleSignals::from(bool_state, !bool_state, second_tri_state);
        assert_eq!(
            PossibleSignals::contend_together(first, second).unwrap(),
            PossibleSignals::from(bool_state, !bool_state, first_tri_state & second_tri_state)
        );
    }

    #[rstest]
    #[case(false, true, true, false)]
    #[case(false, true, true, true)]
    #[case(true, false, false, true)]
    #[case(true, false, true, true)]
    #[case(true, true, false, true)]
    #[case(true, true, true, false)]
    #[case(true, true, true, true)]
    fn contend_failure_contention(
        #[case] first_high: bool,
        #[case] first_low: bool,
        #[case] second_high: bool,
        #[case] second_low: bool,
        #[values(true, false)] first_tri_state: bool,
        #[values(true, false)] second_tri_state: bool,
    ) {
        let first = PossibleSignals::from(first_high, first_low, first_tri_state);
        let second = PossibleSignals::from(second_high, second_low, second_tri_state);
        assert!(PossibleSignals::contend_together(first, second).is_none());
    }
}
