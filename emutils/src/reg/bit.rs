use crate::{pin::PinInputter, reg::states::PossibleBitStates};
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
    pub fn high_possible(&self) -> bool {
        self.states.high
    }

    #[must_use]
    pub fn low_possible(&self) -> bool {
        self.states.low
    }

    pub fn input_from_pin<'a, P>(&mut self, pin: &P, only_possible: bool)
    where
        P: PinInputter<'a>,
    {
        if only_possible {
            self.states.high = pin.could_read_high();
            self.states.low = pin.could_read_low();
        } else {
            if pin.could_read_high() {
                self.states.high = true;
            }

            if pin.could_read_low() {
                self.states.low = true;
            }
        }
    }

    delegate! {
        #[must_use]
        to self.states {
            pub fn collapsed(&self) -> Option<bool>;
            pub fn possible_reads(&self) -> Vec<bool>;
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
    use rstest::{fixture, rstest};

    const REG_NAME: &str = "reg";

    #[fixture]
    fn reg() -> BitRegister {
        BitRegister::new(String::from(REG_NAME))
    }

    #[rstest]
    fn name(reg: BitRegister) {
        assert_eq!(reg.name(), REG_NAME);
    }

    #[rstest]
    fn read_initial(reg: BitRegister) {
        assert!(reg.high_possible());
        assert!(reg.low_possible());
    }
}
