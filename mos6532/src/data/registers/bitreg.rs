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
            None => Err(RiotError::RegisterUninitialised {
                name: self.name.clone(),
            }),
        }
    }

    pub fn write(&mut self, state: bool) {
        self.state = Some(state);
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
    fn write_read(mut reg: BitReg, #[values(false, true)] state: bool) {
        reg.write(state);
        assert_eq!(reg.read().unwrap(), state);
    }

    #[rstest]
    fn read_uninitialised(reg: BitReg) {
        assert!(reg.read().is_err());
    }
}
