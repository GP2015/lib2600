pub mod state;

use crate::{
    line::{LineError, LineSignal, LineState},
    reg::BitRegister,
};
use delegate::delegate;

#[derive(Debug)]
pub struct Line {
    name: String,
    connection_states: Vec<LineState>,
    combined_state: LineState,
}

impl Line {
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self {
            name: name.into(),
            connection_states: Vec::new(),
            combined_state: LineState::new(false, false, false),
        }
    }

    pub fn create_connection(&mut self) -> usize {
        let state = LineState::new(false, false, false);
        self.connection_states.push(state);
        self.connection_states.len() - 1
    }

    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[must_use]
    pub const fn state(&self) -> LineState {
        self.combined_state
    }

    fn update_combined_state(&mut self) -> Result<(), LineError> {
        let init = LineState::new(false, false, true);
        self.combined_state = self
            .connection_states
            .iter()
            .copied()
            .try_fold(init, LineState::contend_with)
            .ok_or_else(|| LineError::ShortCircuit {
                name: self.name().to_string(),
            })?;
        Ok(())
    }

    pub fn copy_from_line(
        &mut self,
        connection: usize,
        line: &Self,
        only_possible: bool,
    ) -> Result<(), LineError> {
        let state = &mut self.connection_states[connection];

        if only_possible {
            state.set_all(
                line.high_possible(),
                line.low_possible(),
                line.high_z_possible(),
            );
        } else {
            if line.high_possible() {
                state.high = true;
            }

            if line.low_possible() {
                state.low = true;
            }

            if line.high_z_possible() {
                state.high_z = true;
            }
        }

        self.update_combined_state()
    }

    pub fn copy_from_reg(
        &mut self,
        connection: usize,
        reg: &BitRegister,
        only_possible: bool,
    ) -> Result<(), LineError> {
        let state = &mut self.connection_states[connection];

        if only_possible {
            state.set_all(reg.high_possible(), reg.low_possible(), false);
        } else {
            if reg.high_possible() {
                state.high = true;
            }
            if reg.low_possible() {
                state.low = true;
            }
        }

        self.update_combined_state()
    }

    delegate! {
        #[must_use]
        to self.combined_state{
            pub const fn is_possible(&self, signal: LineSignal) -> bool;

            pub const fn high_possible(&self) -> bool;
            pub const fn low_possible(&self) -> bool;
            pub const fn high_z_possible(&self) -> bool;

            pub const fn could_read_high(&self) -> bool;
            pub const fn could_read_low(&self) -> bool;

            pub const fn collapsed(&self) -> Option<LineSignal>;
            pub fn read(&self) -> Option<bool>;
            pub const fn possible_reads(&self) -> &'static [bool];
        }

        to self.combined_state{
            pub fn iter_possible(&self) -> impl Iterator<Item = LineSignal>;
        }

        #[expr($; self.update_combined_state())]
        to |connection: usize| self.connection_states[connection]{
            pub fn add(&mut self, signal: LineSignal, only_possible: bool) -> Result<(), LineError>;

            pub fn add_high(&mut self, only_possible: bool) -> Result<(), LineError>;
            pub fn add_low(&mut self, only_possible: bool) -> Result<(), LineError>;

            pub fn add_drive(&mut self, val: bool, only_possible: bool) -> Result<(), LineError>;
            pub fn set_all(&mut self, high: bool, low: bool, high_z: bool) -> Result<(), LineError>;
        }

        #[expr($self.update_combined_state().expect("valid");)]
        to |connection: usize| self.connection_states[connection]{
            pub fn add_high_z(&mut self, only_possible: bool);

            pub fn remove(&mut self, signal: LineSignal);

            pub fn remove_high(&mut self);
            pub fn remove_low(&mut self);
            pub fn remove_high_z(&mut self);

            pub fn remove_drive(&mut self, val: bool);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::{fixture, rstest};

    const LINE_NAME: &str = "line";

    #[fixture]
    fn line() -> Line {
        Line::new(LINE_NAME)
    }

    #[rstest]
    fn new(line: Line) {
        assert_eq!(line.name(), LINE_NAME);
    }

    #[rstest]
    fn single_connection(mut line: Line) {
        let connection = line.create_connection();
        line.add_high(connection, true).unwrap();
        assert!(line.read().unwrap());
        line.add_low(connection, true).unwrap();
        assert!(!line.read().unwrap());
    }

    #[rstest]
    fn double_connection(mut line: Line) {
        let connection1 = line.create_connection();
        let connection2 = line.create_connection();
        line.add_high(connection1, true).unwrap();
        line.add_high_z(connection2, true);
        assert!(line.could_read_high());
        assert!(!line.could_read_low());
        line.add_high(connection2, false).unwrap();
        assert!(line.could_read_high());
        assert!(!line.could_read_low());
        line.add_high_z(connection1, true);
        line.remove_high(connection2);
        assert!(line.could_read_high());
        assert!(line.could_read_low());
        line.add_low(connection1, true).unwrap();
        assert!(!line.could_read_high());
        assert!(line.could_read_low());
    }
}
