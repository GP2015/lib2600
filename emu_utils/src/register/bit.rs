use crate::register::RegError;

pub struct BitReg {
    name: String,
    state: Option<bool>,
}

impl BitReg {
    pub fn new(name: String) -> Self {
        Self { name, state: None }
    }

    pub fn read(&self) -> Result<bool, RegError> {
        match self.state {
            Some(state) => Ok(state),
            None => Err(RegError::RegisterUninitialised {
                name: self.name.clone(),
            }),
        }
    }

    pub fn write(&mut self, state: bool) {
        self.state = Some(state);
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
