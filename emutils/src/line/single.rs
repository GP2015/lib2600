use crate::{
    line::{
        LineConnection, LineError, PinSignal,
        state::{now::DriveState, when::DriveStateWhen},
    },
    reg::BitRegister,
};
use delegate::delegate;

pub struct Line {
    name: String,
    connection_states: Vec<DriveStateWhen>,
    combined_state: DriveStateWhen,
}

impl Line {
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self {
            name: name.into(),
            connection_states: Vec::new(),
            combined_state: DriveStateWhen::default(),
        }
    }

    pub fn create_connection(&mut self) -> LineConnection {
        self.connection_states.push(DriveStateWhen::default());
        LineConnection::new(self.connection_states.len() - 1)
    }

    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    fn update_combined_state(&mut self) -> Result<(), LineError> {
        let init = DriveState::from(false, false, true);

        let combined_state_now = self
            .connection_states
            .iter()
            .map(|&state| state.prev_state())
            .try_fold(init, DriveState::contend_with);

        let Some(state) = combined_state_now else {
            return Err(LineError::ShortCircuit {
                name: self.name().to_string(),
            });
        };

        self.combined_state.copy_from_drive_state(state);
        Ok(())
    }

    pub fn copy_from_line(
        &mut self,
        connection: LineConnection,
        line: &Line,
        only_possible: bool,
    ) -> Result<(), LineError> {
        if only_possible {
            self.set_all(
                connection,
                line.high_possible(),
                line.low_possible(),
                line.high_z_possible(),
            )?;
        } else {
            if line.high_possible() {
                self.add_high(connection, false)?;
            }

            if line.low_possible() {
                self.add_low(connection, false)?;
            }

            if line.high_z_possible() {
                self.add_high_z(connection, false);
            }
        }

        self.update_combined_state()
    }

    pub fn copy_from_reg(
        &mut self,
        connection: LineConnection,
        reg: &BitRegister,
        only_possible: bool,
    ) -> Result<(), LineError> {
        if only_possible {
            self.set_all(connection, reg.high_possible(), reg.low_possible(), false)?;
        } else {
            if reg.high_possible() {
                self.add_high(connection, false)?;
            }

            if reg.low_possible() {
                self.add_low(connection, false)?;
            }
        }

        self.update_combined_state()
    }

    delegate! {
        #[must_use]
        to self.combined_state{
            pub fn is_possible(&self, signal: PinSignal) -> bool;
            pub fn is_prev_possible(&self, signal: PinSignal) -> bool;
            pub fn is_possible_when(&self, prev: bool, signal: PinSignal) -> bool;

            pub fn high_possible(&self) -> bool;
            pub fn low_possible(&self) -> bool;
            pub fn high_z_possible(&self) -> bool;
            pub fn prev_high_possible(&self) -> bool;
            pub fn prev_low_possible(&self) -> bool;
            pub fn prev_high_z_possible(&self) -> bool;
            pub fn high_possible_when(&self, prev: bool) -> bool;
            pub fn low_possible_when(&self, prev: bool) -> bool;
            pub fn high_z_possible_when(&self, prev: bool) -> bool;

            pub fn could_read_high(&self) -> bool;
            pub fn could_read_low(&self) -> bool;
            pub fn prev_could_read_high(&self) -> bool;
            pub fn prev_could_read_low(&self) -> bool;
            pub fn could_read_high_when(&self, prev: bool) -> bool;
            pub fn could_read_low_when(&self, prev: bool) -> bool;

            pub fn collapsed(&self) -> Option<PinSignal>;
            pub fn prev_collapsed(&self) -> Option<PinSignal>;
            pub fn collapsed_when(&self, prev: bool) -> Option<PinSignal>;

            pub fn read(&self) -> Option<bool>;
            pub fn read_prev(&self) -> Option<bool>;
            pub fn read_when(&self, prev: bool) -> Option<bool>;

            pub fn possible_reads(&self) -> &'static [bool];
            pub fn prev_possible_reads(&self) -> &'static [bool];
            pub fn possible_reads_when(&self, prev: bool) -> &'static [bool];
        }

        to self.combined_state{
            pub fn iter_possible(&self) -> impl Iterator<Item = PinSignal>;
            pub fn iter_prev_possible(&self) -> impl Iterator<Item = PinSignal>;
            pub fn iter_possible_when(&self, prev: bool) -> impl Iterator<Item = PinSignal>;
        }

        #[expr($; self.update_combined_state())]
        to |connection: LineConnection| self.connection_states[connection.id()]{
            pub fn add(&mut self, signal: PinSignal, only_possible: bool) -> Result<(), LineError>;

            pub fn add_high(&mut self, only_possible: bool) -> Result<(), LineError>;
            pub fn add_low(&mut self, only_possible: bool) -> Result<(), LineError>;

            pub fn add_drive(&mut self, val: bool, only_possible: bool) -> Result<(), LineError>;
            pub fn copy_from_prev(&mut self) -> Result<(), LineError>;
            pub fn set_all(&mut self, high: bool, low: bool, high_z: bool) -> Result<(), LineError>;
        }

        #[expr($self.update_combined_state().expect("valid");)]
        to |connection: LineConnection| self.connection_states[connection.id()]{
            pub fn add_high_z(&mut self, only_possible: bool);

            pub fn remove(&mut self, signal: PinSignal);

            pub fn remove_high(&mut self);
            pub fn remove_low(&mut self);
            pub fn remove_high_z(&mut self);

            pub fn remove_drive(&mut self, val: bool);
        }
    }
}
