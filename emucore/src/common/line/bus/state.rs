use crate::common::{bit, line::single::state::LineState};
use itertools::Itertools;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct BusState<const SIZE: usize> {
    pub line_states: [LineState; SIZE],
}

impl<const SIZE: usize> BusState<SIZE> {
    #[must_use]
    pub const fn new(line_states: [LineState; SIZE]) -> Self {
        Self { line_states }
    }

    #[must_use]
    pub const fn line_state<const BIT: usize>(&self) -> LineState {
        const { assert!(BIT < SIZE) }
        self.line_states[BIT]
    }

    #[must_use]
    pub fn try_line_state(&self, bit: usize) -> Option<LineState> {
        self.line_states.get(bit).copied()
    }

    pub fn iter(&self) -> impl Iterator<Item = LineState> {
        self.line_states.iter().copied()
    }

    #[must_use]
    pub fn read(&self) -> Option<usize> {
        bit::some_bits_to_usize(self.line_states.iter().copied().map(LineState::read))
    }

    pub fn iter_possible_reads(&self) -> impl Iterator<Item = usize> {
        self.line_states
            .iter()
            .map(|line_state| line_state.possible_reads().iter().copied())
            .multi_cartesian_product()
            .map(|bits| bit::bits_to_usize(bits.into_iter()))
    }
}
