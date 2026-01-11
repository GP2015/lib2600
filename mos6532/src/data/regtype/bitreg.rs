use crate::error::RiotError;

pub struct BitReg {
    name: String,
    state: Option<bool>,
}

impl BitReg {
    pub fn new(name: String) -> Self {
        Self { name, state: None }
    }

    pub fn reset(&mut self) {
        self.state = None;
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_read() {
        let mut reg = BitReg::new(String::new());
        reg.write(true);
        assert!(reg.read().unwrap());
        reg.write(false);
        assert!(!reg.read().unwrap());
    }

    #[test]
    fn read_uninitialised() {
        let reg = BitReg::new(String::new());
        assert!(reg.read().is_err());
    }

    #[test]
    fn is_writen() {
        let mut reg = BitReg::new(String::new());
        assert!(!reg.is_written());
        reg.write(true);
        assert!(reg.is_written());
    }
}
