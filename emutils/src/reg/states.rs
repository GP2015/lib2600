#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct PossibleBitStates {
    pub high: bool,
    pub low: bool,
}

impl PossibleBitStates {
    pub const fn new(high: bool, low: bool) -> Self {
        Self { high, low }
    }

    pub const fn high_possible(self) -> bool {
        self.high
    }

    pub const fn low_possible(self) -> bool {
        self.low
    }

    pub const fn is_possible(self, state: bool) -> bool {
        if state { self.high } else { self.low }
    }

    pub const fn collapsed(self) -> Option<bool> {
        match (self.high, self.low) {
            (false, true) => Some(false),
            (true, false) => Some(true),
            _ => None,
        }
    }

    pub const fn possible_reads(self) -> &'static [bool] {
        match (self.high, self.low) {
            (false, false) => &[],
            (false, true) => &[false],
            (true, false) => &[true],
            (true, true) => &[true, false],
        }
    }

    pub const fn add(&mut self, state: bool, only_possible: bool) {
        if state {
            self.high = true;
        } else {
            self.low = true;
        }

        if only_possible {
            self.remove(!state);
        }
    }

    pub const fn remove(&mut self, state: bool) {
        if state {
            self.high = false;
        } else {
            self.low = false;
        }
    }

    pub const fn set_all(&mut self, high: bool, low: bool) {
        self.high = high;
        self.low = low;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    fn new(#[values(true, false)] high: bool, #[values(true, false)] low: bool) {
        let states = PossibleBitStates::new(high, low);
        assert_eq!(states.high, high);
        assert_eq!(states.low, low);
    }

    #[rstest]
    fn is_possible(#[values(true, false)] high: bool, #[values(true, false)] low: bool) {
        let states = PossibleBitStates::new(high, low);
        assert_eq!(states.is_possible(true), high);
        assert_eq!(states.is_possible(false), low);
    }

    #[rstest]
    #[case(true, false, true)]
    #[case(false, true, false)]
    fn collapsed_success(#[case] high: bool, #[case] low: bool, #[case] state: bool) {
        let states = PossibleBitStates::new(high, low);
        assert_eq!(states.collapsed().unwrap(), state);
    }

    #[rstest]
    #[case(false, false)]
    #[case(true, true)]
    fn collapsed_failure(#[case] high: bool, #[case] low: bool) {
        let states = PossibleBitStates::new(high, low);
        assert!(states.collapsed().is_none());
    }

    #[rstest]
    #[case(false, false, &[])]
    #[case(false, true, &[false])]
    #[case(true, false, &[true])]
    #[case(true, true, &[true, false])]
    fn possible_reads(#[case] high: bool, #[case] low: bool, #[case] res: &[bool]) {
        let states = PossibleBitStates::new(high, low).possible_reads();
        assert_eq!(states, res);
    }

    #[rstest]
    fn add_not_only_possible(
        #[values(true, false)] initial: bool,
        #[values(true, false)] state: bool,
    ) {
        let mut states = PossibleBitStates::new(initial, initial);
        states.add(state, false);
        assert!(states.is_possible(state));
        assert_eq!(states.is_possible(!state), initial);
    }

    #[rstest]
    fn add_only_possible(#[values(true, false)] initial: bool, #[values(true, false)] state: bool) {
        let mut states = PossibleBitStates::new(initial, initial);
        states.add(state, true);
        assert!(states.is_possible(state));
        assert!(!states.is_possible(!state));
    }

    #[rstest]
    fn remove(#[values(true, false)] initial: bool, #[values(true, false)] state: bool) {
        let mut states = PossibleBitStates::new(initial, initial);
        states.remove(state);
        assert!(!states.is_possible(state));
        assert_eq!(states.is_possible(!state), initial);
    }

    #[rstest]
    fn set_all(
        #[values(true, false)] initial: bool,
        #[values(true, false)] high: bool,
        #[values(true, false)] low: bool,
    ) {
        let mut states = PossibleBitStates::new(initial, initial);
        states.set_all(high, low);
        assert_eq!(states, PossibleBitStates::new(high, low));
    }
}
