use crate::line::LineSignal;
use delegate::delegate;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LineState {
    pub high: bool,
    pub low: bool,
    pub high_z: bool,
}

impl LineState {
    #[must_use]
    pub fn from(high: bool, low: bool, high_z: bool) -> Self {
        Self { high, low, high_z }
    }

    #[must_use]
    pub fn high_possible(self) -> bool {
        self.high
    }

    #[must_use]
    pub fn low_possible(self) -> bool {
        self.low
    }

    #[must_use]
    pub fn high_z_possible(self) -> bool {
        self.high_z
    }

    #[must_use]
    pub fn is_possible(self, signal: LineSignal) -> bool {
        match signal {
            LineSignal::High => self.high,
            LineSignal::Low => self.low,
            LineSignal::HighZ => self.high_z,
        }
    }

    #[must_use]
    pub fn could_read_high(self) -> bool {
        self.high | self.high_z
    }

    #[must_use]
    pub fn could_read_low(self) -> bool {
        self.low | self.high_z
    }

    #[must_use]
    pub fn collapsed(self) -> Option<LineSignal> {
        match (self.high, self.low, self.high_z) {
            (true, false, false) => Some(LineSignal::High),
            (false, true, false) => Some(LineSignal::Low),
            (false, false, true) => Some(LineSignal::HighZ),
            _ => None,
        }
    }

    #[must_use]
    pub fn read(self) -> Option<bool> {
        self.collapsed().and_then(|signal| signal.as_bool())
    }

    pub fn iter_possible(self) -> impl Iterator<Item = LineSignal> {
        [
            (self.high, LineSignal::High),
            (self.low, LineSignal::Low),
            (self.high_z, LineSignal::HighZ),
        ]
        .into_iter()
        .filter_map(|(enabled, signal)| enabled.then_some(signal))
    }

    #[must_use]
    pub fn possible_reads(self) -> &'static [bool] {
        match (self.high, self.low, self.high_z) {
            (false, false, false) => &[],
            (false, true, false) => &[false],
            (true, false, false) => &[true],
            (true, true, false) | (_, _, true) => &[true, false],
        }
    }

    pub(crate) fn add(&mut self, signal: LineSignal, only_possible: bool) {
        if only_possible {
            self.set_all(false, false, false);
        }

        match signal {
            LineSignal::High => self.high = true,
            LineSignal::Low => self.low = true,
            LineSignal::HighZ => self.high_z = true,
        }
    }

    pub(crate) fn remove(&mut self, signal: LineSignal) {
        match signal {
            LineSignal::High => self.high = false,
            LineSignal::Low => self.low = false,
            LineSignal::HighZ => self.high_z = false,
        }
    }

    pub(crate) fn add_drive(&mut self, val: bool, only_possible: bool) {
        if val {
            self.add_high(only_possible);
        } else {
            self.add_low(only_possible);
        }
    }

    pub(crate) fn remove_drive(&mut self, val: bool) {
        if val {
            self.remove_high();
        } else {
            self.remove_low();
        }
    }

    pub(crate) fn set_all(&mut self, high: bool, low: bool, high_z: bool) {
        self.high = high;
        self.low = low;
        self.high_z = high_z;
    }

    #[must_use]
    pub(crate) fn contend_with(self, other: Self) -> Option<Self> {
        let mut result = Self::from(false, false, false);

        for first_signal in self.iter_possible() {
            for second_signal in other.iter_possible() {
                let signal = first_signal.contend_with(second_signal)?;
                result.add(signal, false);
            }
        }

        Some(result)
    }

    delegate! {
        to self{
            #[call(add)]
            pub(crate) fn add_high(&mut self, [LineSignal::High], only_possible: bool);
            #[call(add)]
            pub(crate) fn add_low(&mut self, [LineSignal::Low], only_possible: bool);
            #[call(add)]
            pub(crate) fn add_high_z(&mut self, [LineSignal::HighZ], only_possible: bool);

            #[call(remove)]
            pub(crate) fn remove_high(&mut self, [LineSignal::High]);
            #[call(remove)]
            pub(crate) fn remove_low(&mut self, [LineSignal::Low]);
            #[call(remove)]
            pub(crate) fn remove_high_z(&mut self, [LineSignal::HighZ]);
        }
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
        let signals = LineState::from(high, low, high_z);
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
        let signals = LineState::from(high, low, high_z);
        assert_eq!(signals.is_possible(LineSignal::High), high);
        assert_eq!(signals.is_possible(LineSignal::Low), low);
        assert_eq!(signals.is_possible(LineSignal::HighZ), high_z);
    }

    #[rstest]
    #[case(false, false, false, false, false)]
    #[case(false, false, true, true, true)]
    #[case(false, true, false, false, true)]
    #[case(false, true, true, true, true)]
    #[case(true, false, false, true, false)]
    #[case(true, false, true, true, true)]
    #[case(true, true, false, true, true)]
    #[case(true, true, true, true, true)]
    fn could_read(
        #[case] high: bool,
        #[case] low: bool,
        #[case] high_z: bool,
        #[case] could_read_high: bool,
        #[case] could_read_low: bool,
    ) {
        let signals = LineState::from(high, low, high_z);
        assert_eq!(signals.could_read_high(), could_read_high);
        assert_eq!(signals.could_read_low(), could_read_low);
    }

    #[rstest]
    #[case(true, false, false, LineSignal::High)]
    #[case(false, true, false, LineSignal::Low)]
    #[case(false, false, true, LineSignal::HighZ)]
    fn collapsed_success(
        #[case] high: bool,
        #[case] low: bool,
        #[case] high_z: bool,
        #[case] res: LineSignal,
    ) {
        let state = LineState::from(high, low, high_z);
        assert_eq!(state.collapsed().unwrap(), res);
    }

    #[rstest]
    #[case(false, false, false)]
    #[case(false, true, true)]
    #[case(true, false, true)]
    #[case(true, true, false)]
    #[case(true, true, true)]
    fn collapsed_failure(#[case] high: bool, #[case] low: bool, #[case] high_z: bool) {
        let state = LineState::from(high, low, high_z);
        assert!(state.collapsed().is_none());
    }

    #[rstest]
    #[case(true, false, false, true)]
    #[case(false, true, false, false)]
    fn read_success(
        #[case] high: bool,
        #[case] low: bool,
        #[case] high_z: bool,
        #[case] res: bool,
    ) {
        let signals = LineState::from(high, low, high_z);
        assert_eq!(signals.read().unwrap(), res);
    }

    #[rstest]
    #[case(false, false, false)]
    #[case(false, false, true)]
    #[case(false, true, true)]
    #[case(true, false, true)]
    #[case(true, true, false)]
    #[case(true, true, true)]
    fn read_failure(#[case] high: bool, #[case] low: bool, #[case] high_z: bool) {
        let signals = LineState::from(high, low, high_z);
        assert!(signals.read().is_none());
    }

    #[rstest]
    #[case(false, false, false, &[])]
    #[case(false, false, true, &[LineSignal::HighZ])]
    #[case(false, true, false, &[LineSignal::Low])]
    #[case(false, true, true, &[LineSignal::Low, LineSignal::HighZ])]
    #[case(true, false, false, &[LineSignal::High])]
    #[case(true, false, true, &[LineSignal::High, LineSignal::HighZ])]
    #[case(true, true, false, &[LineSignal::High, LineSignal::Low])]
    #[case(true, true, true, &[LineSignal::High, LineSignal::Low, LineSignal::HighZ])]
    fn iter_possible(
        #[case] high: bool,
        #[case] low: bool,
        #[case] high_z: bool,
        #[case] res: &[LineSignal],
    ) {
        let signals: Vec<LineSignal> = LineState::from(high, low, high_z).iter_possible().collect();
        assert_eq!(signals, res);
    }

    #[rstest]
    #[case(false, false, false, &[])]
    #[case(false, false, true, &[true, false])]
    #[case(false, true, false, &[false])]
    #[case(false, true, true, &[true, false])]
    #[case(true, false, false, &[true])]
    #[case(true, false, true, &[true, false])]
    #[case(true, true, false, &[true, false])]
    #[case(true, true, true, &[true, false])]
    fn possible_reads(
        #[case] high: bool,
        #[case] low: bool,
        #[case] high_z: bool,
        #[case] res: &[bool],
    ) {
        let signals = LineState::from(high, low, high_z).possible_reads();
        assert_eq!(signals, res);
    }

    #[rstest]
    fn add_not_only_possible(
        #[values(true, false)] initial: bool,
        #[values(LineSignal::High, LineSignal::Low, LineSignal::HighZ)] signal: LineSignal,
    ) {
        let mut signals = LineState::from(initial, initial, initial);
        signals.add(signal, false);
        for s in [LineSignal::High, LineSignal::Low, LineSignal::HighZ] {
            assert_eq!(signals.is_possible(s), signal == s || initial);
        }
    }

    #[rstest]
    fn add_only_possible(
        #[values(true, false)] initial: bool,
        #[values(LineSignal::High, LineSignal::Low, LineSignal::HighZ)] signal: LineSignal,
    ) {
        let mut signals = LineState::from(initial, initial, initial);
        signals.add(signal, true);
        for s in [LineSignal::High, LineSignal::Low, LineSignal::HighZ] {
            assert_eq!(signals.is_possible(s), signal == s);
        }
    }

    #[rstest]
    fn remove(
        #[values(true, false)] initial: bool,
        #[values(LineSignal::High, LineSignal::Low, LineSignal::HighZ)] signal: LineSignal,
    ) {
        let mut signals = LineState::from(initial, initial, initial);
        signals.remove(signal);
        for s in [LineSignal::High, LineSignal::Low, LineSignal::HighZ] {
            assert_eq!(signals.is_possible(s), signal != s && initial);
        }
    }

    #[rstest]
    fn add_drive(#[values(true, false)] state: bool) {
        let mut signals = LineState::from(false, false, false);
        signals.add_drive(state, true);
        assert_eq!(signals.is_possible(LineSignal::High), state);
        assert_eq!(signals.is_possible(LineSignal::Low), !state);
        assert!(!signals.is_possible(LineSignal::HighZ));
    }

    #[rstest]
    fn remove_drive(#[values(true, false)] state: bool) {
        let mut signals = LineState::from(true, true, true);
        signals.remove_drive(state);
        assert_eq!(signals.is_possible(LineSignal::High), !state);
        assert_eq!(signals.is_possible(LineSignal::Low), state);
        assert!(signals.is_possible(LineSignal::HighZ));
    }

    #[rstest]
    fn set_all(
        #[values(true, false)] initial: bool,
        #[values(true, false)] high: bool,
        #[values(true, false)] low: bool,
        #[values(true, false)] high_z: bool,
    ) {
        let mut signals = LineState::from(initial, initial, initial);
        signals.set_all(high, low, high_z);
        assert_eq!(signals, LineState::from(high, low, high_z));
    }

    #[rstest]
    fn contend_success_empty(
        #[values(true, false)] high: bool,
        #[values(true, false)] low: bool,
        #[values(true, false)] high_z: bool,
    ) {
        let non_empty = LineState::from(high, low, high_z);
        let empty = LineState::from(false, false, false);
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
        let first = LineState::from(first_high, first_low, first_high_z);
        let second = LineState::from(false, false, true);
        assert_eq!(first.contend_with(second).unwrap(), first);
    }

    #[rstest]
    fn contend_success_contention(
        #[values(true, false)] bool_state: bool,
        #[values(true, false)] first_high_z: bool,
        #[values(true, false)] second_high_z: bool,
    ) {
        let first = LineState::from(bool_state, !bool_state, first_high_z);
        let second = LineState::from(bool_state, !bool_state, second_high_z);
        let res = LineState::from(bool_state, !bool_state, first_high_z & second_high_z);
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
        let first = LineState::from(first_high, first_low, first_high_z);
        let second = LineState::from(second_high, second_low, second_high_z);
        assert!(first.contend_with(second).is_none());
    }
}
