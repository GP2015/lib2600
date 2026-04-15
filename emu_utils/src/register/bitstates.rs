#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PossibleBitStates {
    pub high: bool,
    pub low: bool,
}

impl PossibleBitStates {
    pub fn from(high: bool, low: bool) -> Self {
        Self { high, low }
    }

    pub fn set(&mut self, state: bool, enable: bool) {
        if state {
            self.low = enable;
        } else {
            self.high = enable;
        }
    }

    pub fn set_all(&mut self, enable: bool) {
        self.high = enable;
        self.low = enable;
    }

    pub fn add(&mut self, state: bool) {
        self.set(state, true);
    }

    pub fn add_all(&mut self) {
        self.set_all(true);
    }

    pub fn possible_reads(self) -> Vec<bool> {
        match (self.high, self.low) {
            (false, false) => Vec::new(),
            (false, true) => vec![false],
            (true, false) => vec![true],
            (true, true) => vec![true, false],
        }
    }

    pub fn collapsed(self) -> Option<bool> {
        let vec = self.possible_reads();
        if vec.len() == 1 {
            vec.first().copied()
        } else {
            None
        }
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
        states.set(state, enable);
        let result = if state { states.high } else { states.low };
        assert_eq!(result, enable);
    }

    #[rstest]
    fn set_all(
        #[values(true, false)] high: bool,
        #[values(true, false)] low: bool,
        #[values(true, false)] enable: bool,
    ) {
        let mut states = PossibleBitStates::from(high, low);
        states.set_all(enable);
        assert_eq!(states, PossibleBitStates::from(enable, enable));
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
