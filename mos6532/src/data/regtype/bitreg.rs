use crate::error::RiotError;

pub struct BitReg {
    name: String,
    state: Option<bool>,
}

impl BitReg {
    pub fn new(name: String) -> Self {
        Self { name, state: None }
    }

    pub fn read(&self) -> Result<bool, RiotError> {
        match self.state {
            Some(state) => Ok(state),
            None => Err(RiotError::UninitialisedBitReg {
                reg_name: self.name.clone(),
            }),
        }
    }

    pub fn write(&mut self, state: bool) {
        self.state = Some(state);
    }

    pub fn is_written(&self) -> bool {
        self.state.is_some()
    }

    pub fn reset(&mut self) {
        self.state = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::{fixture, rstest};

    #[fixture]
    fn reg() -> BitReg {
        BitReg::new(String::new())
    }

    #[rstest]
    fn write_read(mut reg: BitReg) {
        reg.write(true);
        assert!(reg.read().unwrap());
        reg.write(false);
        assert!(!reg.read().unwrap());
    }

    #[rstest]
    fn read_uninitialised(reg: BitReg) {
        assert!(reg.read().is_err());
    }

    #[rstest]
    fn is_written(mut reg: BitReg) {
        assert!(!reg.is_written());
        reg.write(true);
        assert!(reg.is_written());
    }

    #[rstest]
    fn reset(mut reg: BitReg) {
        reg.write(true);
        reg.reset();
        assert!(reg.read().is_err());
        assert!(!reg.is_written());
    }
}
