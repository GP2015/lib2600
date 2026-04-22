use crate::{pin::PinInputUI, reg::states::PossibleBitStates};
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

    pub fn input_from_pin(&mut self, pin: &impl PinInputUI, only_possible: bool) {
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
    use crate::pin::{PinCore, PinOutput, single::mock::MockPin};
    use rstest::{fixture, rstest};

    const REG_NAME: &str = "reg";

    #[fixture]
    fn single_pin() -> MockPin {
        let mut single_pin = MockPin::new("pin");
        single_pin.set_all_in(false, false, false).unwrap();
        single_pin.set_all_out(false, false, false).unwrap();
        single_pin
    }

    #[fixture]
    fn reg() -> BitRegister {
        BitRegister::new(REG_NAME)
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

    #[rstest]
    fn input_from_pin_not_only_possible(
        #[values(true, false)] initial: bool,
        #[values(true, false)] high: bool,
        #[values(true, false)] low: bool,
        mut reg: BitRegister,
        mut single_pin: MockPin,
    ) {
        reg.set_all(initial, initial);
        single_pin.set_all_in(high, low, false).unwrap();
        reg.input_from_pin(&single_pin, false);
        assert_eq!(reg.high_possible(), high | initial);
        assert_eq!(reg.low_possible(), low | initial);
    }

    #[rstest]
    fn input_from_pin_only_possible(
        #[values(true, false)] initial: bool,
        #[values(true, false)] high: bool,
        #[values(true, false)] low: bool,
        mut reg: BitRegister,
        mut single_pin: MockPin,
    ) {
        reg.set_all(initial, initial);
        single_pin.set_all_in(high, low, false).unwrap();
        reg.input_from_pin(&single_pin, true);
        assert_eq!(reg.high_possible(), high);
        assert_eq!(reg.low_possible(), low);
    }
}
