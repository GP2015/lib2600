use crate::register::RegisterError;

#[derive(Clone)]
pub struct BitRegister {
    name: String,
    state: Option<bool>,
}

impl BitRegister {
    pub fn new(name: String) -> Self {
        Self { name, state: None }
    }

    pub fn state(&self) -> Option<bool> {
        self.state
    }

    pub fn read(&self) -> Result<bool, RegisterError> {
        match self.state {
            Some(state) => Ok(state),
            None => Err(RegisterError::ReadUndefined {
                name: self.name.clone(),
            }),
        }
    }

    pub fn write(&mut self, state: bool) {
        self.state = Some(state);
    }

    pub fn undefine(&mut self) {
        self.state = None;
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
    fn read_initial(reg: BitRegister) {
        assert!(matches!(
            reg.read().err().unwrap(),
            RegisterError::ReadUndefined { .. }
        ))
    }

    #[rstest]
    fn undefine(mut reg: BitRegister) {
        reg.write(true);
        reg.undefine();

        assert!(matches!(
            reg.read().err().unwrap(),
            RegisterError::ReadUndefined { .. }
        ))
    }
}
