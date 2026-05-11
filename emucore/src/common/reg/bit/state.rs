#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct BitRegState {
    pub low: bool,
    pub high: bool,
}

impl BitRegState {
    #[must_use]
    pub const fn new(low: bool, high: bool) -> Self {
        Self { low, high }
    }

    #[must_use]
    pub const fn low_high_possible(self) -> (bool, bool) {
        (self.low, self.high)
    }

    #[must_use]
    pub const fn is_possible(self, state: bool) -> bool {
        if state { self.high } else { self.low }
    }

    #[must_use]
    pub const fn is_defined(self) -> bool {
        self.low ^ self.high
    }

    #[must_use]
    pub const fn collapsed(self) -> Option<bool> {
        match (self.low, self.high) {
            (true, false) => Some(false),
            (false, true) => Some(true),
            _ => None,
        }
    }

    #[must_use]
    pub const fn possible_reads(self) -> &'static [bool] {
        match (self.low, self.high) {
            (false, false) => &[],
            (true, false) => &[false],
            (false, true) => &[true],
            (true, true) => &[false, true],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    fn new(#[values(true, false)] low: bool, #[values(true, false)] high: bool) {
        let states = BitRegState::new(low, high);
        assert_eq!(states.low, low);
        assert_eq!(states.high, high);
    }

    #[rstest]
    fn is_possible(#[values(true, false)] low: bool, #[values(true, false)] high: bool) {
        let states = BitRegState::new(low, high);
        assert_eq!(states.is_possible(false), low);
        assert_eq!(states.is_possible(true), high);
    }

    #[rstest]
    #[case(false, true, true)]
    #[case(true, false, false)]
    fn collapsed_success(#[case] low: bool, #[case] high: bool, #[case] state: bool) {
        let states = BitRegState::new(low, high);
        assert_eq!(states.collapsed().unwrap(), state);
    }

    #[rstest]
    #[case(false, false)]
    #[case(true, true)]
    fn collapsed_failure(#[case] low: bool, #[case] high: bool) {
        let states = BitRegState::new(low, high);
        assert!(states.collapsed().is_none());
    }

    #[rstest]
    #[case(false, false, &[])]
    #[case(true, false, &[false])]
    #[case(false, true, &[true])]
    #[case(true, true, &[false, true])]
    fn possible_reads(#[case] low: bool, #[case] high: bool, #[case] res: &[bool]) {
        let states = BitRegState::new(low, high).possible_reads();
        assert_eq!(states, res);
    }
}
