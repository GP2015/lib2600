use crate::register::states::PossibleBitStates;
use delegate::delegate;

#[derive(Clone)]
pub struct BitRegister {
    name: String,
    states: PossibleBitStates,
}

impl BitRegister {
    #[must_use]
    pub fn new(name: String) -> Self {
        Self {
            name,
            states: PossibleBitStates::from(true, true),
        }
    }

    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[must_use]
    pub fn could_read_high(&self) -> bool {
        self.states.high
    }

    #[must_use]
    pub fn could_read_low(&self) -> bool {
        self.states.low
    }

    delegate! {
        #[must_use]
        to self.states {
            pub fn collapsed(&self) -> Option<bool>;
            pub fn possible_reads(&self) -> Vec<bool>;
        }

        to self.states {
            pub fn set(&mut self, signal: bool, possible: bool);
            pub fn set_all(&mut self, possible: bool);
            pub fn add(&mut self, signal: bool);
            pub fn add_all(&mut self);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::{fixture, rstest};

    const REG_NAME: &str = "reg";

    #[fixture]
    fn reg() -> BitRegister {
        BitRegister::new(String::new())
    }

    #[rstest]
    fn name(reg: BitRegister) {
        assert_eq!(reg.name(), REG_NAME);
    }

    #[rstest]
    fn read_initial(reg: BitRegister) {
        assert!(!reg.could_read_high());
        assert!(!reg.could_read_low());
    }
}
