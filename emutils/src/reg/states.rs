use delegate::delegate;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PossibleBitStates {
    pub high: bool,
    pub low: bool,
}

impl PossibleBitStates {
    pub fn from(high: bool, low: bool) -> Self {
        Self { high, low }
    }

    pub fn is_possible(self, state: bool) -> bool {
        if state { self.high } else { self.low }
    }

    delegate! {
        to self {
            #[call(is_possible)]
            pub fn high_possible(self, [true]) -> bool;
            #[call(is_possible)]
            pub fn low_possible(self, [false]) -> bool;
        }
    }

    pub fn collapsed(self) -> Option<bool> {
        match (self.high, self.low) {
            (false, true) => Some(false),
            (true, false) => Some(true),
            _ => None,
        }
    }

    pub fn possible_reads(self) -> &'static [bool] {
        match (self.high, self.low) {
            (false, false) => &[],
            (false, true) => &[false],
            (true, false) => &[true],
            (true, true) => &[true, false],
        }
    }

    pub fn add(&mut self, state: bool, only_possible: bool) {
        if state {
            self.high = true;
        } else {
            self.low = true;
        }

        if only_possible {
            self.remove(!state);
        }
    }

    pub fn remove(&mut self, state: bool) {
        if state {
            self.high = false;
        } else {
            self.low = false;
        }
    }

    pub fn set_all(&mut self, high: bool, low: bool) {
        self.high = high;
        self.low = low;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    fn from(#[values(true, false)] high: bool, #[values(true, false)] low: bool) {
        let states = PossibleBitStates::from(high, low);
        assert_eq!(states.high, high);
        assert_eq!(states.low, low);
    }

    #[rstest]
    fn is_possible(#[values(true, false)] high: bool, #[values(true, false)] low: bool) {
        let states = PossibleBitStates::from(high, low);
        assert_eq!(states.is_possible(true), high);
        assert_eq!(states.is_possible(false), low);
    }

    #[rstest]
    #[case(true, false, true)]
    #[case(false, true, false)]
    fn collapsed_success(#[case] high: bool, #[case] low: bool, #[case] state: bool) {
        let states = PossibleBitStates::from(high, low);
        assert_eq!(states.collapsed().unwrap(), state);
    }

    #[rstest]
    #[case(false, false)]
    #[case(true, true)]
    fn collapsed_failure(#[case] high: bool, #[case] low: bool) {
        let states = PossibleBitStates::from(high, low);
        assert!(states.collapsed().is_none());
    }

    #[rstest]
    #[case(false, false, &[])]
    #[case(false, true, &[false])]
    #[case(true, false, &[true])]
    #[case(true, true, &[true, false])]
    fn possible_reads(#[case] high: bool, #[case] low: bool, #[case] res_vec: &[bool]) {
        let states = PossibleBitStates::from(high, low).possible_reads();
        assert_eq!(states, res_vec);
    }

    #[rstest]
    fn add_not_only_possible(
        #[values(true, false)] initial: bool,
        #[values(true, false)] state: bool,
    ) {
        let mut states = PossibleBitStates::from(initial, initial);
        states.add(state, false);
        assert!(states.is_possible(state));
        assert_eq!(states.is_possible(!state), initial);
    }

    #[rstest]
    fn add_only_possible(#[values(true, false)] initial: bool, #[values(true, false)] state: bool) {
        let mut states = PossibleBitStates::from(initial, initial);
        states.add(state, true);
        assert!(states.is_possible(state));
        assert!(!states.is_possible(!state));
    }

    #[rstest]
    fn remove(#[values(true, false)] initial: bool, #[values(true, false)] state: bool) {
        let mut states = PossibleBitStates::from(initial, initial);
        states.remove(state);
        assert!(!states.is_possible(state));
        assert_eq!(states.is_possible(!state), initial);
    }

    #[rstest]
    fn set_all(#[values(true, false)] high: bool, #[values(true, false)] low: bool) {
        let mut states = PossibleBitStates::from(false, false);
        states.set_all(high, low);
        assert_eq!(states, PossibleBitStates::from(high, low));
    }
}
