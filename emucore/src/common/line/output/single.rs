use crate::common::{bit::single::BitState, combine::Combine, line::signal::LineSignal};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct LineOutput {
    pub low: bool,
    pub high: bool,
    pub high_z: bool,
}

impl LineOutput {
    #[must_use]
    pub const fn new(low: bool, high: bool, high_z: bool) -> Self {
        Self { low, high, high_z }
    }

    #[must_use]
    pub const fn read(self) -> Option<BitState> {
        match (self.low, self.high, self.high_z) {
            (false, false, false) => None,
            (true, false, false) => Some(BitState::Low),
            (false, true, false) => Some(BitState::High),
            _ => Some(BitState::Unknown),
        }
    }

    #[must_use]
    pub fn contend_with(self, other: Self) -> Option<Self> {
        let mut result = Self::new(false, false, false);

        let iter_possible = |state: Self| {
            [
                (state.low, LineSignal::Low),
                (state.high, LineSignal::High),
                (state.high_z, LineSignal::HighZ),
            ]
            .into_iter()
            .filter_map(|(enabled, signal)| enabled.then_some(signal))
        };

        for first_signal in iter_possible(self) {
            for second_signal in iter_possible(other) {
                match first_signal.contend_with(second_signal)? {
                    LineSignal::Low => result.low = true,
                    LineSignal::High => result.high = true,
                    LineSignal::HighZ => result.high_z = true,
                }
            }
        }

        Some(result)
    }
}

impl From<LineSignal> for LineOutput {
    fn from(value: LineSignal) -> Self {
        Self {
            low: matches!(value, LineSignal::Low),
            high: matches!(value, LineSignal::High),
            high_z: matches!(value, LineSignal::HighZ),
        }
    }
}

impl From<BitState> for LineOutput {
    fn from(value: BitState) -> Self {
        Self {
            low: matches!(value, BitState::Low | BitState::Unknown),
            high: matches!(value, BitState::High | BitState::Unknown),
            high_z: false,
        }
    }
}

impl From<bool> for LineOutput {
    fn from(value: bool) -> Self {
        Self {
            low: !value,
            high: value,
            high_z: false,
        }
    }
}

impl Combine for LineOutput {
    fn combine_with(&self, other: &Self) -> Self {
        Self {
            low: self.low || other.low,
            high: self.high || other.high,
            high_z: self.high_z || other.high_z,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use paste::paste;
    use rstest::rstest;

    #[rstest]
    fn new(
        #[values(false, true)] low: bool,
        #[values(false, true)] high: bool,
        #[values(false, true)] high_z: bool,
    ) {
        assert_eq!(
            LineOutput::new(low, high, high_z),
            LineOutput { low, high, high_z }
        );
    }

    #[test]
    fn default() {
        assert_eq!(LineOutput::default(), LineOutput::new(false, false, false));
    }

    #[rstest]
    #[case(true, false, BitState::Low)]
    #[case(false, true, BitState::High)]
    fn read_defined(#[case] low: bool, #[case] high: bool, #[case] output: BitState) {
        assert_eq!(output, LineOutput::new(low, high, false).read().unwrap());
    }

    #[rstest]
    #[case(false, false, true)]
    #[case(false, true, true)]
    #[case(true, false, true)]
    #[case(true, true, false)]
    #[case(true, true, true)]
    fn read_unknown(#[case] low: bool, #[case] high: bool, #[case] high_z: bool) {
        assert_eq!(
            BitState::Unknown,
            LineOutput::new(low, high, high_z).read().unwrap()
        );
    }

    #[test]
    fn read_failure() {
        assert!(LineOutput::default().read().is_none());
    }

    #[rstest]
    #[case(LineSignal::Low, BitState::Low)]
    #[case(LineSignal::High, BitState::High)]
    #[case(LineSignal::HighZ, BitState::Unknown)]
    fn from_signal(#[case] signal: LineSignal, #[case] state: BitState) {
        assert_eq!(state, LineOutput::from(signal).read().unwrap());
    }

    #[rstest]
    #[case(BitState::Low, true, false)]
    #[case(BitState::High, false, true)]
    #[case(BitState::Unknown, true, true)]
    fn from_state(#[case] state: BitState, #[case] low: bool, #[case] high: bool) {
        assert_eq!(LineOutput::from(state), LineOutput::new(low, high, false));
    }

    #[rstest]
    #[case(false, BitState::Low)]
    #[case(true, BitState::High)]
    fn from_bool(#[case] b: bool, #[case] state: BitState) {
        assert_eq!(state, LineOutput::from(b).read().unwrap());
    }

    macro_rules! combine_with {
        ($check:ident, $other0:ident, $other1:ident) => {
            paste! {
                #[rstest]
                #[case(false, false, false)]
                fn [<combine_with_ $check>](
                    #[case] [<a_ $check>]: bool,
                    #[case] [<b_ $check>]: bool,
                    #[case] res: bool,
                    #[values(false, true)] [<a_ $other0>]: bool,
                    #[values(false, true)] [<a_ $other1>]: bool,
                    #[values(false, true)] [<b_ $other0>]: bool,
                    #[values(false, true)] [<b_ $other1>]: bool,
                ) {
                    let a = LineOutput::new(a_low, a_high, a_high_z);
                    let b = LineOutput::new(b_low, b_high, b_high_z);
                    assert_eq!(a.combine_with(&b).$check, res);
                }
            }
        };
    }

    combine_with!(low, high, high_z);
    combine_with!(high, low, high_z);
    combine_with!(high_z, low, high);

    #[rstest]
    fn contend_with_failure(
        #[values(false, true)] state: bool,
        #[values(false, true)] a_high_z: bool,
        #[values(false, true)] b_high_z: bool,
    ) {
        let a = LineOutput::new(state, !state, a_high_z);
        let b = LineOutput::new(!state, state, b_high_z);
        assert!(a.contend_with(b).is_none());
    }

    #[rstest]
    fn contend_with_success(
        #[values(false, true)] main_low: bool,
        #[values(false, true)] main_high: bool,
        #[values(false, true)] main_high_z: bool,
        #[values(false, true)] other_high_z: bool,
    ) {
        let main = LineOutput::new(main_low, main_high, main_high_z);
        let other = LineOutput::new(false, false, other_high_z);

        let mut res = main;
        res.low &= other_high_z;
        res.high &= other_high_z;
        res.high_z &= other_high_z;

        for (a, b) in [(main, other), (other, main)] {
            assert_eq!(a.contend_with(b).unwrap(), res);
        }
    }

    #[rstest]
    fn contend_with_success_2(
        #[values(false, true)] state: bool,
        #[values(false, true)] main_high_z: bool,
        #[values(false, true)] other_high_z: bool,
    ) {
        let main = LineOutput::new(state, !state, main_high_z);
        let other = LineOutput::new(state, !state, other_high_z);

        let mut res = main;
        res.high_z &= other_high_z;

        for (a, b) in [(main, other), (other, main)] {
            assert_eq!(a.contend_with(b).unwrap(), res);
        }
    }
}
