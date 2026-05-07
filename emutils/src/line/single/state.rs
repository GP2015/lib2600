use crate::line::LineSignal;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct LineState {
    pub low: bool,
    pub high: bool,
    pub high_z: bool,
}

impl LineState {
    #[must_use]
    pub const fn new(low: bool, high: bool, high_z: bool) -> Self {
        Self { low, high, high_z }
    }

    #[must_use]
    pub const fn is_possible(self, signal: LineSignal) -> bool {
        match signal {
            LineSignal::Low => self.low,
            LineSignal::High => self.high,
            LineSignal::HighZ => self.high_z,
        }
    }

    #[must_use]
    pub const fn could_read_low(self) -> bool {
        self.low | self.high_z
    }

    #[must_use]
    pub const fn could_read_high(self) -> bool {
        self.high | self.high_z
    }

    #[must_use]
    pub const fn could_read_low_high(self) -> (bool, bool) {
        (self.could_read_low(), self.could_read_high())
    }

    #[must_use]
    pub const fn is_defined(self) -> bool {
        matches!(
            (self.low, self.high, self.high_z),
            (true, false, false) | (false, true, false) | (false, false, true)
        )
    }

    #[must_use]
    pub const fn collapsed(self) -> Option<LineSignal> {
        match (self.low, self.high, self.high_z) {
            (true, false, false) => Some(LineSignal::Low),
            (false, true, false) => Some(LineSignal::High),
            (false, false, true) => Some(LineSignal::HighZ),
            _ => None,
        }
    }

    #[must_use]
    pub const fn read(self) -> Option<bool> {
        match (self.low, self.high, self.high_z) {
            (true, false, false) => Some(false),
            (false, true, false) => Some(true),
            _ => None,
        }
    }

    #[must_use]
    pub const fn is_valid(self) -> bool {
        self.low | self.high | self.high_z
    }

    pub fn iter_possible(self) -> impl Iterator<Item = LineSignal> {
        [
            (self.low, LineSignal::Low),
            (self.high, LineSignal::High),
            (self.high_z, LineSignal::HighZ),
        ]
        .into_iter()
        .filter_map(|(enabled, signal)| enabled.then_some(signal))
    }

    #[must_use]
    pub const fn possible_reads(self) -> &'static [bool] {
        match (self.low, self.high, self.high_z) {
            (false, false, false) => &[],
            (true, false, false) => &[false],
            (false, true, false) => &[true],
            (true, true, false) | (_, _, true) => &[false, true],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    fn new(
        #[values(true, false)] low: bool,
        #[values(true, false)] high: bool,
        #[values(true, false)] high_z: bool,
    ) {
        let signals = LineState::new(low, high, high_z);
        assert_eq!(signals.low, low);
        assert_eq!(signals.high, high);
        assert_eq!(signals.high_z, high_z);
    }

    #[rstest]
    fn is_possible(
        #[values(true, false)] low: bool,
        #[values(true, false)] high: bool,
        #[values(true, false)] high_z: bool,
    ) {
        let signals = LineState::new(low, high, high_z);
        assert_eq!(signals.is_possible(LineSignal::Low), low);
        assert_eq!(signals.is_possible(LineSignal::High), high);
        assert_eq!(signals.is_possible(LineSignal::HighZ), high_z);
    }

    #[rstest]
    #[case(false, false, false, false, false)]
    #[case(false, false, true, true, true)]
    #[case(true, false, false, true, false)]
    #[case(true, false, true, true, true)]
    #[case(false, true, false, false, true)]
    #[case(false, true, true, true, true)]
    #[case(true, true, false, true, true)]
    #[case(true, true, true, true, true)]
    fn could_read(
        #[case] low: bool,
        #[case] high: bool,
        #[case] high_z: bool,
        #[case] could_read_low: bool,
        #[case] could_read_high: bool,
    ) {
        let signals = LineState::new(low, high, high_z);
        assert_eq!(signals.could_read_low(), could_read_low);
        assert_eq!(signals.could_read_high(), could_read_high);
    }

    #[rstest]
    #[case(true, false, false, LineSignal::Low)]
    #[case(false, true, false, LineSignal::High)]
    #[case(false, false, true, LineSignal::HighZ)]
    fn collapsed_success(
        #[case] low: bool,
        #[case] high: bool,
        #[case] high_z: bool,
        #[case] res: LineSignal,
    ) {
        let state = LineState::new(low, high, high_z);
        assert_eq!(state.collapsed().unwrap(), res);
    }

    #[rstest]
    #[case(false, false, false)]
    #[case(true, false, true)]
    #[case(false, true, true)]
    #[case(true, true, false)]
    #[case(true, true, true)]
    fn collapsed_failure(#[case] low: bool, #[case] high: bool, #[case] high_z: bool) {
        let state = LineState::new(low, high, high_z);
        assert!(state.collapsed().is_none());
    }

    #[rstest]
    #[case(true, false, false, false)]
    #[case(false, true, false, true)]
    fn read_success(
        #[case] low: bool,
        #[case] high: bool,
        #[case] high_z: bool,
        #[case] res: bool,
    ) {
        let signals = LineState::new(low, high, high_z);
        assert_eq!(signals.read().unwrap(), res);
    }

    #[rstest]
    #[case(false, false, false)]
    #[case(false, false, true)]
    #[case(true, false, true)]
    #[case(false, true, true)]
    #[case(true, true, false)]
    #[case(true, true, true)]
    fn read_failure(#[case] low: bool, #[case] high: bool, #[case] high_z: bool) {
        let signals = LineState::new(low, high, high_z);
        assert!(signals.read().is_none());
    }

    #[rstest]
    #[case(false, false, false, &[])]
    #[case(false, false, true, &[LineSignal::HighZ])]
    #[case(true, false, false, &[LineSignal::Low])]
    #[case(true, false, true, &[LineSignal::Low, LineSignal::HighZ])]
    #[case(false, true, false, &[LineSignal::High])]
    #[case(false, true, true, &[LineSignal::High, LineSignal::HighZ])]
    #[case(true, true, false, &[LineSignal::Low, LineSignal::High])]
    #[case(true, true, true, &[LineSignal::Low, LineSignal::High, LineSignal::HighZ])]
    fn iter_possible(
        #[case] low: bool,
        #[case] high: bool,
        #[case] high_z: bool,
        #[case] res: &[LineSignal],
    ) {
        let signals: Vec<LineSignal> = LineState::new(low, high, high_z).iter_possible().collect();
        assert_eq!(signals, res);
    }

    #[rstest]
    #[case(false, false, false, &[])]
    #[case(false, false, true, &[false, true])]
    #[case(true, false, false, &[false])]
    #[case(true, false, true, &[false, true])]
    #[case(false, true, false, &[true])]
    #[case(false, true, true, &[false, true])]
    #[case(true, true, false, &[false, true])]
    #[case(true, true, true, &[false, true])]
    fn possible_reads(
        #[case] low: bool,
        #[case] high: bool,
        #[case] high_z: bool,
        #[case] res: &[bool],
    ) {
        let signals = LineState::new(low, high, high_z).possible_reads();
        assert_eq!(signals, res);
    }
}
