use crate::line::PinSignal;
use delegate::delegate;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DriveState {
    pub high: bool,
    pub low: bool,
    pub high_z: bool,
}

impl DriveState {
    pub fn from(high: bool, low: bool, high_z: bool) -> Self {
        Self { high, low, high_z }
    }

    pub fn high_possible(self) -> bool {
        self.high
    }

    pub fn low_possible(self) -> bool {
        self.low
    }

    pub fn high_z_possible(self) -> bool {
        self.high_z
    }

    pub fn is_possible(self, signal: PinSignal) -> bool {
        match signal {
            PinSignal::High => self.high,
            PinSignal::Low => self.low,
            PinSignal::HighZ => self.high_z,
        }
    }

    pub fn could_read_high(self) -> bool {
        self.high | self.high_z
    }

    pub fn could_read_low(self) -> bool {
        self.low | self.high_z
    }

    pub fn collapsed(self) -> Option<PinSignal> {
        match (self.high, self.low, self.high_z) {
            (true, false, false) => Some(PinSignal::High),
            (false, true, false) => Some(PinSignal::Low),
            (false, false, true) => Some(PinSignal::HighZ),
            _ => None,
        }
    }

    pub fn read(self) -> Option<bool> {
        self.collapsed().and_then(|signal| signal.as_bool())
    }

    pub fn iter_possible(self) -> impl Iterator<Item = PinSignal> {
        [
            (self.high, PinSignal::High),
            (self.low, PinSignal::Low),
            (self.high_z, PinSignal::HighZ),
        ]
        .into_iter()
        .filter_map(|(enabled, signal)| enabled.then_some(signal))
    }

    pub fn possible_reads(self) -> &'static [bool] {
        match (self.high, self.low, self.high_z) {
            (false, false, false) => &[],
            (false, true, false) => &[false],
            (true, false, false) => &[true],
            (true, true, false) | (_, _, true) => &[true, false],
        }
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

    pub fn add_drive(&mut self, val: bool, only_possible: bool) {
        if val {
            self.add_high(only_possible);
        } else {
            self.add_low(only_possible);
        }
    }

    pub fn remove_drive(&mut self, val: bool) {
        if val {
            self.remove_high();
        } else {
            self.remove_low();
        }
    }

    pub fn set_all(&mut self, high: bool, low: bool, high_z: bool) {
        self.high = high;
        self.low = low;
        self.high_z = high_z;
    }

    pub fn contend_with(self, other: Self) -> Option<Self> {
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
            pub fn add_high(&mut self, [PinSignal::High], only_possible: bool);
            #[call(add)]
            pub fn add_low(&mut self, [PinSignal::Low], only_possible: bool);
            #[call(add)]
            pub fn add_high_z(&mut self, [PinSignal::HighZ], only_possible: bool);

            #[call(remove)]
            pub fn remove_high(&mut self, [PinSignal::High]);
            #[call(remove)]
            pub fn remove_low(&mut self, [PinSignal::Low]);
            #[call(remove)]
            pub fn remove_high_z(&mut self, [PinSignal::HighZ]);
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
        let signals = DriveState::from(high, low, high_z);
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
        let signals = DriveState::from(high, low, high_z);
        assert_eq!(signals.is_possible(PinSignal::High), high);
        assert_eq!(signals.is_possible(PinSignal::Low), low);
        assert_eq!(signals.is_possible(PinSignal::HighZ), high_z);
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
        let signals = DriveState::from(high, low, high_z);
        assert_eq!(signals.could_read_high(), could_read_high);
        assert_eq!(signals.could_read_low(), could_read_low);
    }

    #[rstest]
    #[case(true, false, false, PinSignal::High)]
    #[case(false, true, false, PinSignal::Low)]
    #[case(false, false, true, PinSignal::HighZ)]
    fn collapsed_success(
        #[case] high: bool,
        #[case] low: bool,
        #[case] high_z: bool,
        #[case] res: PinSignal,
    ) {
        let state = DriveState::from(high, low, high_z);
        assert_eq!(state.collapsed().unwrap(), res);
    }

    #[rstest]
    #[case(false, false, false)]
    #[case(false, true, true)]
    #[case(true, false, true)]
    #[case(true, true, false)]
    #[case(true, true, true)]
    fn collapsed_failure(#[case] high: bool, #[case] low: bool, #[case] high_z: bool) {
        let state = DriveState::from(high, low, high_z);
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
        let signals = DriveState::from(high, low, high_z);
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
        let signals = DriveState::from(high, low, high_z);
        assert!(signals.read().is_none());
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
    fn iter_possible(
        #[case] high: bool,
        #[case] low: bool,
        #[case] high_z: bool,
        #[case] res: &[PinSignal],
    ) {
        let signals = DriveState::from(high, low, high_z)
            .iter_possible()
            .collect::<Vec<PinSignal>>();
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
        let signals = DriveState::from(high, low, high_z).possible_reads();
        assert_eq!(signals, res);
    }

    #[rstest]
    fn add_not_only_possible(
        #[values(true, false)] initial: bool,
        #[values(PinSignal::High, PinSignal::Low, PinSignal::HighZ)] signal: PinSignal,
    ) {
        let mut signals = DriveState::from(initial, initial, initial);
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
        let mut signals = DriveState::from(initial, initial, initial);
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
        let mut signals = DriveState::from(initial, initial, initial);
        signals.remove(signal);
        for s in [PinSignal::High, PinSignal::Low, PinSignal::HighZ] {
            assert_eq!(signals.is_possible(s), signal != s && initial);
        }
    }

    #[rstest]
    fn add_drive(#[values(true, false)] state: bool) {
        let mut signals = DriveState::from(false, false, false);
        signals.add_drive(state, true);
        assert_eq!(signals.is_possible(PinSignal::High), state);
        assert_eq!(signals.is_possible(PinSignal::Low), !state);
        assert!(!signals.is_possible(PinSignal::HighZ));
    }

    #[rstest]
    fn remove_drive(#[values(true, false)] state: bool) {
        let mut signals = DriveState::from(true, true, true);
        signals.remove_drive(state);
        assert_eq!(signals.is_possible(PinSignal::High), !state);
        assert_eq!(signals.is_possible(PinSignal::Low), state);
        assert!(signals.is_possible(PinSignal::HighZ));
    }

    #[rstest]
    fn set_all(
        #[values(true, false)] initial: bool,
        #[values(true, false)] high: bool,
        #[values(true, false)] low: bool,
        #[values(true, false)] high_z: bool,
    ) {
        let mut signals = DriveState::from(initial, initial, initial);
        signals.set_all(high, low, high_z);
        assert_eq!(signals, DriveState::from(high, low, high_z));
    }

    #[rstest]
    fn contend_success_empty(
        #[values(true, false)] high: bool,
        #[values(true, false)] low: bool,
        #[values(true, false)] high_z: bool,
    ) {
        let non_empty = DriveState::from(high, low, high_z);
        let empty = DriveState::from(false, false, false);
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
        let first = DriveState::from(first_high, first_low, first_high_z);
        let second = DriveState::from(false, false, true);
        assert_eq!(first.contend_with(second).unwrap(), first);
    }

    #[rstest]
    fn contend_success_contention(
        #[values(true, false)] bool_state: bool,
        #[values(true, false)] first_high_z: bool,
        #[values(true, false)] second_high_z: bool,
    ) {
        let first = DriveState::from(bool_state, !bool_state, first_high_z);
        let second = DriveState::from(bool_state, !bool_state, second_high_z);
        let res = DriveState::from(bool_state, !bool_state, first_high_z & second_high_z);
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
        let first = DriveState::from(first_high, first_low, first_high_z);
        let second = DriveState::from(second_high, second_low, second_high_z);
        assert!(first.contend_with(second).is_none());
    }
}
