#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PossibleBitStates {
    pub high: bool,
    pub low: bool,
}

impl PossibleBitStates {
    pub fn from(high: bool, low: bool) -> Self {
        Self { high, low }
    }

    pub fn collapsed(self) -> Option<bool> {
        match (self.high, self.low) {
            (false, true) => Some(false),
            (true, false) => Some(true),
            _ => None,
        }
    }

    pub fn possible_reads(self) -> Vec<bool> {
        match (self.high, self.low) {
            (false, false) => Vec::new(),
            (false, true) => vec![false],
            (true, false) => vec![true],
            (true, true) => vec![true, false],
        }
    }

    pub fn add(&mut self, state: bool, only_possible: bool) {
        match (only_possible, state) {
            (false, false) => self.low = true,
            (false, true) => self.high = true,
            (true, false) => {
                self.high = false;
                self.low = true;
            }
            (true, true) => {
                self.high = true;
                self.low = false;
            }
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
    fn set(
        #[values(true, false)] initial: bool,
        #[values(true, false)] enable: bool,
        #[values(true, false)] state: bool,
    ) {
        let mut states = PossibleBitStates::from(initial, initial);
        states.add(state, enable);
        let result = if state { states.high } else { states.low };
        assert_eq!(result, enable);
    }

    #[rstest]
    fn set_all(#[values(true, false)] high: bool, #[values(true, false)] low: bool) {
        let mut states = PossibleBitStates::from(false, false);
        states.set_all(high, low);
        assert_eq!(states, PossibleBitStates::from(high, low));
    }

    #[rstest]
    #[case(false, false, Vec::new())]
    #[case(false, true, vec![false])]
    #[case(true, false, vec![true])]
    #[case(true, true, vec![true, false])]
    fn possible_reads(#[case] high: bool, #[case] low: bool, #[case] res_vec: Vec<bool>) {
        let states = PossibleBitStates::from(high, low).possible_reads();
        assert_eq!(states, res_vec);
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
}
