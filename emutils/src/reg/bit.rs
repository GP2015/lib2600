use crate::{line::Line, reg::states::PossibleBitStates};
use delegate::delegate;

#[derive(Clone)]
pub struct BitRegister {
    name: String,
    states: PossibleBitStates,
}

impl BitRegister {
    #[must_use]
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self {
            name: name.into(),
            states: PossibleBitStates::from(true, true),
        }
    }

    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn copy_from_line(&mut self, line: &Line, only_possible: bool) {
        if only_possible {
            self.states.high = line.could_read_high();
            self.states.low = line.could_read_low();
        } else {
            if line.could_read_high() {
                self.states.high = true;
            }

            if line.could_read_low() {
                self.states.low = true;
            }
        }
    }

    delegate! {
        #[must_use]
        to self.states {
            pub fn is_possible(&self, state: bool) -> bool;
            pub fn high_possible(&self) -> bool;
            pub fn low_possible(&self) -> bool;
            pub fn collapsed(&self) -> Option<bool>;
            pub fn possible_reads(&self) -> &'static [bool];
        }

        to self.states {
            pub fn add(&mut self, state: bool, only_possible: bool);
            pub fn remove(&mut self, state: bool);
            pub fn set_all(&mut self, high: bool, low: bool);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::line::LineConnection;
    use rstest::{fixture, rstest};

    const REG_NAME: &str = "reg";

    #[fixture]
    fn line_and_connection() -> (Line, LineConnection) {
        let mut line = Line::new("");
        let connection = line.create_connection();
        (line, connection)
    }

    #[fixture]
    fn reg() -> BitRegister {
        BitRegister::new(REG_NAME)
    }

    #[rstest]
    fn initial(reg: BitRegister) {
        assert!(reg.high_possible());
        assert!(reg.low_possible());
    }

    #[rstest]
    fn name(reg: BitRegister) {
        assert_eq!(reg.name(), REG_NAME);
    }

    #[rstest]
    fn copy_from_line_only_possible(
        #[values(true, false)] initial: bool,
        #[values(true, false)] high: bool,
        #[values(true, false)] low: bool,
        mut reg: BitRegister,
        #[from(line_and_connection)] (mut line, connection): (Line, LineConnection),
    ) {
        reg.set_all(initial, initial);
        line.set_all(&connection, high, low, false).unwrap();
        reg.copy_from_line(&line, true);
        assert_eq!(reg.high_possible(), high);
        assert_eq!(reg.low_possible(), low);
    }

    #[rstest]
    fn copy_from_line_not_only_possible(
        #[values(true, false)] initial: bool,
        #[values(true, false)] high: bool,
        #[values(true, false)] low: bool,
        mut reg: BitRegister,
        #[from(line_and_connection)] (mut line, connection): (Line, LineConnection),
    ) {
        reg.set_all(initial, initial);
        line.set_all(&connection, high, low, false).unwrap();
        reg.copy_from_line(&line, false);
        assert_eq!(reg.high_possible(), high | initial);
        assert_eq!(reg.low_possible(), low | initial);
    }
}
