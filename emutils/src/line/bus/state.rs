use crate::{bit, line::LineState};
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
    pub fn line_state(&self, bit: usize) -> Option<LineState> {
        self.line_states.get(bit).copied()
    }

    pub fn iter(&self) -> impl Iterator<Item = LineState> {
        self.line_states.iter().copied()
    }

    #[must_use]
    pub fn is_defined(&self) -> bool {
        self.iter().all(LineState::is_defined)
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
