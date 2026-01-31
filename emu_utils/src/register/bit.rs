use crate::register::RegisterError;

pub struct BitRegister {
    name: String,
    state: Option<bool>,
}

impl BitRegister {
    pub fn new(name: String) -> Self {
        Self { name, state: None }
    }

    pub fn read(&self) -> Result<bool, RegisterError> {
        match self.state {
            Some(state) => Ok(state),
            None => Err(RegisterError::RegisterUninitialised {
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
    fn reg() -> BitRegister {
        BitRegister::new(String::new())
    }

    #[rstest]
    fn write_read(mut reg: BitRegister, #[values(false, true)] state: bool) {
        reg.write(state);
        assert_eq!(reg.read().unwrap(), state);
    }

    #[rstest]
    fn read_uninitialised(reg: BitRegister) {
        assert!(reg.read().is_err());
    }
}
