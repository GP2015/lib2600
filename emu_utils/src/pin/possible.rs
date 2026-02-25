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

    pub fn set_signal(&mut self, signal: PinSignal, enable: bool) {
        match signal {
            PinSignal::High => self.high = enable,
            PinSignal::Low => self.low = enable,
            PinSignal::HighZ => self.high_z = enable,
        };
    }

    pub fn set_all(&mut self, enable: bool) {
        self.high = enable;
        self.low = enable;
        self.high_z = enable;
    }

    pub fn with_signal(mut self, signal: PinSignal, enable: bool) -> Self {
        self.set_signal(signal, enable);
        self
    }

    pub fn with_all(mut self, enable: bool) -> Self {
        self.set_all(enable);
        self
    }

    pub fn iter_all_enabled(&self) -> impl Iterator<Item = PinSignal> {
        [
            (self.high, PinSignal::High),
            (self.low, PinSignal::Low),
            (self.high_z, PinSignal::HighZ),
        ]
        .into_iter()
        .filter_map(|(enabled, signal)| enabled.then_some(signal))
    }

    pub fn all_enabled(&self) -> Vec<PinSignal> {
        self.iter_all_enabled().collect()
    }

    pub fn all_possible_reads(&self) -> Vec<bool> {
        match (self.high, self.low, self.high_z) {
            (false, false, false) => Vec::new(),
            (false, true, false) => vec![false],
            (true, false, false) => vec![true],
            (true, true, false) => vec![true, false],
            (_, _, true) => vec![true, false],
        }
    }

    pub fn collapsed(&self) -> Option<PinSignal> {
        let vec = self.all_enabled();
        if vec.len() == 1 { Some(vec[0]) } else { None }
    }

    pub fn contend_together(first: Self, second: Self) -> Option<Self> {
        let first_all_enabled = first.all_enabled();
        let second_all_enabled = second.all_enabled();

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
        #[values(true, false)] high_z: bool,
    ) {
        let signals = PossibleSignals::from(high, low, high_z);
        assert_eq!(signals.high, high);
        assert_eq!(signals.low, low);
        assert_eq!(signals.high_z, high_z);
    }

    #[rstest]
    fn set_signal(
        #[values(true, false)] initial: bool,
        #[values(true, false)] enable: bool,
        #[values(PinSignal::High, PinSignal::Low, PinSignal::HighZ)] signal: PinSignal,
    ) {
        let mut signals = PossibleSignals::from(initial, initial, initial);
        signals.set_signal(signal, enable);
        let result: bool = match signal {
            PinSignal::High => signals.high,
            PinSignal::Low => signals.low,
            PinSignal::HighZ => signals.high_z,
        };
        assert_eq!(result, enable);
    }

    #[rstest]
    fn set_all(
        #[values(true, false)] high: bool,
        #[values(true, false)] low: bool,
        #[values(true, false)] high_z: bool,
        #[values(true, false)] enable: bool,
    ) {
        let mut signals = PossibleSignals::from(high, low, high_z);
        signals.set_all(enable);
        assert_eq!(signals, PossibleSignals::from(enable, enable, enable));
    }

    #[rstest]
    fn with_signal(
        #[values(true, false)] initial: bool,
        #[values(true, false)] enable: bool,
        #[values(PinSignal::High, PinSignal::Low, PinSignal::HighZ)] signal: PinSignal,
    ) {
        let signals = PossibleSignals::from(initial, initial, initial).with_signal(signal, enable);
        let result = match signal {
            PinSignal::High => signals.high,
            PinSignal::Low => signals.low,
            PinSignal::HighZ => signals.high_z,
        };
        assert_eq!(result, enable);
    }

    #[rstest]
    fn with_all(
        #[values(true, false)] high: bool,
        #[values(true, false)] low: bool,
        #[values(true, false)] high_z: bool,
        #[values(true, false)] enable: bool,
    ) {
        let signals = PossibleSignals::from(high, low, high_z).with_all(enable);
        assert_eq!(signals, PossibleSignals::from(enable, enable, enable));
    }

    #[rstest]
    #[case(false, false, false, vec![])]
    #[case(false, false, true, vec![PinSignal::HighZ])]
    #[case(false, true, false, vec![PinSignal::Low])]
    #[case(false, true, true, vec![PinSignal::Low, PinSignal::HighZ])]
    #[case(true, false, false, vec![PinSignal::High])]
    #[case(true, false, true, vec![PinSignal::High, PinSignal::HighZ])]
    #[case(true, true, false, vec![PinSignal::High, PinSignal::Low])]
    #[case(true, true, true, vec![PinSignal::High, PinSignal::Low, PinSignal::HighZ])]
    fn all_enabled(
        #[case] high: bool,
        #[case] low: bool,
        #[case] high_z: bool,
        #[case] res_vec: Vec<PinSignal>,
    ) {
        let signals = PossibleSignals::from(high, low, high_z).all_enabled();
        assert_eq!(signals, res_vec);
    }

    #[rstest]
    #[case(true, false, false, PinSignal::High)]
    #[case(false, true, false, PinSignal::Low)]
    #[case(false, false, true, PinSignal::HighZ)]
    fn collapsed_success(
        #[case] high: bool,
        #[case] low: bool,
        #[case] high_z: bool,
        #[case] signal: PinSignal,
    ) {
        let signals = PossibleSignals::from(high, low, high_z);
        assert_eq!(signals.collapsed().unwrap(), signal);
    }

    #[rstest]
    #[case(false, false, false)]
    #[case(false, true, true)]
    #[case(true, false, true)]
    #[case(true, true, false)]
    #[case(true, true, true)]
    fn collapsed_failure(#[case] high: bool, #[case] low: bool, #[case] high_z: bool) {
        let signals = PossibleSignals::from(high, low, high_z);
        assert!(signals.collapsed().is_none());
    }

    #[rstest]
    fn contend_success_empty(
        #[values(true, false)] high: bool,
        #[values(true, false)] low: bool,
        #[values(true, false)] high_z: bool,
    ) {
        let non_empty = PossibleSignals::from(high, low, high_z);
        let empty = PossibleSignals::from(false, false, false);
        assert_eq!(
            PossibleSignals::contend_together(non_empty, empty).unwrap(),
            empty
        );
        assert_eq!(
            PossibleSignals::contend_together(empty, non_empty).unwrap(),
            empty
        );
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
        assert_eq!(
            PossibleSignals::contend_together(first, second).unwrap(),
            PossibleSignals::from(first_high, first_low, first_high_z)
        );
    }

    #[rstest]
    fn contend_success_contention(
        #[values(true, false)] bool_state: bool,
        #[values(true, false)] first_high_z: bool,
        #[values(true, false)] second_high_z: bool,
    ) {
        let first = PossibleSignals::from(bool_state, !bool_state, first_high_z);
        let second = PossibleSignals::from(bool_state, !bool_state, second_high_z);
        assert_eq!(
            PossibleSignals::contend_together(first, second).unwrap(),
            PossibleSignals::from(bool_state, !bool_state, first_high_z & second_high_z)
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
        #[values(true, false)] first_high_z: bool,
        #[values(true, false)] second_high_z: bool,
    ) {
        let first = PossibleSignals::from(first_high, first_low, first_high_z);
        let second = PossibleSignals::from(second_high, second_low, second_high_z);
        assert!(PossibleSignals::contend_together(first, second).is_none());
    }
}
