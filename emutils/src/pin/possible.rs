use crate::pin::PinSignal;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PossibleSignals {
    pub high: bool,
    pub low: bool,
    pub high_z: bool,
}

impl PossibleSignals {
    pub fn from(high: bool, low: bool, high_z: bool) -> Self {
        Self { high, low, high_z }
    }

    pub fn is_possible(self, signal: PinSignal) -> bool {
        match signal {
            PinSignal::High => self.high,
            PinSignal::Low => self.low,
            PinSignal::HighZ => self.high_z,
        }
    }

    pub fn iter_all_enabled(self) -> impl Iterator<Item = PinSignal> {
        [
            (self.high, PinSignal::High),
            (self.low, PinSignal::Low),
            (self.high_z, PinSignal::HighZ),
        ]
        .into_iter()
        .filter_map(|(enabled, signal)| enabled.then_some(signal))
    }

    pub fn add(&mut self, signal: PinSignal, only_possible: bool) {
        if only_possible {
            self.set_all(false, false, false);
        }

        match signal {
            PinSignal::High => self.high = true,
            PinSignal::Low => self.low = true,
            PinSignal::HighZ => self.high_z = true,
        }
    }

    pub fn remove(&mut self, signal: PinSignal) {
        match signal {
            PinSignal::High => self.high = false,
            PinSignal::Low => self.low = false,
            PinSignal::HighZ => self.high_z = false,
        }
    }

    pub fn set_all(&mut self, high: bool, low: bool, high_z: bool) {
        self.high = high;
        self.low = low;
        self.high_z = high_z;
    }

    pub fn contend_with(self, other: Self) -> Option<Self> {
        let mut result = Self::from(false, false, false);

        for first_signal in self.iter_all_enabled() {
            for second_signal in other.iter_all_enabled() {
                let signal = PinSignal::contend_together(first_signal, second_signal)?;
                result.add(signal, false);
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
        #[values(true, false)] high_z: bool,
    ) {
        let signals = PossibleSignals::from(high, low, high_z);
        assert_eq!(signals.high, high);
        assert_eq!(signals.low, low);
        assert_eq!(signals.high_z, high_z);
    }

    #[rstest]
    fn is_possible(
        #[values(true, false)] high: bool,
        #[values(true, false)] low: bool,
        #[values(true, false)] high_z: bool,
    ) {
        let signals = PossibleSignals::from(high, low, high_z);
        assert_eq!(signals.is_possible(PinSignal::High), high);
        assert_eq!(signals.is_possible(PinSignal::Low), low);
        assert_eq!(signals.is_possible(PinSignal::HighZ), high_z);
    }

    #[rstest]
    #[case(false, false, false, &[])]
    #[case(false, false, true, &[PinSignal::HighZ])]
    #[case(false, true, false, &[PinSignal::Low])]
    #[case(false, true, true, &[PinSignal::Low, PinSignal::HighZ])]
    #[case(true, false, false, &[PinSignal::High])]
    #[case(true, false, true, &[PinSignal::High, PinSignal::HighZ])]
    #[case(true, true, false, &[PinSignal::High, PinSignal::Low])]
    #[case(true, true, true, &[PinSignal::High, PinSignal::Low, PinSignal::HighZ])]
    fn iter_all_enabled(
        #[case] high: bool,
        #[case] low: bool,
        #[case] high_z: bool,
        #[case] res: &[PinSignal],
    ) {
        let signals = PossibleSignals::from(high, low, high_z)
            .iter_all_enabled()
            .collect::<Vec<PinSignal>>();
        assert_eq!(signals, res);
    }

    #[rstest]
    fn add_not_only_possible(
        #[values(true, false)] initial: bool,
        #[values(PinSignal::High, PinSignal::Low, PinSignal::HighZ)] signal: PinSignal,
    ) {
        let mut signals = PossibleSignals::from(initial, initial, initial);
        signals.add(signal, false);
        for s in [PinSignal::High, PinSignal::Low, PinSignal::HighZ] {
            assert_eq!(signals.is_possible(s), signal == s || initial);
        }
    }

    #[rstest]
    fn add_only_possible(
        #[values(true, false)] initial: bool,
        #[values(PinSignal::High, PinSignal::Low, PinSignal::HighZ)] signal: PinSignal,
    ) {
        let mut signals = PossibleSignals::from(initial, initial, initial);
        signals.add(signal, true);
        for s in [PinSignal::High, PinSignal::Low, PinSignal::HighZ] {
            assert_eq!(signals.is_possible(s), signal == s);
        }
    }

    #[rstest]
    fn remove(
        #[values(true, false)] initial: bool,
        #[values(PinSignal::High, PinSignal::Low, PinSignal::HighZ)] signal: PinSignal,
    ) {
        let mut signals = PossibleSignals::from(initial, initial, initial);
        signals.remove(signal);
        for s in [PinSignal::High, PinSignal::Low, PinSignal::HighZ] {
            assert_eq!(signals.is_possible(s), signal != s && initial);
        }
    }

    #[rstest]
    fn set_all(
        #[values(true, false)] initial: bool,
        #[values(true, false)] high: bool,
        #[values(true, false)] low: bool,
        #[values(true, false)] high_z: bool,
    ) {
        let mut signals = PossibleSignals::from(initial, initial, initial);
        signals.set_all(high, low, high_z);
        assert_eq!(signals, PossibleSignals::from(high, low, high_z));
    }

    #[rstest]
    fn contend_success_empty(
        #[values(true, false)] high: bool,
        #[values(true, false)] low: bool,
        #[values(true, false)] high_z: bool,
    ) {
        let non_empty = PossibleSignals::from(high, low, high_z);
        let empty = PossibleSignals::from(false, false, false);
        assert_eq!(non_empty.contend_with(empty).unwrap(), empty);
        assert_eq!(empty.contend_with(non_empty).unwrap(), empty);
    }

    #[rstest]
    #[case(false, false, true)]
    #[case(false, true, false)]
    #[case(false, true, true)]
    #[case(true, false, false)]
    #[case(true, false, true)]
    #[case(true, true, false)]
    #[case(true, true, true)]
    fn contend_success_high_z_only(
        #[case] first_high: bool,
        #[case] first_low: bool,
        #[case] first_high_z: bool,
    ) {
        let first = PossibleSignals::from(first_high, first_low, first_high_z);
        let second = PossibleSignals::from(false, false, true);
        assert_eq!(first.contend_with(second).unwrap(), first);
    }

    #[rstest]
    fn contend_success_contention(
        #[values(true, false)] bool_state: bool,
        #[values(true, false)] first_high_z: bool,
        #[values(true, false)] second_high_z: bool,
    ) {
        let first = PossibleSignals::from(bool_state, !bool_state, first_high_z);
        let second = PossibleSignals::from(bool_state, !bool_state, second_high_z);
        let res = PossibleSignals::from(bool_state, !bool_state, first_high_z & second_high_z);
        assert_eq!(first.contend_with(second).unwrap(), res);
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
        #[values(true, false)] first_high_z: bool,
        #[values(true, false)] second_high_z: bool,
    ) {
        let first = PossibleSignals::from(first_high, first_low, first_high_z);
        let second = PossibleSignals::from(second_high, second_low, second_high_z);
        assert!(first.contend_with(second).is_none());
    }
}
